#[macro_use]
extern crate maplit;

mod db;
mod models;
mod routes;
mod services;

mod traits;

use crate::routes::{bestiary, encounter, health, npc, shop};
use actix_cors::Cors;
use actix_web::http::header::{CacheControl, CacheDirective};
use actix_web::{App, HttpResponse, HttpServer, Responder, get, middleware, web};
use dotenvy::{dotenv, from_path};
use sqlx::{Pool, Sqlite, sqlite::SqlitePoolOptions};
use std::env;
use std::num::NonZero;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(paths(index))]
struct ApiDoc;

#[derive(Clone)]
pub struct AppState {
    conn: Pool<Sqlite>,
    name_json_path: String,
    nick_json_path: String,
}

#[derive(Default)]
pub enum StartupState {
    #[default]
    Clean,
    Persistent,
}

#[derive(Default)]
pub enum InitializeLogResponsibility {
    Delegated,
    #[default]
    Personal,
}

impl From<String> for StartupState {
    fn from(value: String) -> Self {
        match value.to_uppercase().as_str() {
            "CLEAN" => Self::Clean,
            "PERSISTENT" => Self::Persistent,
            _ => Self::default(),
        }
    }
}

#[utoipa::path(get, path = "/")]
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

fn get_service_db_url() -> String {
    env::var("DATABASE_URL").expect("Error fetching database URL")
}

fn get_nickname_json_path() -> String {
    env::var("NICKNAMES_PATH").expect("Error fetching nickname json")
}

fn get_name_json_path() -> String {
    env::var("NAMES_PATH").expect("Error fetching name json")
}

fn get_service_ip() -> String {
    env::var("SERVICE_IP").unwrap_or_else(|_| "0.0.0.0".to_string())
}

fn get_service_startup_state() -> StartupState {
    env::var("SERVICE_STARTUP_STATE").unwrap_or_default().into()
}

fn get_service_port() -> u16 {
    env::var("SERVICE_PORT")
        .ok()
        .map_or(25566, |port| port.parse().unwrap_or(25566))
}

fn get_service_workers() -> usize {
    let available_cpus =
        usize::from(std::thread::available_parallelism().unwrap_or(NonZero::new(1).unwrap()));
    env::var("N_OF_SERVICE_WORKERS")
        .ok()
        .map_or(available_cpus, |n_of_workers| {
            n_of_workers.parse().unwrap_or(available_cpus)
        })
}

fn init_docs(openapi: &mut utoipa::openapi::OpenApi) {
    health::init_docs(openapi);
    bestiary::init_docs(openapi);
    encounter::init_docs(openapi);
    shop::init_docs(openapi);
    npc::init_docs(openapi);
}

#[actix_web::main]
pub async fn start(
    env_location: Option<String>,
    db_location: Option<String>,
    jsons_location: Option<(String, String)>,
    init_log_resp: InitializeLogResponsibility,
) -> std::io::Result<()> {
    if let Some(env_path) = env_location {
        from_path(env_path).ok();
    } else {
        dotenv().ok();
    }
    match init_log_resp {
        InitializeLogResponsibility::Personal => {
            env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
        }
        InitializeLogResponsibility::Delegated => {} // do nothing, someone else has already initialized them
    }
    let db_url = db_location.map_or_else(get_service_db_url, |x| x);
    let (name_json_path, nick_json_path) =
        jsons_location.unwrap_or_else(|| (get_name_json_path(), get_nickname_json_path()));
    let service_ip = get_service_ip();
    let service_port = get_service_port();
    let startup_state: StartupState = get_service_startup_state();
    let service_workers = get_service_workers();

    log::info!("Starting DB connection");

    // establish connection to database
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Error building connection pool");
    match startup_state {
        StartupState::Clean => {
            log::info!("Starting DB Table cleanup & creation of update CORE tables");
            db::cr_core_initializer::update_creature_core_table(&pool)
                .await
                .expect("Could not initialize correctly core creature table.. Startup failed");
        }
        StartupState::Persistent => {}
    }

    log::info!(
        "starting HTTP server at http://{}:{}",
        service_ip.as_str(),
        service_port
    );

    // Swagger initialization
    let mut openapi = ApiDoc::openapi();
    init_docs(&mut openapi);

    // Configure endpoints
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .wrap(middleware::Logger::default())
            .wrap(
                middleware::DefaultHeaders::new()
                    // Cache header
                    .add(CacheControl(vec![
                        CacheDirective::Private,
                        CacheDirective::MaxAge(86400u32),
                    ]))
                    // Do not infer mime type header
                    .add(("X-Content-Type-Options", "nosniff")),
            )
            .service(index)
            .configure(health::init_endpoints)
            .configure(bestiary::init_endpoints)
            .configure(encounter::init_endpoints)
            .configure(shop::init_endpoints)
            .configure(npc::init_endpoints)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            .service(SwaggerUi::new("/docs/{_:.*}").url("/api-docs/openapi.json", openapi.clone()))
            .app_data(web::Data::new(AppState {
                conn: pool.clone(),
                name_json_path: name_json_path.clone(),
                nick_json_path: nick_json_path.clone(),
            }))
    })
    .workers(service_workers)
    .bind((get_service_ip(), get_service_port()))?;
    server.run().await
}
