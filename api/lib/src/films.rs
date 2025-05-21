use actix_web::{
    HttpResponse,
    web::{self, ServiceConfig},
};

pub fn service(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/v1/films")
            .route("", web::get().to(get_films))
            .route("/{film_id}", web::get().to(get_film_by_id))
            .route("", web::post().to(create_film))
            .route("/{film_id}", web::put().to(update_film))
            .route("/{film_id}", web::delete().to(delete_film)),
    );
}

async fn get_films() -> HttpResponse {
    tracing::info!("Getting films");
    HttpResponse::Ok().finish()
}

async fn get_film_by_id(film_id: web::Path<String>) -> HttpResponse {
    tracing::info!("Getting film by id: {}", film_id);
    HttpResponse::Ok().finish()
}

async fn create_film() -> HttpResponse {
    tracing::info!("Creating film");
    HttpResponse::Created().finish()
}

async fn update_film(film_id: web::Path<String>) -> HttpResponse {
    tracing::info!("Updating film with id: {}", film_id);
    HttpResponse::Ok().finish()
}

async fn delete_film(film_id: web::Path<String>) -> HttpResponse {
    tracing::info!("Deleting film with id: {}", film_id);
    HttpResponse::NoContent().finish()
}
