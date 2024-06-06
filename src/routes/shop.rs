use crate::models::item::item_metadata::type_enum::ItemTypeEnum;
use crate::models::item::item_struct::Item;
use crate::models::response_data::ResponseItem;
use crate::models::routers_validator_structs::ItemFieldFilters;
use crate::models::routers_validator_structs::{Dice, PaginatedRequest};
use crate::models::shop_structs::ShopTypeEnum;
use crate::models::shop_structs::{ItemSortEnum, ShopPaginatedRequest};
use crate::models::shop_structs::{RandomShopData, ShopSortData};
use crate::services::shop_service;
use crate::services::shop_service::ShopListingResponse;
use crate::AppState;
use actix_web::web::Query;
use actix_web::{error, get, post, web, Responder};
use utoipa::OpenApi;

pub fn init_endpoints(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/shop")
            .service(get_item)
            .service(get_shop_listing)
            .service(get_random_shop_listing),
    );
}

pub fn init_docs(doc: &mut utoipa::openapi::OpenApi) {
    #[derive(OpenApi)]
    #[openapi(
        paths(get_shop_listing, get_item, get_random_shop_listing),
        components(schemas(
            ResponseItem,
            ItemTypeEnum,
            ShopListingResponse,
            Item,
            RandomShopData,
            Dice,
            ShopTypeEnum,
            ItemSortEnum,
        ))
    )]
    struct ApiDoc;

    doc.merge(ApiDoc::openapi());
}

#[utoipa::path(
    get,
    path = "/shop/list",
    tag = "shop",
    params(
        ItemFieldFilters, PaginatedRequest, ShopSortData
    ),
    responses(
        (status=200, description = "Successful Response", body = ShopListingResponse),
        (status=400, description = "Bad request.")
    ),
)]
#[get("/list")]
pub async fn get_shop_listing(
    data: web::Data<AppState>,
    filters: Query<ItemFieldFilters>,
    pagination: Query<PaginatedRequest>,
    sort_data: Query<ShopSortData>,
) -> actix_web::Result<impl Responder> {
    Ok(web::Json(
        shop_service::get_shop_listing(
            &data,
            &filters.0,
            &ShopPaginatedRequest {
                paginated_request: pagination.0,
                shop_sort_data: sort_data.0,
            },
        )
        .await,
    ))
}

#[utoipa::path(
    post,
    path = "/shop/generator",
    tag = "shop",
    request_body(
        content = RandomShopData,
        content_type = "application/json",
    ),
    params(

    ),
    responses(
        (status=200, description = "Successful Response", body = ShopListingResponse),
        (status=400, description = "Bad request.")
    ),
)]
#[post("/generator")]
pub async fn get_random_shop_listing(
    data: web::Data<AppState>,
    web::Json(body): web::Json<RandomShopData>,
) -> actix_web::Result<impl Responder> {
    Ok(web::Json(
        shop_service::generate_random_shop_listing(&data, &body).await,
    ))
}

#[utoipa::path(
    get,
    path = "/shop/item/{item_id}",
    tag = "shop",
    params(
        ("item_id" = String, Path, description = "id of the item to fetch")
    ),
    responses(
        (status=200, description = "Successful Response", body = ResponseItem),
        (status=400, description = "Bad request.")
    ),
)]
#[get("/item/{item_id}")]
pub async fn get_item(
    data: web::Data<AppState>,
    item_id: web::Path<String>,
) -> actix_web::Result<impl Responder> {
    Ok(web::Json(
        shop_service::get_item(&data, sanitize_id(&item_id)?).await,
    ))
}

#[utoipa::path(
    get,
    path = "/shop/traits",
    tag = "shop",
    params(

    ),
    responses(
        (status=200, description = "Successful Response", body = [String]),
        (status=400, description = "Bad request.")
    ),
)]
#[get("/traits")]
pub async fn get_traits_list(data: web::Data<AppState>) -> actix_web::Result<impl Responder> {
    Ok(web::Json(shop_service::get_traits_list(&data).await))
}

fn sanitize_id(creature_id: &str) -> actix_web::Result<i64> {
    let id = creature_id.parse::<i64>();
    match id {
        Ok(s) => Ok(s),
        Err(e) => Err(error::ErrorNotFound(e)),
    }
}
