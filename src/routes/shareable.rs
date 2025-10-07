use crate::models::shearable_data::ShareableEncounter;
use crate::models::shearable_data::ShareableNpcList;
use crate::models::shearable_data::ShareableShop;
use crate::traits::base64::base64_decode::Base64Decode;
use crate::traits::base64::base64_encode::Base64Encode;
use actix_web::error::ErrorBadRequest;
use actix_web::{Responder, Result, get, post, web};
use utoipa::OpenApi;

pub fn init_endpoints(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/shareable")
            .service(get_shop_shareable_link)
            .service(get_npc_shareable_link)
            .service(get_encounter_shareable_link)
            .service(get_shop_from_shareable_link)
            .service(get_npc_list_from_shareable_link)
            .service(get_encounter_from_shareable_link),
    );
}

pub fn init_docs() -> utoipa::openapi::OpenApi {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            get_shop_shareable_link,
            get_npc_shareable_link,
            get_encounter_shareable_link,
            get_shop_from_shareable_link,
            get_npc_list_from_shareable_link,
            get_encounter_from_shareable_link
        ),
        components(schemas(ShareableNpcList, ShareableShop, ShareableEncounter,))
    )]
    struct ApiDoc;
    ApiDoc::openapi()
}

#[utoipa::path(
    post,
    path = "/shareable/shop/encode",
    tags = ["shop", "shareable"],
    request_body(
        content = ShareableShop,
        description = "Get unique link for given shop data",
        content_type = "application/json",
    ),
    responses(
        (status=200, description = "Successful Response", body = String),
        (status=400, description = "Bad request.")
    ),
)]
#[post("/shop/encode")]
pub async fn get_shop_shareable_link(
    web::Json(body): web::Json<ShareableShop>,
) -> Result<impl Responder> {
    body.encode()
        .await
        .map_or_else(|_| Err(ErrorBadRequest("Invalid JSON data for Shop")), Ok)
}

#[utoipa::path(
    post,
    path = "/shareable/npc/encode",
    tags = ["npc", "shareable"],
    request_body(
        content = ShareableNpcList,
        description = "Get unique link for given npc list data",
        content_type = "application/json",
    ),
    responses(
        (status=200, description = "Successful Response", body = String),
        (status=400, description = "Bad request.")
    ),
)]
#[post("/npc/encode")]
pub async fn get_npc_shareable_link(
    web::Json(body): web::Json<ShareableNpcList>,
) -> Result<impl Responder> {
    body.encode()
        .await
        .map_or_else(|_| Err(ErrorBadRequest("Invalid JSON data for Npc")), Ok)
}

#[utoipa::path(
    post,
    path = "/shareable/encounter/encode",
    tags = ["encounter", "shareable"],
    request_body(
        content = ShareableEncounter,
        description = "Get unique link for given encounter data",
        content_type = "application/json",
    ),
    responses(
        (status=200, description = "Successful Response", body = String),
        (status=400, description = "Bad request.")
    ),
)]
#[post("/encounter/encode")]
pub async fn get_encounter_shareable_link(
    web::Json(body): web::Json<ShareableEncounter>,
) -> Result<impl Responder> {
    body.encode().await.map_or_else(
        |_| Err(ErrorBadRequest("Invalid JSON data for Encounter")),
        Ok,
    )
}

#[utoipa::path(
    get,
    path = "/shareable/shop/decode/{encoded_data}",
    tags = ["shop", "shareable"],
    responses(
        (status=200, description = "Successful Response", body = [ShareableShop]),
        (status=400, description = "Bad request.")
    ),
)]
#[get("/shop/decode/{encoded_data}")]
pub async fn get_shop_from_shareable_link(
    encoded_data: web::Path<String>,
) -> Result<impl Responder> {
    println!("{}", encoded_data.clone());
    ShareableShop::decode(encoded_data.clone())
        .await
        .map_or_else(
            |_| Err(ErrorBadRequest("Invalid link for shop")),
            |res| Ok(web::Json(res)),
        )
}

#[utoipa::path(
    get,
    path = "/shareable/npc/decode/{encoded_data}",
    tags = ["npc", "shareable"],
    responses(
        (status=200, description = "Successful Response", body = [ShareableNpcList]),
        (status=400, description = "Bad request.")
    ),
)]
#[get("/npc/decode/{encoded_data}")]
pub async fn get_npc_list_from_shareable_link(
    encoded_data: web::Path<String>,
) -> Result<impl Responder> {
    ShareableNpcList::decode(encoded_data.clone())
        .await
        .map_or_else(
            |_| Err(ErrorBadRequest("Invalid link for npc list")),
            |res| Ok(web::Json(res)),
        )
}

#[utoipa::path(
    get,
    path = "/shareable/encounter/decode/{encoded_data}",
    tags = ["encounter", "shareable"],
    responses(
        (status=200, description = "Successful Response", body = [ShareableEncounter]),
        (status=400, description = "Bad request.")
    ),
)]
#[get("/encounter/decode/{encoded_data}")]
pub async fn get_encounter_from_shareable_link(
    encoded_data: web::Path<String>,
) -> Result<impl Responder> {
    ShareableEncounter::decode(encoded_data.clone())
        .await
        .map_or_else(
            |_| Err(ErrorBadRequest("Invalid link for encounter")),
            |res| Ok(web::Json(res)),
        )
}
