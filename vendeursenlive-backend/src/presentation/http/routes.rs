use actix_web::web;

use super::health::health_check;

pub fn configure(config: &mut web::ServiceConfig) {
    config.route("/health", web::get().to(health_check));
}
