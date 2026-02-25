use crate::AppState;
use crate::models::encounter_structs::{
    AdventureGroupEnum, EncounterChallengeEnum, EncounterParams, RandomCreatureData,
    RandomEncounterData, RandomHazardData,
};
use crate::models::shared::game_system_enum::GameSystem;
use crate::services::encounter_handler::encounter_calculator;
use crate::services::encounter_handler::encounter_calculator::EncounterInfoResponse;
use crate::services::encounter_handler::encounter_calculator::RandomEncounterGeneratorResponse;
use crate::services::encounter_service;
use actix_web::{Responder, Result, post, web};
use utoipa::OpenApi;

pub fn init_endpoints(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/encounter")
            .service(sf_get_encounter_info)
            .service(sf_get_generated_random_encounter),
    );
}

pub fn init_docs() -> utoipa::openapi::OpenApi {
    #[derive(OpenApi)]
    #[openapi(
        paths(sf_get_encounter_info, sf_get_generated_random_encounter),
        components(schemas(
            EncounterInfoResponse,
            RandomEncounterData,
            EncounterParams,
            EncounterChallengeEnum,
            AdventureGroupEnum,
            RandomEncounterGeneratorResponse,
            RandomCreatureData,
            RandomHazardData,
        ))
    )]
    struct ApiDoc;
    ApiDoc::openapi()
}

#[utoipa::path(
    post,
    path = "/encounter/info",
    tags = ["sf", "encounter"],
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
pub async fn sf_get_encounter_info(
    web::Json(body): web::Json<EncounterParams>,
) -> Result<impl Responder> {
    Ok(web::Json(encounter_calculator::get_encounter_info(&body)))
}

#[utoipa::path(
    post,
    path = "/encounter/generator",
    tags = ["sf", "encounter"],
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
pub async fn sf_get_generated_random_encounter(
    data: web::Data<AppState>,
    web::Json(body): web::Json<RandomEncounterData>,
) -> Result<impl Responder> {
    Ok(web::Json(
        encounter_service::generate_random_encounter(&data, body, &GameSystem::Starfinder).await,
    ))
}
