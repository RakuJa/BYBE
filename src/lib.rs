#[macro_use]
extern crate maplit;

pub mod db;
pub mod models;
pub mod services;
pub mod traits;

use sqlx::{Pool, Sqlite};

#[derive(Clone)]
pub struct AppState {
    pub conn: Pool<Sqlite>,
    pub name_json_path: String,
    pub nick_json_path: String,
}
