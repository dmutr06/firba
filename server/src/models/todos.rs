use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize, Serialize, FromRow)]
pub struct Todo {
    pub id: Option<String>,
    pub title: String,
    pub checked: bool,
    pub related_to: String,
    #[serde(with = "chrono::serde::ts_seconds_option")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct TodoList {
    pub id: Option<String>,
    pub name: String,
    pub owner: String,
    #[serde(with = "chrono::serde::ts_seconds_option")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize, Deserialize)]
pub struct TodoListCreationDto {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct TodoCreationDto {
    pub title: String,
}

#[derive(Serialize, Deserialize)]
pub struct TodoUpdateDto {
    pub checked: bool,
}
