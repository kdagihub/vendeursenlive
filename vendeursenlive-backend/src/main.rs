use actix_web::{middleware::Logger, web, App, HttpServer};
use sea_orm::Database;
use tracing::{info, instrument};
use tracing_actix_web::TracingLogger;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use vendeursenlive_backend::{
    infrastructure::{auth::jwt::JwtService, config::AppConfig},
    presentation,
};

#[derive(Clone)]
pub struct AppState {
    pub db: sea_orm::DatabaseConnection,
    pub redis: redis::Client,
    pub jwt: JwtService,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    init_tracing();

    let config = AppConfig::from_env().map_err(to_io_error)?;
    let state = bootstrap_state(&config).await.map_err(to_io_error)?;
    let bind_address = config.server.bind_address();

    info!(
        app_env = ?config.app_env,
        %bind_address,
        "starting VendeursEnLive backend"
    );

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
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
