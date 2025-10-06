use crate::AppState;
use crate::models::npc::ancestry_enum::PfAncestry;
use crate::models::npc::class_enum::ClassFilter;
use crate::models::npc::culture_enum::PfCulture;
use crate::models::npc::gender_enum::Gender;
use crate::models::npc::job_enum::JobFilter;
use crate::models::npc::request_npc_struct::{AncestryData, RandomNameData, RandomNpcData};
use crate::models::response_data::ResponseNpc;
use crate::models::routers_validator_structs::LevelData;
use crate::models::shared::game_system_enum::GameSystem;
use crate::services::pf::npc_service as pf_npc_service;
use crate::services::shared::npc_service;
use actix_web::error::ErrorBadRequest;
use actix_web::{Responder, get, post, web};
use utoipa::OpenApi;

pub fn init_endpoints(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/npc")
            .service(pf_get_random_npc)
            .service(pf_get_random_ancestry)
            .service(pf_get_random_culture)
            .service(pf_get_random_class)
            .service(pf_get_random_gender)
            .service(pf_get_random_job)
            .service(pf_get_random_names)
            .service(pf_get_random_nickname)
            .service(pf_get_random_level)
            .service(pf_get_npc_classes_list)
            .service(pf_get_npc_genders_list)
            .service(pf_get_npc_jobs_list)
            .service(pf_get_npc_ancestries_list)
            .service(pf_get_npc_cultures_list),
    );
}

pub fn init_docs() -> utoipa::openapi::OpenApi {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            pf_get_random_npc,
            pf_get_random_ancestry,
            pf_get_random_culture,
            pf_get_random_class,
            pf_get_random_gender,
            pf_get_random_job,
            pf_get_random_names,
            pf_get_random_nickname,
            pf_get_random_level,
            pf_get_npc_classes_list,
            pf_get_npc_genders_list,
            pf_get_npc_jobs_list,
            pf_get_npc_ancestries_list,
            pf_get_npc_cultures_list
        ),
        components(schemas(ResponseNpc, RandomNpcData, RandomNameData, AncestryData))
    )]
    struct ApiDoc;

    ApiDoc::openapi()
}

#[utoipa::path(
    post,
    path = "/npc/generator",
    tags = ["pf", "npc"],
    request_body(
        content = RandomNpcData,
        content_type = "application/json",
    ),
    params(

    ),
    responses(
        (status=200, description = "Successful Response", body = ResponseNpc),
        (status=400, description = "Bad request.")
    ),
)]
#[post("/generator")]
pub async fn pf_get_random_npc(
    data: web::Data<AppState>,
    body: Option<web::Json<RandomNpcData>>,
) -> actix_web::Result<impl Responder> {
    let npc_data = body
        .map(|x| x.0)
        .unwrap_or_else(|| RandomNpcData::default_with_system(GameSystem::Pathfinder));
    if npc_data.is_valid() {
        npc_service::generate_random_npc(&data, npc_data)
            .map_or_else(|_| Err(ErrorBadRequest(
                "Given parameters are not valid. Check for conflicts e.g. Ancestry's unsupported gender chosen"
            )), |npc| Ok(web::Json(npc)))
    } else {
        Err(ErrorBadRequest(
            "Given parameters are not valid. Check for conflicts e.g. Ancestry's unsupported gender chosen",
        ))
    }
}

#[utoipa::path(
    post,
    path = "/npc/generator/class",
    tags = ["pf", "npc"],
    request_body(
        content = Option<ClassFilter>,
        content_type = "application/json",
    ),
    params(

    ),
    responses(
        (status=200, description = "Successful Response", body = String),
        (status=400, description = "Bad request.")
    ),
)]
#[post("/generator/class")]
pub async fn pf_get_random_class(
    body: Option<web::Json<ClassFilter>>,
) -> actix_web::Result<impl Responder> {
    Ok(web::Json(npc_service::get_random_class(
        if let Some(body) = body {
            body.0
        } else {
            ClassFilter::FromPf(None)
        },
    )))
}

