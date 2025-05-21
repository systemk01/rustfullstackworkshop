use shared::models::{Film, CreateFilm};
use uuid::Uuid;

pub type FilmError = String;
pub type FilmResult<T> = Result<T, FilmError>;

#[async_trait::async_trait]
pub trait FilmRepository: Send + Sync + 'static {
    fn get_films(&self) -> FilmResult<Vec<Film>>;
    fn get_film_by_id(&self, id: &Uuid) -> FilmResult<Film>;
    fn create_film(&self, film: CreateFilm) -> FilmResult<Film>;
    fn update_film(&self, film: &Film) -> FilmResult<Film>;
    fn delete_film(&self, id: &Uuid) -> FilmResult<Uuid>;
    
}