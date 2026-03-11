use crate::AppState;
use crate::models::hazard::hazard_field_filter::HazardFieldFilters;
use crate::models::hazard::hazard_listing_struct::{
    HazardListingPaginatedRequest, HazardListingSortData,
};
use crate::models::hazard::hazard_struct::HazardRanges;
use crate::models::response_data::{HazardListingResponse, ResponseHazard};
use crate::models::routers_validator_structs::PaginatedRequest;
use crate::models::shared::action::Action;
use crate::models::shared::game_system_enum::GameSystem;
use crate::services::hazard_service;
use crate::services::sanitizer::sanitize_id;
use actix_web::web::Query;
use actix_web::{Responder, get, post, web};
use utoipa::OpenApi;

pub fn init_endpoints(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/hazard")
            .service(sf_get_hazard_listing)
            .service(sf_get_hazard_traits_list)
            .service(sf_get_hazard_sources_list)
            .service(sf_get_hazard_rarities_list)
            .service(sf_get_hazard_sizes_list)
            .service(sf_get_hazard_ranges)
            .service(sf_get_hazard), // last one, to avoid wildcard matching on source,traits, etc
    );
}

pub fn init_docs() -> utoipa::openapi::OpenApi {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            sf_get_hazard_listing,
            sf_get_hazard_traits_list,
            sf_get_hazard_sources_list,
            sf_get_hazard_rarities_list,
            sf_get_hazard_sizes_list,
            sf_get_hazard_ranges,
            sf_get_hazard,
        ),
        components(schemas(
            HazardFieldFilters,
            HazardListingSortData,
            HazardListingResponse,
            Action,
            HazardRanges
        ))
    )]
    struct ApiDoc;
    ApiDoc::openapi()
}

#[utoipa::path(
    post,
    path = "/hazard/list",
    tags = ["sf", "hazard"],
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
pub async fn sf_get_hazard_listing(
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
            GameSystem::Starfinder,
        )
        .await,
    ))
}

#[utoipa::path(
    get,
    path = "/hazard/traits",
    tags = ["sf", "hazard"],
    params(

    ),
    responses(
        (status=200, description = "Successful Response", body = [String]),
        (status=400, description = "Bad request.")
    ),
)]
#[get("/traits")]
pub async fn sf_get_hazard_traits_list(
    data: web::Data<AppState>,
) -> actix_web::Result<impl Responder> {
    Ok(web::Json(
        hazard_service::get_traits_list(&data, GameSystem::Starfinder).await,
    ))
}

#[utoipa::path(
    get,
    path = "/hazard/sources",
    tags = ["sf", "hazard"],
    params(

    ),
    responses(
        (status=200, description = "Successful Response", body = [String]),
        (status=400, description = "Bad request.")
    ),
)]
#[get("/sources")]
pub async fn sf_get_hazard_sources_list(
    data: web::Data<AppState>,
) -> actix_web::Result<impl Responder> {
    Ok(web::Json(
        hazard_service::get_sources_list(&data, GameSystem::Starfinder).await,
    ))
}

#[utoipa::path(
    get,
    path = "/hazard/rarities",
    tags = ["sf", "hazard"],
    params(

    ),
    responses(
        (status=200, description = "Successful Response", body = [String]),
        (status=400, description = "Bad request.")
    ),
)]
#[get("/rarities")]
pub async fn sf_get_hazard_rarities_list(
    data: web::Data<AppState>,
) -> actix_web::Result<impl Responder> {
    Ok(web::Json(
        hazard_service::get_rarities_list(&data, GameSystem::Starfinder).await,
    ))
}

#[utoipa::path(
    get,
    path = "/hazard/sizes",
    tags = ["sf", "hazard"],
    params(

    ),
    responses(
        (status=200, description = "Successful Response", body = [String]),
        (status=400, description = "Bad request.")
    ),
)]
#[get("/sizes")]
pub async fn sf_get_hazard_sizes_list(
    data: web::Data<AppState>,
) -> actix_web::Result<impl Responder> {
    Ok(web::Json(
        hazard_service::get_sizes_list(&data, GameSystem::Starfinder).await,
    ))
}

#[utoipa::path(
    get,
    path = "/hazard/{hazard_id}",
    tags = ["sf", "hazard"],
    params(
        ("hazard_id" = String, Path, description = "id of the hazard to fetch"),
    ),
    responses(
        (status=200, description = "Successful Response", body = ResponseHazard),
        (status=400, description = "Bad request.")
    ),
)]
#[get("/{hazard_id}")]
pub async fn sf_get_hazard(
    data: web::Data<AppState>,
    hazard_id: web::Path<String>,
) -> actix_web::Result<impl Responder> {
    Ok(web::Json(
        hazard_service::get_hazard(&data, sanitize_id(&hazard_id)?, GameSystem::Starfinder).await,
    ))
}

#[utoipa::path(
    get,
    path = "/hazard/ranges/",
    tags = ["sf", "hazard"],
    params(),
    responses(
        (status=200, description = "Successful Response", body = HazardRanges),
        (status=400, description = "Bad request.")
    ),
)]
#[get("/ranges")]
pub async fn sf_get_hazard_ranges(data: web::Data<AppState>) -> actix_web::Result<impl Responder> {
    Ok(web::Json(
        hazard_service::get_hazard_ranges(&data, GameSystem::Starfinder).await,
    ))
}
