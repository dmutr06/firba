mod db;

pub mod jwt;
pub mod models;

pub mod todos;
pub mod auth;
pub mod users;


#[derive(Clone, Debug)]
pub struct AppState;

pub use db::init_db;
