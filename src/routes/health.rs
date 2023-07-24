

use std::collections::HashMap;
use maplit::hashmap;
use rocket::fairing::AdHoc;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::{deadpool_redis, Database, Connection};
use crate::rocket;

#[derive(Database)]
#[database("redis_pool")]
pub struct Db(deadpool_redis::Pool);

#[derive(Serialize, Deserialize)]
struct HealthResponse {
    ready: String,
    dependencies: Vec<HashMap<String, String>>
}


#[get("/")]
async fn get_health(mut db: Connection<Db>) -> Json<HealthResponse> {
    Json(HealthResponse {
        ready: true.to_string(),
        dependencies: vec![
            hashmap! {
                "name".to_string() => "redis database".to_string(),
                "ready".to_string() => true.to_string(),
                "live".to_string() => true.to_string(),
                "type".to_string() => "REQUIRED".to_string(),
            }
        ]
    })

}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("redis stage", |rocket| async {
        rocket.attach(Db::init())
            .mount("/health", routes![get_health])
    })
}