#[utoipa::path(
    post,
    path = "/npc/generator/level",
    tags = ["pf", "npc"],
    request_body(
        content = LevelData,
        content_type = "application/json",
    ),
    params(

    ),
    responses(
        (status=200, description = "Successful Response", body = i64),
        (status=400, description = "Bad request.")
    ),
)]
#[post("/generator/level")]
pub async fn pf_get_random_level(
    body: Option<web::Json<LevelData>>,
) -> actix_web::Result<impl Responder> {
    if let Some(json) = &body
        && !json.0.is_data_valid()
    {
        return Err(ErrorBadRequest(
            "Given parameters are not valid. Check for conflicts e.g. min lvl > max lvl",
        ));
    }
    Ok(web::Json(npc_service::get_random_level(body.map(|x| x.0))))
}

#[utoipa::path(
    post,
    path = "/npc/generator/ancestry",
    tags = ["pf", "npc"],
    request_body(
        content = Option<Vec<PfAncestry>>,
        content_type = "application/json",
    ),
    params(

    ),
    responses(
        (status=200, description = "Successful Response", body = String),
        (status=400, description = "Bad request.")
    ),
)]
#[post("/generator/ancestry")]
pub async fn pf_get_random_ancestry(
    body: Option<web::Json<Vec<PfAncestry>>>,
) -> actix_web::Result<impl Responder> {
    Ok(web::Json(pf_npc_service::get_random_ancestry(
        if let Some(body) = body {
            Some(body.0)
        } else {
            None
        },
    )))
}

#[utoipa::path(
    post,
    path = "/npc/generator/culture",
    tags = ["pf", "npc"],
    request_body(
        content = Option<Vec<PfCulture>>,
        content_type = "application/json",
    ),
    params(

    ),
    responses(
        (status=200, description = "Successful Response", body = String),
        (status=400, description = "Bad request.")
    ),
)]
#[post("/generator/culture")]
pub async fn pf_get_random_culture(
    body: Option<web::Json<Vec<PfCulture>>>,
) -> actix_web::Result<impl Responder> {
    Ok(web::Json(pf_npc_service::get_random_culture(
        if let Some(body) = body {
            Some(body.0)
        } else {
            None
        },
    )))
}

#[utoipa::path(
    post,
    path = "/npc/generator/gender",
    tags = ["pf", "npc"],
    request_body(
        content = Option<Vec<Gender>>,
        content_type = "application/json",
    ),
    params(

    ),
    responses(
        (status=200, description = "Successful Response", body = String),
        (status=400, description = "Bad request.")
    ),
)]
#[post("/generator/gender")]
pub async fn pf_get_random_gender(
    body: Option<web::Json<Vec<Gender>>>,
) -> actix_web::Result<impl Responder> {
    npc_service::get_random_gender(if let Some(body) = body {
        Some(body.0)
    } else {
        None
    }).
        map_or_else(|_| Err(ErrorBadRequest(
           "Given parameters are not valid. Check for empty whitelist vector (if whitelist is empty there cannot be a valid gender)"
        )), |g| Ok(web::Json(g)))
}

#[utoipa::path(
    post,
    path = "/npc/generator/job",
    tags = ["pf", "npc"],
    request_body(
        content = Option<JobFilter>,
        content_type = "application/json",
    ),
    params(

    ),
    responses(
        (status=200, description = "Successful Response", body = String),
        (status=400, description = "Bad request.")
    ),
)]
#[post("/generator/job")]
pub async fn pf_get_random_job(
    body: Option<web::Json<JobFilter>>,
) -> actix_web::Result<impl Responder> {
    Ok(web::Json(npc_service::get_random_job(
        if let Some(body) = body {
            body.0
        } else {
            JobFilter::FromPf(None)
        },
    )))
}

