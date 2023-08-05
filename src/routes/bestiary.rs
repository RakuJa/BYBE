use crate::models::routers_validator_structs::{FieldFilters, PaginatedRequest, SortData};
use crate::services::bestiary_service;
use actix_web::{get, web, Responder, Result};
use actix_web_validator::Query;

#[get("/bestiary/creature_id={creature_id}")]
pub async fn get_creature(creature_id: web::Path<String>) -> Result<impl Responder> {
    Ok(web::Json(
        bestiary_service::get_creature(&creature_id).await,
    ))
}

#[get("/bestiary/list")]
pub async fn get_bestiary(
    sort: Query<SortData>,
    filters: Query<FieldFilters>,
    pagination: Query<PaginatedRequest>,
) -> Result<impl Responder> {
    log::info!("We in");
    Ok(web::Json(
        bestiary_service::get_bestiary(&sort.0, &filters.0, &pagination.0).await,
    ))
}

#[get("/keys")]
pub async fn get_keys() -> Result<impl Responder> {
    Ok(web::Json(bestiary_service::get_keys().await))
}
