use super::{FilmRepository, FilmResult};
use shared::models::{CreateFilm, Film};
use uuid::Uuid;

pub struct PostgresFilmRepository {
    pool: sqlx::PgPool,
}

impl PostgresFilmRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl FilmRepository for PostgresFilmRepository {
    async fn get_films_db(&self) -> FilmResult<Vec<Film>> {
        sqlx::query_as::<_, Film>(
            r#"
      SELECT id, title, director, year, poster, created_at, updated_at
      FROM films
      "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn get_film_by_id_db(&self, film_id: &Uuid) -> FilmResult<Film> {
        let film = sqlx::query_as::<_, Film>(
            r#"
        SELECT id, title, director, year, poster, created_at, updated_at 
        FROM films 
        WHERE id = $1"#,
        )
        .bind(film_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(film)
    }

    async fn create_film_db(&self, create_film: &CreateFilm) -> FilmResult<Film> {
        let film = sqlx::query_as::<_, Film>(
            r#"
            INSERT INTO films (title, director, year, poster) 
            VALUES ($1, $2, $3, $4) RETURNING id, title, director, year, poster, created_at, updated_at"#,
        )
        .bind(&create_film.title)
        .bind(&create_film.director)
        .bind(create_film.year as i16)
        .bind(&create_film.poster)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(film)
    }

    async fn update_film_db(&self, film: &Film) -> FilmResult<Film> {
        let updated_film = sqlx::query_as::<_, Film>(
            r#"
      UPDATE films
      SET title = $2, director = $3, year = $4, poster = $5
      WHERE id = $1
      RETURNING id, title, director, year, poster, created_at, updated_at
      "#,
        )
        .bind(film.id)
        .bind(&film.title)
        .bind(&film.director)
        .bind(film.year as i16)
        .bind(&film.poster)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(updated_film)
    }

    async fn delete_film_db(&self, film_id: &Uuid) -> FilmResult<Uuid> {
        let deleted_film = sqlx::query_scalar::<_, Uuid>(
            r#"
            DELETE FROM films WHERE id = $1 RETURNING id"#,
        )
        .bind(film_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(deleted_film)
    }
}
