use crate::models::encounter_structs::{EncounterDifficultyEnum, EncounterParams, Party};
use crate::models::routers_validator_structs::RandomEncounterData;
use crate::services::encounter_service;
use crate::services::encounter_service::EncounterInfoResponse;
use crate::services::encounter_service::RandomEncounterGeneratorResponse;
use actix_web::web::Query;
use actix_web::{post, web, Responder, Result};
use utoipa::OpenApi;

pub fn init_endpoints(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/encounter")
            .service(get_encounter_info)
            .service(get_generated_random_encounter),
    );
}

pub fn init_docs(doc: &mut utoipa::openapi::OpenApi) {
    #[derive(OpenApi)]
    #[openapi(
        paths(get_encounter_info, get_generated_random_encounter),
        components(schemas(
            EncounterInfoResponse,
            EncounterParams,
            EncounterDifficultyEnum,
            RandomEncounterGeneratorResponse,
        ))
    )]
    struct ApiDoc;

    doc.merge(ApiDoc::openapi());
}

#[utoipa::path(
    post,
    path = "/encounter/info",
    tag = "encounter",
    request_body(
        content = EncounterParams,
        description = "Party and enemy levels.\
         Could send one value for each, representing the average",
        content_type = "application/json",
    ),
    responses(
        (status=200, description = "Successful Response", body = EncounterInfoResponse),
        (status=400, description = "Bad request.")
    ),
)]
#[post("/info")]
pub async fn get_encounter_info(
    web::Json(body): web::Json<EncounterParams>,
) -> Result<impl Responder> {
    Ok(web::Json(encounter_service::get_encounter_info(body)))
}

#[utoipa::path(
    post,
    path = "/encounter/generator",
    tag = "encounter",
    request_body(
        content = EncounterParams,
        description = "Party and enemy levels.\
         Could send one value for each, representing the average",
        content_type = "application/json",
    ),
    params(
        RandomEncounterData
    ),
    responses(
        (status=200, description = "Successful Response", body = [String]),
        (status=400, description = "Bad request.")
    ),
)]
#[post("/generator")]
pub async fn get_generated_random_encounter(
    data: Query<RandomEncounterData>,
    web::Json(body): web::Json<Party>,
) -> Result<impl Responder> {
    Ok(web::Json(encounter_service::generate_random_encounter(
        data.0,
        body.party_levels,
    )))
}
