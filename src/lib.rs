#[macro_use]
extern crate maplit;

pub mod db;
pub mod models;
pub mod services;
pub mod traits;

use sqlx::{Pool, Postgres};

#[derive(Clone)]
pub struct AppState {
    pub conn: Pool<Postgres>,
    pub name_json_path: String,
    pub nick_json_path: String,
}
