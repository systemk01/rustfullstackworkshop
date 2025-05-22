use actix_web::{
    HttpResponse,
    web::{self, ServiceConfig},
};

use crate::film_repository::{FilmRepository, FilmResult};
use shared::models::{CreateFilm, Film};
use uuid::Uuid;

type Repository = web::Data<Box<dyn FilmRepository>>;

pub fn service(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/v1/films")
            .route("", web::get().to(get_films))
            .route("/{film_id}", web::get().to(get_film_by_id))
            .route("", web::post().to(create_film))
            .route("", web::put().to(update_film))
            .route("/{film_id}", web::delete().to(delete_film)),
    );
}

async fn get_films(repo: Repository) -> HttpResponse {
    tracing::info!("Getting films");
    match repo.get_films_db().await {
        Ok(films) => HttpResponse::Ok().json(films),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

async fn get_film_by_id(film_id: web::Path<Uuid>, repo: Repository) -> HttpResponse {
    tracing::info!("Getting film by id: {}", film_id);

    match repo.get_film_by_id_db(&film_id).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(e) => HttpResponse::NotFound().body(format!("Not found error: {:?}", e)),
    }
}

async fn create_film(create_film: web::Json<CreateFilm>, repo: Repository) -> HttpResponse {
    tracing::info!("Creating film");
    match repo.create_film_db(&create_film).await {
        Ok(film) => HttpResponse::Created().json(film),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Internal server error: {:?}", e))
        }
    }
}

async fn update_film(film: web::Json<Film>, repo: Repository) -> HttpResponse {
    tracing::info!("Updating film: {:?}", film);
    match repo.update_film_db(&film).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

async fn delete_film(film_id: web::Path<Uuid>, repo: Repository) -> HttpResponse {
    tracing::info!("Deleting film with id: {}", film_id);
    match repo.delete_film_db(&film_id).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}
