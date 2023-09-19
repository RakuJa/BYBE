use crate::db::db_communicator::is_redis_up;
use actix_web::web::Json;
use actix_web::{get, web};
use maplit::hashmap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::{OpenApi, ToSchema};

#[derive(Serialize, Deserialize, ToSchema)]
pub struct HealthResponse {
    ready: String,
    dependencies: Vec<HashMap<String, String>>,
}

pub fn init_endpoints(cfg: &mut web::ServiceConfig) {
    cfg.service(get_health);
}

pub fn init_docs(doc: &mut utoipa::openapi::OpenApi) {
    #[derive(OpenApi)]
    #[openapi(paths(get_health), components(schemas(HealthResponse)))]
    struct ApiDoc;

    doc.merge(ApiDoc::openapi());
}

#[utoipa::path(
    get,
    path = "/health",
    tag = "health",
    responses(
        (status=200, description = "Successful Response", body = HealthResponse),
        (status=502, description = "The database is offline.")
    ),
)]
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
