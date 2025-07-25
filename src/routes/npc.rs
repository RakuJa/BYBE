use crate::AppState;
use crate::models::npc::ancestry_enum::Ancestry;
use crate::models::npc::class_enum::Class;
use crate::models::npc::culture_enum::Culture;
use crate::models::npc::gender_enum::Gender;
use crate::models::npc::job_enum::Job;
use crate::models::npc::request_npc_struct::{AncestryData, RandomNameData, RandomNpcData};
use crate::models::response_data::ResponseNpc;
use crate::models::routers_validator_structs::LevelData;
use crate::services::npc_service;
use actix_web::error::ErrorBadRequest;
use actix_web::{Responder, get, post, web};
use utoipa::OpenApi;

pub fn init_endpoints(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/npc")
            .service(get_random_npc)
            .service(get_random_ancestry)
            .service(get_random_culture)
            .service(get_random_class)
            .service(get_random_gender)
            .service(get_random_job)
            .service(get_random_names)
            .service(get_random_nickname)
            .service(get_random_level)
            .service(get_npc_classes_list)
            .service(get_npc_genders_list)
            .service(get_npc_jobs_list)
            .service(get_npc_ancestries_list)
            .service(get_npc_cultures_list),
    );
}

pub fn init_docs(doc: &mut utoipa::openapi::OpenApi) {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            get_random_npc,
            get_random_ancestry,
            get_random_culture,
            get_random_class,
            get_random_gender,
            get_random_job,
            get_random_names,
            get_random_nickname,
            get_random_level,
            get_npc_classes_list,
            get_npc_genders_list,
            get_npc_jobs_list,
            get_npc_ancestries_list,
            get_npc_cultures_list
        ),
        components(schemas(ResponseNpc, RandomNpcData, RandomNameData, AncestryData))
    )]
    struct ApiDoc;

    doc.merge(ApiDoc::openapi());
}

#[utoipa::path(
    post,
    path = "/npc/generator",
    tag = "npc",
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
pub async fn get_random_npc(
    data: web::Data<AppState>,
    body: Option<web::Json<RandomNpcData>>,
) -> actix_web::Result<impl Responder> {
    let npc_data = body.map(|x| x.0).unwrap_or_default();
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
    tag = "npc",
    request_body(
        content = Option<Vec<Class>>,
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
pub async fn get_random_class(
    body: Option<web::Json<Vec<Class>>>,
) -> actix_web::Result<impl Responder> {
    Ok(web::Json(npc_service::get_random_class(
        if let Some(body) = body {
            Some(body.0)
        } else {
            None
        },
    )))
}

#[utoipa::path(
    post,
    path = "/npc/generator/level",
    tag = "npc",
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
pub async fn get_random_level(
    body: Option<web::Json<LevelData>>,
) -> actix_web::Result<impl Responder> {
    if let Some(json) = &body {
        if !json.0.is_data_valid() {
            return Err(ErrorBadRequest(
                "Given parameters are not valid. Check for conflicts e.g. min lvl > max lvl",
            ));
        }
    }
    Ok(web::Json(npc_service::get_random_level(body.map(|x| x.0))))
}

#[utoipa::path(
    post,
    path = "/npc/generator/ancestry",
    tag = "npc",
    request_body(
        content = Option<Vec<Ancestry>>,
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
pub async fn get_random_ancestry(
    body: Option<web::Json<Vec<Ancestry>>>,
) -> actix_web::Result<impl Responder> {
    Ok(web::Json(npc_service::get_random_ancestry(
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
    tag = "npc",
    request_body(
        content = Option<Vec<Culture>>,
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
pub async fn get_random_culture(
    body: Option<web::Json<Vec<Culture>>>,
) -> actix_web::Result<impl Responder> {
    Ok(web::Json(npc_service::get_random_culture(
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
    tag = "npc",
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
pub async fn get_random_gender(
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
    tag = "npc",
    request_body(
        content = Option<Vec<Job>>,
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
pub async fn get_random_job(
    body: Option<web::Json<Vec<Job>>>,
) -> actix_web::Result<impl Responder> {
    Ok(web::Json(npc_service::get_random_job(
        if let Some(body) = body {
            Some(body.0)
        } else {
            None
        },
    )))
}

#[utoipa::path(
    post,
    path = "/npc/generator/nickname",
    tag = "npc",
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
pub async fn get_random_nickname(data: web::Data<AppState>) -> actix_web::Result<impl Responder> {
    Ok(web::Json(npc_service::generate_random_nickname(
        &data.nick_json_path,
    )))
}

#[utoipa::path(
    post,
    path = "/npc/generator/names",
    tag = "npc",
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
pub async fn get_random_names(
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
            RandomNameData::default(),
            &data.name_json_path,
        )))
    }
}

#[utoipa::path(
    get,
    path = "/npc/ancestries",
    tag = "npc",
    params(

    ),
    responses(
        (status=200, description = "Successful Response", body = [AncestryData]),
        (status=400, description = "Bad request.")
    ),
)]
#[get("/ancestries")]
pub async fn get_npc_ancestries_list() -> actix_web::Result<impl Responder> {
    Ok(web::Json(npc_service::get_ancestries_list()))
}

#[utoipa::path(
    get,
    path = "/npc/cultures",
    tag = "npc",
    params(

    ),
    responses(
        (status=200, description = "Successful Response", body = [String]),
        (status=400, description = "Bad request.")
    ),
)]
#[get("/cultures")]
pub async fn get_npc_cultures_list() -> actix_web::Result<impl Responder> {
    Ok(web::Json(npc_service::get_cultures_list()))
}

#[utoipa::path(
    get,
    path = "/npc/genders",
    tag = "npc",
    params(

    ),
    responses(
        (status=200, description = "Successful Response", body = [String]),
        (status=400, description = "Bad request.")
    ),
)]
#[get("/genders")]
pub async fn get_npc_genders_list() -> actix_web::Result<impl Responder> {
    Ok(web::Json(npc_service::get_genders_list()))
}

#[utoipa::path(
    get,
    path = "/npc/jobs",
    tag = "npc",
    params(

    ),
    responses(
        (status=200, description = "Successful Response", body = [String]),
        (status=400, description = "Bad request.")
    ),
)]
#[get("/jobs")]
pub async fn get_npc_jobs_list() -> actix_web::Result<impl Responder> {
    Ok(web::Json(npc_service::get_jobs_list()))
}

#[utoipa::path(
    get,
    path = "/npc/classes",
    tag = "npc",
    params(

    ),
    responses(
        (status=200, description = "Successful Response", body = [String]),
        (status=400, description = "Bad request.")
    ),
)]
#[get("/classes")]
pub async fn get_npc_classes_list() -> actix_web::Result<impl Responder> {
    Ok(web::Json(npc_service::get_classes_list()))
}
