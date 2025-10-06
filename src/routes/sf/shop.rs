use crate::AppState;
use crate::models::item::armor_struct::ArmorData;
use crate::models::item::item_metadata::type_enum::ItemTypeEnum;
use crate::models::item::item_metadata::type_enum::WeaponTypeEnum;
use crate::models::item::item_struct::Item;
use crate::models::item::shield_struct::ShieldData;
use crate::models::item::weapon_struct::DamageData;
use crate::models::item::weapon_struct::WeaponData;
use crate::models::response_data::ResponseItem;
use crate::models::response_data::ShopListingResponse;
use crate::models::routers_validator_structs::ItemFieldFilters;
use crate::models::routers_validator_structs::{Dice, PaginatedRequest};
use crate::models::shop_structs::ShopTemplateData;
use crate::models::shop_structs::ShopTemplateEnum;
use crate::models::shop_structs::{ItemSortEnum, ShopPaginatedRequest};
use crate::models::shop_structs::{RandomShopData, ShopSortData};
use crate::services::sf::shop_service;
use crate::services::shared::sanitizer::sanitize_id;
use actix_web::web::Query;
use actix_web::{Responder, get, post, web};
use utoipa::OpenApi;

pub fn init_endpoints(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/shop")
            .service(sf_get_item)
            .service(sf_get_shop_listing)
            .service(sf_get_items_traits_list)
            .service(sf_get_templates_data)
            .service(sf_get_items_sources_list)
            .service(sf_get_random_shop_listing),
    );
}

pub fn init_docs() -> utoipa::openapi::OpenApi {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            sf_get_shop_listing,
            sf_get_item,
            sf_get_random_shop_listing,
            sf_get_items_traits_list,
            sf_get_templates_data,
            sf_get_items_sources_list
        ),
        components(schemas(
            ResponseItem,
            ItemTypeEnum,
            ShopListingResponse,
            Item,
            RandomShopData,
            Dice,
            ShopTemplateEnum,
            ShopTemplateData,
            ItemFieldFilters,
            ItemSortEnum,
            DamageData,
            WeaponData,
            ArmorData,
            ShieldData,
            WeaponTypeEnum
        ))
    )]
    struct ApiDoc;

    ApiDoc::openapi()
}

#[utoipa::path(
    post,
    path = "/shop/list",
    tags = ["sf", "shop"],
    request_body(
        content = ItemFieldFilters,
        content_type = "application/json",
    ),
    params(
        PaginatedRequest, ShopSortData
    ),
    responses(
        (status=200, description = "Successful Response", body = ShopListingResponse),
        (status=400, description = "Bad request.")
    ),
)]
#[post("/list")]
pub async fn sf_get_shop_listing(
    data: web::Data<AppState>,
    web::Json(body): web::Json<ItemFieldFilters>,
    pagination: Query<PaginatedRequest>,
    sort_data: Query<ShopSortData>,
) -> actix_web::Result<impl Responder> {
    Ok(web::Json(
        shop_service::get_shop_listing(
            &data,
            &body,
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
    tags = ["sf", "shop"],
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
pub async fn sf_get_random_shop_listing(
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
    tags = ["sf", "shop"],
    params(
        ("item_id" = String, Path, description = "id of the item to fetch")
    ),
    responses(
        (status=200, description = "Successful Response", body = ResponseItem),
        (status=400, description = "Bad request.")
    ),
)]
#[get("/item/{item_id}")]
pub async fn sf_get_item(
    data: web::Data<AppState>,
    item_id: web::Path<String>,
) -> actix_web::Result<impl Responder> {
    Ok(web::Json(
        shop_service::get_item(&data, sanitize_id(&item_id)?).await,
    ))
}

#[utoipa::path(
    get,
    path = "/shop/sources",
    tags = ["sf", "shop"],
    params(

    ),
    responses(
        (status=200, description = "Successful Response", body = [String]),
        (status=400, description = "Bad request.")
    ),
)]
#[get("/sources")]
pub async fn sf_get_items_sources_list(
    data: web::Data<AppState>,
) -> actix_web::Result<impl Responder> {
    Ok(web::Json(shop_service::get_sources_list(&data).await))
}

#[utoipa::path(
    get,
    path = "/shop/traits",
    tags = ["sf", "shop"],
    params(

    ),
    responses(
        (status=200, description = "Successful Response", body = [String]),
        (status=400, description = "Bad request.")
    ),
)]
#[get("/traits")]
pub async fn sf_get_items_traits_list(
    data: web::Data<AppState>,
) -> actix_web::Result<impl Responder> {
    Ok(web::Json(shop_service::get_traits_list(&data).await))
}

#[utoipa::path(
    get,
    path = "/shop/templates_data",
    tags = ["sf", "shop"],
    params(

    ),
    responses(
        (status=200, description = "Successful Response", body = [ShopTemplateData]),
        (status=400, description = "Bad request.")
    ),
)]
#[get("/templates_data")]
pub async fn sf_get_templates_data() -> actix_web::Result<impl Responder> {
    Ok(web::Json(shop_service::get_shop_templates_data()))
}
