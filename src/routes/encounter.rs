use crate::models::encounter_structs::{EncounterDifficultyEnum, EncounterParams};
use crate::services::encounter_service::EncounterInfoResponse;
use crate::services::{bestiary_service, encounter_service};
use actix_web::{get, post, web, Responder, Result};
use utoipa::OpenApi;

pub fn init_endpoints(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/encounter").service(get_encounter_info), //.service(get_generated_random_encounter),
    );
}

pub fn init_docs(doc: &mut utoipa::openapi::OpenApi) {
    #[derive(OpenApi)]
    #[openapi(
        paths(get_encounter_info, get_generated_random_encounter,),
        components(schemas(EncounterInfoResponse, EncounterParams, EncounterDifficultyEnum))
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
    web::Json(form): web::Json<EncounterParams>,
) -> Result<impl Responder> {
    Ok(web::Json(encounter_service::get_encounter_info(form)))
}

#[utoipa::path(
    post,
    path = "/encounter/generator",
    tag = "encounter",
    params(

    ),
    responses(
        (status=200, description = "Successful Response", body = [String]),
        (status=400, description = "Bad request.")
    ),
)]
#[get("/generator")]
pub async fn get_generated_random_encounter() -> Result<impl Responder> {
    Ok(web::Json(bestiary_service::get_families_list()))
}
