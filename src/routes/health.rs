use crate::db::db_communicator::is_redis_up;
use actix_web::get;
use actix_web::web::Json;
use maplit::hashmap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct HealthResponse {
    ready: String,
    dependencies: Vec<HashMap<String, String>>,
}

#[get("/health")]
pub async fn get_health() -> Json<HealthResponse> {
    let is_redis_up = is_redis_up().unwrap_or(false);
    Json(HealthResponse {
        ready: is_redis_up.to_string(),
        dependencies: vec![hashmap! {
            "name".to_string() => "redis database".to_string(),
            "ready".to_string() => is_redis_up.to_string(),
            "live".to_string() => is_redis_up.to_string(),
            "type".to_string() => "REQUIRED".to_string(),
        }],
    })
}
