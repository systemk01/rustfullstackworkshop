use serde::{Deserialize, Serialize};
use uuid::Uuid;

//sqlx is only compiled when the feature "backend" is enabled

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Film {
    pub id: Uuid, //using uuid for id
    pub title: String,
    pub director: String,
    #[cfg_attr(feature = "backend", sqlx(try_from = "i16"))]
    pub year: u16, //only positive numbers
    pub poster: String, //url to the poster
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct CreateFilm {
    pub title: String,
    pub director: String,
    #[cfg_attr(feature = "backend", sqlx(try_from = "i16"))]
    pub year: u16, //only positive numbers
    pub poster: String, //url to the poster
}
