use std::{
    collections::{HashMap, VecDeque},
    future::{ready, Future, Ready},
    pin::Pin,
    rc::Rc,
    sync::{Arc, Mutex},
    task::{Context, Poll},
    time::{Duration, Instant},
};

use actix_web::{
    body::{EitherBody, MessageBody},
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};

#[derive(Debug, Clone)]
pub struct RateLimit {
    enabled: bool,
    requests_per_minute: u32,
    store: Arc<Mutex<HashMap<String, VecDeque<Instant>>>>,
}

impl RateLimit {
    pub fn new(enabled: bool, requests_per_minute: u32) -> Self {
        Self {
            enabled,
            requests_per_minute,
            store: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for RateLimit
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = RateLimitMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RateLimitMiddleware {
            service: Rc::new(service),
            enabled: self.enabled,
            requests_per_minute: self.requests_per_minute,
            store: self.store.clone(),
        }))
    }
}

pub struct RateLimitMiddleware<S> {
    service: Rc<S>,
    enabled: bool,
    requests_per_minute: u32,
    store: Arc<Mutex<HashMap<String, VecDeque<Instant>>>>,
}

impl<S, B> Service<ServiceRequest> for RateLimitMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, context: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(context)
    }

    fn call(&self, request: ServiceRequest) -> Self::Future {
        if self.enabled && is_limited(&request, self.requests_per_minute, &self.store) {
            let response = HttpResponse::TooManyRequests()
                .finish()
                .map_into_right_body();
            return Box::pin(async move { Ok(request.into_response(response)) });
        }

        let service = self.service.clone();
        Box::pin(async move {
            service
                .call(request)
                .await
                .map(ServiceResponse::map_into_left_body)
        })
    }
}

fn is_limited(
    request: &ServiceRequest,
    requests_per_minute: u32,
    store: &Arc<Mutex<HashMap<String, VecDeque<Instant>>>>,
) -> bool {
    if requests_per_minute == 0 {
        return true;
    }

    let key = request
        .connection_info()
        .realip_remote_addr()
        .map(str::to_owned)
        .unwrap_or_else(|| "unknown".to_owned());
    let now = Instant::now();
    let window = Duration::from_secs(60);
    let mut store = store.lock().expect("rate limit store mutex poisoned");
    let bucket = store.entry(key).or_default();

    while bucket
        .front()
        .is_some_and(|timestamp| now.duration_since(*timestamp) > window)
    {
        bucket.pop_front();
    }

    if bucket.len() >= requests_per_minute as usize {
        return true;
    }

    bucket.push_back(now);
    false
}
