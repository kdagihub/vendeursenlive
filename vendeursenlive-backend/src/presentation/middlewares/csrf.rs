use std::{
    future::{ready, Future, Ready},
    pin::Pin,
    rc::Rc,
    task::{Context, Poll},
};

use actix_web::{
    body::{EitherBody, MessageBody},
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    http::Method,
    Error, HttpResponse,
};

use crate::presentation::http::auth::CSRF_TOKEN_COOKIE;

pub const CSRF_HEADER: &str = "x-csrf-token";

#[derive(Debug, Clone)]
pub struct CsrfProtection {
    enabled: bool,
}

impl CsrfProtection {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}

impl<S, B> Transform<S, ServiceRequest> for CsrfProtection
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = CsrfProtectionMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CsrfProtectionMiddleware {
            service: Rc::new(service),
            enabled: self.enabled,
        }))
    }
}

pub struct CsrfProtectionMiddleware<S> {
    service: Rc<S>,
    enabled: bool,
}

impl<S, B> Service<ServiceRequest> for CsrfProtectionMiddleware<S>
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
        if self.enabled && requires_csrf(&request) && !has_valid_csrf(&request) {
            let response = HttpResponse::Forbidden().finish().map_into_right_body();
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

fn requires_csrf(request: &ServiceRequest) -> bool {
    if matches!(
        *request.method(),
        Method::GET | Method::HEAD | Method::OPTIONS
    ) {
        return false;
    }

    !matches!(
        request.path(),
        "/auth/register"
            | "/auth/login"
            | "/auth/phone/otp/request"
            | "/auth/phone/otp/verify"
            | "/auth/password-reset/request"
            | "/auth/password-reset/confirm"
            | "/auth/email-verification/confirm"
    )
}

fn has_valid_csrf(request: &ServiceRequest) -> bool {
    let Some(cookie_token) = request
        .cookie(CSRF_TOKEN_COOKIE)
        .map(|cookie| cookie.value().to_owned())
    else {
        return false;
    };

    request
        .headers()
        .get(CSRF_HEADER)
        .and_then(|value| value.to_str().ok())
        .is_some_and(|header_token| header_token == cookie_token)
}
