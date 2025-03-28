use crate::AppState;
use crate::models::encounter_structs::{
    AdventureGroupEnum, EncounterChallengeEnum, EncounterParams, RandomEncounterData,
};
use crate::services::encounter_service;
use crate::services::encounter_service::EncounterInfoResponse;
use crate::services::encounter_service::RandomEncounterGeneratorResponse;
use actix_web::{Responder, Result, post, web};
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
            RandomEncounterData,
            EncounterParams,
            EncounterChallengeEnum,
            AdventureGroupEnum,
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
    Ok(web::Json(encounter_service::get_encounter_info(&body)))
}

#[utoipa::path(
    post,
    path = "/encounter/generator",
    tag = "encounter",
    request_body(
        content = RandomEncounterData,
        description = "Party levels as a vector of integers,\
         if min and max are not set they will not be considered. If only one of them is set, \
         the other one will be set at the same value.",
        content_type = "application/json",
    ),
    params(

    ),
    responses(
        (status=200, description = "Successful Response", body = RandomEncounterGeneratorResponse),
        (status=400, description = "Bad request.")
    ),
)]
#[post("/generator")]
pub async fn get_generated_random_encounter(
    data: web::Data<AppState>,
    web::Json(body): web::Json<RandomEncounterData>,
) -> Result<impl Responder> {
    Ok(web::Json(
        encounter_service::generate_random_encounter(&data, body).await,
    ))
}
