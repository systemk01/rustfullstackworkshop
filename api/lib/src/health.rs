use actix_web::{
    HttpResponse,
    web::{self, ServiceConfig},
};
pub const API_VERSION: &str = "v0.0.1_healthc";

pub fn service(cfg: &mut ServiceConfig) {
    cfg.route("/health", web::get().to(health));
}
/// Health check endpoint
async fn health() -> HttpResponse {
    tracing::info!("Getting health");
    HttpResponse::Ok()
        .append_header(("version", API_VERSION))
        .finish()
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;

    #[actix_rt::test]
    async fn test_health_check() {
        let res = health().await;
        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(res.status().is_success(), true);
        let data = res.headers().get("version").and_then(|v| v.to_str().ok());
        assert_eq!(data, Some(API_VERSION));
    }
}
