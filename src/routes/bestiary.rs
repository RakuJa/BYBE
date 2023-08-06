use crate::models::routers_validator_structs::{FieldFilters, PaginatedRequest, SortData};
use crate::services::bestiary_service;
use actix_web::{get, web, Responder, Result};
use actix_web_validator::Query;

pub fn init_endpoints(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/bestiary")
            .service(get_bestiary)
            .service(get_creature)
            .service(get_families_list)
            .service(get_rarities_list)
            .service(get_size_list)
            .service(get_alignment_list),
    );
}
#[get("/creature_id={creature_id}")]
pub async fn get_creature(creature_id: web::Path<String>) -> Result<impl Responder> {
    Ok(web::Json(
        bestiary_service::get_creature(&creature_id).await,
    ))
}

#[get("/list")]
pub async fn get_bestiary(
    sort: Query<SortData>,
    filters: Query<FieldFilters>,
    pagination: Query<PaginatedRequest>,
) -> Result<impl Responder> {
    Ok(web::Json(bestiary_service::get_bestiary(
        &sort.0,
        &filters.0,
        &pagination.0,
    )))
}

#[get("/families")]
pub async fn get_families_list() -> Result<impl Responder> {
    Ok(web::Json(bestiary_service::get_families_list()))
}

#[get("/rarities")]
pub async fn get_rarities_list() -> Result<impl Responder> {
    Ok(web::Json(bestiary_service::get_rarities_list()))
}

#[get("/sizes")]
pub async fn get_size_list() -> Result<impl Responder> {
    Ok(web::Json(bestiary_service::get_size_list()))
}

#[get("/alignment")]
pub async fn get_alignment_list() -> Result<impl Responder> {
    Ok(web::Json(bestiary_service::get_alignment_list()))
}
