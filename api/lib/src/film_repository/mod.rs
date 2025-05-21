use shared::models::{CreateFilm, Film};
use uuid::Uuid;
pub mod postgres_film_repository;

pub use postgres_film_repository::PostgresFilmRepository;

pub type FilmError = String;
pub type FilmResult<T> = Result<T, FilmError>;

#[async_trait::async_trait]
pub trait FilmRepository: Send + Sync + 'static {
    async fn get_films_db(&self) -> FilmResult<Vec<Film>>;
    async fn get_film_by_id_db(&self, id: &Uuid) -> FilmResult<Film>;
    async fn create_film_db(&self, film: &CreateFilm) -> FilmResult<Film>;
    async fn update_film_db(&self, film: &Film) -> FilmResult<Film>;
    async fn delete_film_db(&self, id: &Uuid) -> FilmResult<Uuid>;
}
