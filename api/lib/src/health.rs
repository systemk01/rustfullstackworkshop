use actix_web::{get, web::{self, ServiceConfig}, HttpResponse};

pub fn service(cfg: &mut ServiceConfig) {
    cfg.route("/health", web::get().to(health));
}
/// Health check endpoint
async fn health() -> HttpResponse {
    tracing::info!("Getting health");
    HttpResponse::Ok()
        .append_header(("version", "1.0"))
        .finish()
}
