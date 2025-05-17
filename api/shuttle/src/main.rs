use actix_web::{get, web::ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use sqlx::Executor;

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World hey carlo!"
}

/// This function is the entry point for the Actix web application.

#[shuttle_runtime::main]
async fn actix_web(#[shuttle_shared_db::Postgres] pool: sqlx::PgPool) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    // initialize the database if not already initialized
    let pool = actix_web::web::Data::new(pool);

pool.execute(include_str!("../../db/schema.sql"))
.await
.map_err(CustomError::new)?;

    
    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(pool).service(hello_world).service(version);
    };

    Ok(config.into())
    
}

#[get("/version")]
async fn version(db: actix_web::web::Data<sqlx::PgPool>) -> String {
    let result: Result<String, sqlx::Error> = sqlx::query_scalar("SELECT version()")
        .fetch_one(db.get_ref())
        .await;

    match result {
        Ok(version) => version,
        Err(e) => format!("Error: {:?}", e),
    }
}


//api/db/schema.sql