#[utoipa::path(
    post,
    path = "/npc/generator/nickname",
    tags = ["pf", "npc"],
    request_body(
    ),
    params(

    ),
    responses(
        (status=200, description = "Successful Response", body = String),
        (status=400, description = "Bad request.")
    ),
)]
#[post("/generator/nickname")]
pub async fn pf_get_random_nickname(
    data: web::Data<AppState>,
) -> actix_web::Result<impl Responder> {
    Ok(web::Json(npc_service::generate_random_nickname(
        &data.nick_json_path,
    )))
}

#[utoipa::path(
    post,
    path = "/npc/generator/names",
    tags = ["pf", "npc"],
    request_body(
        content = RandomNameData,
        content_type = "application/json",
    ),
    params(

    ),
    responses(
        (status=200, description = "Successful Response", body = [String]),
        (status=400, description = "Bad request.")
    ),
)]
#[post("/generator/names")]
pub async fn pf_get_random_names(
    data: web::Data<AppState>,
    body: Option<web::Json<RandomNameData>>,
) -> actix_web::Result<impl Responder> {
    if let Some(json) = body {
        let rd = json.0;
        if rd.is_valid() {
            Ok(web::Json(npc_service::generate_random_names(
                rd,
                &data.name_json_path,
            )))
        } else {
            Err(ErrorBadRequest(
                "Given parameters are not valid. Check for conflicts e.g. Ancestry unsupported gender chosen",
            ))
        }
    } else {
        Ok(web::Json(npc_service::generate_random_names(
            RandomNameData::default_with_system(GameSystem::Pathfinder),
            &data.name_json_path,
        )))
    }
}

#[utoipa::path(
    get,
    path = "/npc/ancestries",
    tags = ["pf", "npc"],
    params(

    ),
    responses(
        (status=200, description = "Successful Response", body = [AncestryData]),
        (status=400, description = "Bad request.")
    ),
)]
#[get("/ancestries")]
pub async fn pf_get_npc_ancestries_list() -> actix_web::Result<impl Responder> {
    Ok(web::Json(npc_service::get_ancestries_list(
        GameSystem::Pathfinder,
    )))
}

#[utoipa::path(
    get,
    path = "/npc/cultures",
    tags = ["pf", "npc"],
    params(

    ),
    responses(
        (status=200, description = "Successful Response", body = [String]),
        (status=400, description = "Bad request.")
    ),
)]
#[get("/cultures")]
pub async fn pf_get_npc_cultures_list() -> actix_web::Result<impl Responder> {
    Ok(web::Json(npc_service::get_cultures_list()))
}

#[utoipa::path(
    get,
    path = "/npc/genders",
    tags = ["pf", "npc"],
    params(

    ),
    responses(
        (status=200, description = "Successful Response", body = [String]),
        (status=400, description = "Bad request.")
    ),
)]
#[get("/genders")]
pub async fn pf_get_npc_genders_list() -> actix_web::Result<impl Responder> {
    Ok(web::Json(npc_service::get_genders_list()))
}

#[utoipa::path(
    get,
    path = "/npc/jobs",
    tags = ["pf", "npc"],
    params(

    ),
    responses(
        (status=200, description = "Successful Response", body = [String]),
        (status=400, description = "Bad request.")
    ),
)]
#[get("/jobs")]
pub async fn pf_get_npc_jobs_list() -> actix_web::Result<impl Responder> {
    Ok(web::Json(npc_service::get_jobs_list(
        &GameSystem::Pathfinder,
    )))
}

#[utoipa::path(
    get,
    path = "/npc/classes",
    tags = ["pf", "npc"],
    params(

    ),
    responses(
        (status=200, description = "Successful Response", body = [String]),
        (status=400, description = "Bad request.")
    ),
)]
#[get("/classes")]
pub async fn pf_get_npc_classes_list() -> actix_web::Result<impl Responder> {
    Ok(web::Json(npc_service::get_classes_list(
        &GameSystem::Pathfinder,
    )))
}
