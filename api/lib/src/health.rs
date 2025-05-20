use actix_web::{HttpResponse, get};

/*
#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World hey carlo!"
}

#[get("/version")]
async fn version(db: actix_web::web::Data<sqlx::PgPool>) -> String {
    tracing::info!("Getting version");
    let result: Result<String, sqlx::Error> = sqlx::query_scalar("SELECT version()")
        .fetch_one(db.get_ref())
        .await;

    match result {
        Ok(version) => version,
        Err(e) => format!("Error: {:?}", e),
    }
}
*/

#[get("/health")]
async fn health() -> HttpResponse {
    tracing::info!("Getting health");
    HttpResponse::Ok()
        .append_header(("version", "1.0"))
        .finish()
}
