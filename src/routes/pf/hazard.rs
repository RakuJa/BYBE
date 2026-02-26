use crate::AppState;
use crate::models::hazard::hazard_field_filter::HazardFieldFilters;
use crate::models::hazard::hazard_listing_struct::{
    HazardListingPaginatedRequest, HazardListingSortData,
};
use crate::models::response_data::{HazardListingResponse, ResponseHazard};
use crate::models::routers_validator_structs::PaginatedRequest;
use crate::models::shared::game_system_enum::GameSystem;
use crate::services::hazard_service;
use crate::services::sanitizer::sanitize_id;
use actix_web::web::Query;
use actix_web::{Responder, get, post, web};
use utoipa::OpenApi;

pub fn init_endpoints(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/hazard")
            .service(pf_get_hazard_listing)
            .service(pf_get_hazard)
            .service(pf_get_traits_list)
            .service(pf_get_sources_list)
            .service(pf_get_rarities_list)
            .service(pf_get_sizes_list),
    );
}

pub fn init_docs() -> utoipa::openapi::OpenApi {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            pf_get_hazard_listing,
            pf_get_traits_list,
            pf_get_sources_list,
            pf_get_rarities_list,
            pf_get_sizes_list,
            pf_get_hazard,
        ),
        components(schemas(HazardFieldFilters, HazardListingSortData, HazardListingResponse,))
    )]
    struct ApiDoc;
    ApiDoc::openapi()
}

#[utoipa::path(
    post,
    path = "/hazard/list",
    tags = ["pf", "hazard"],
    request_body(
        content = HazardFieldFilters,
        content_type = "application/json"
    ),
    params(
        PaginatedRequest, HazardListingSortData
    ),
    responses(
        (status=200, description = "Successful Response", body = HazardListingResponse),
        (status=400, description = "Bad request.")
    ),
)]
#[post("/list")]
pub async fn pf_get_hazard_listing(
    data: web::Data<AppState>,
    web::Json(body): web::Json<HazardFieldFilters>,
    pagination: Query<PaginatedRequest>,
    sort_data: Query<HazardListingSortData>,
) -> actix_web::Result<impl Responder> {
    Ok(web::Json(
        hazard_service::get_hazard_listing(
            &data,
            &body,
            &HazardListingPaginatedRequest {
                paginated_request: pagination.0,
                hazard_sort_data: sort_data.0,
            },
            &GameSystem::Pathfinder,
        )
        .await,
    ))
}

#[utoipa::path(
    get,
    path = "/hazard/traits",
    tags = ["pf", "hazard"],
    params(

    ),
    responses(
        (status=200, description = "Successful Response", body = [String]),
        (status=400, description = "Bad request.")
    ),
)]
#[get("/traits")]
pub async fn pf_get_traits_list(data: web::Data<AppState>) -> actix_web::Result<impl Responder> {
    Ok(web::Json(
        hazard_service::get_traits_list(&data, &GameSystem::Pathfinder).await,
    ))
}

#[utoipa::path(
    get,
    path = "/hazard/sources",
    tags = ["pf", "hazard"],
    params(

    ),
    responses(
        (status=200, description = "Successful Response", body = [String]),
        (status=400, description = "Bad request.")
    ),
)]
#[get("/sources")]
pub async fn pf_get_sources_list(data: web::Data<AppState>) -> actix_web::Result<impl Responder> {
    Ok(web::Json(
        hazard_service::get_sources_list(&data, &GameSystem::Pathfinder).await,
    ))
}

#[utoipa::path(
    get,
    path = "/hazard/rarities",
    tags = ["pf", "hazard"],
    params(

    ),
    responses(
        (status=200, description = "Successful Response", body = [String]),
        (status=400, description = "Bad request.")
    ),
)]
#[get("/rarities")]
pub async fn pf_get_rarities_list(data: web::Data<AppState>) -> actix_web::Result<impl Responder> {
    Ok(web::Json(
        hazard_service::get_rarities_list(&data, &GameSystem::Pathfinder).await,
    ))
}

#[utoipa::path(
    get,
    path = "/hazard/sizes",
    tags = ["pf", "hazard"],
    params(

    ),
    responses(
        (status=200, description = "Successful Response", body = [String]),
        (status=400, description = "Bad request.")
    ),
)]
#[get("/sizes")]
pub async fn pf_get_sizes_list(data: web::Data<AppState>) -> actix_web::Result<impl Responder> {
    Ok(web::Json(
        hazard_service::get_sizes_list(&data, &GameSystem::Pathfinder).await,
    ))
}

#[utoipa::path(
    get,
    path = "/hazard/{hazard_id}",
    tags = ["pf", "hazard"],
    params(
        ("hazard_id" = String, Path, description = "id of the hazard to fetch"),
    ),
    responses(
        (status=200, description = "Successful Response", body = ResponseHazard),
        (status=400, description = "Bad request.")
    ),
)]
#[get("/{hazard_id}")]
pub async fn pf_get_hazard(
    data: web::Data<AppState>,
    hazard_id: web::Path<String>,
) -> actix_web::Result<impl Responder> {
    Ok(web::Json(
        hazard_service::get_hazard(&data, sanitize_id(&hazard_id)?, &GameSystem::Pathfinder).await,
    ))
}
