use uuid::Uuid;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone,PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Film {
    pub id: Uuid, //using uuid for id
    pub title: String,
    pub director: String,
    pub year: u16, //only positive numbers
    pub poster: String, //url to the poster
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}
#[derive(Serialize, Deserialize, Debug, Clone,PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct CreateFilm {
    pub title: String,
    pub director: String,
    pub year: u16, //only positive numbers
    pub poster: String, //url to the poster
}
