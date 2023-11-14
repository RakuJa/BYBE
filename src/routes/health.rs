use crate::AppState;
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
pub async fn get_health(data: web::Data<AppState>) -> Json<HealthResponse> {
    let conn = &data.conn;
    let is_db_up = !conn.is_closed();
    Json(HealthResponse {
        ready: is_db_up.to_string(),
        dependencies: vec![hashmap! {
            "name".to_string() => "SQLite database".to_string(),
            "ready".to_string() => is_db_up.to_string(),
            "live".to_string() => is_db_up.to_string(),
            "type".to_string() => "REQUIRED".to_string(),
        }],
    })
}
