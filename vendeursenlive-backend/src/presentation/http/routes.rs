use actix_web::web;

use super::{auth, health::health_check, live_sessions};

pub fn configure(config: &mut web::ServiceConfig) {
    config
        .route("/health", web::get().to(health_check))
        .configure(auth::configure)
        .configure(live_sessions::configure);
}
