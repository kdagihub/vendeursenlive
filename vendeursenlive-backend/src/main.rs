use actix_cors::Cors;
use actix_web::{
    http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    middleware::Logger,
    web, App, HttpServer,
};
use sea_orm::Database;
use tracing::{info, instrument};
use tracing_actix_web::TracingLogger;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use vendeursenlive_backend::{
    infrastructure::{auth::jwt::JwtService, config::AppConfig, database::migrations},
    presentation::{
        self,
        middlewares::{csrf::CsrfProtection, csrf::CSRF_HEADER, rate_limit::RateLimit},
    },
    AppState,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    init_tracing();

    let config = AppConfig::from_env().map_err(to_io_error)?;
    let state = bootstrap_state(&config).await.map_err(to_io_error)?;
    let bind_address = config.server.bind_address();
    let cors_allowed_origins = config.cors.allowed_origins.clone();
    let rate_limit = RateLimit::new(
        config.rate_limit.enabled,
        config.rate_limit.requests_per_minute,
    );
    let csrf_protection = CsrfProtection::new(config.security.csrf_protection_enabled);

    info!(
        app_env = ?config.app_env,
        %bind_address,
        "starting VendeursEnLive backend"
    );

    HttpServer::new(move || {
        let mut cors = Cors::default()
            .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                CONTENT_TYPE,
                AUTHORIZATION,
                ACCEPT,
                actix_web::http::header::HeaderName::from_static(CSRF_HEADER),
            ])
            .supports_credentials()
            .max_age(3600);

        for origin in &cors_allowed_origins {
            cors = cors.allowed_origin(origin);
        }

        App::new()
            .app_data(web::Data::new(state.clone()))
            .wrap(rate_limit.clone())
            .wrap(csrf_protection.clone())
            .wrap(cors)
            .wrap(TracingLogger::default())
            .wrap(Logger::default())
            .configure(presentation::http::routes::configure)
    })
    .bind(bind_address)?
    .run()
    .await
}

#[instrument(skip_all)]
async fn bootstrap_state(config: &AppConfig) -> anyhow::Result<AppState> {
    let db = Database::connect(&config.database.url).await?;

    if config.migrations.run_on_start {
        migrations::run_pending_migrations(&db).await?;
    }

    let redis = redis::Client::open(config.redis.url.as_str())?;
    let mut redis_connection = redis.get_multiplexed_async_connection().await?;
    let _: String = redis::cmd("PING")
        .query_async(&mut redis_connection)
        .await?;

    info!("database and redis connections initialized");

    Ok(AppState {
        db,
        redis,
        jwt: JwtService::new(&config.auth),
        auth_config: config.auth.clone(),
        tiktok_config: config.tiktok.clone(),
        google_config: config.google.clone(),
        email_config: config.email.clone(),
        ikoddi_config: config.ikoddi.clone(),
    })
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(tracing_subscriber::fmt::layer())
        .init();
}

fn to_io_error(error: impl std::fmt::Display) -> std::io::Error {
    std::io::Error::other(error.to_string())
}
