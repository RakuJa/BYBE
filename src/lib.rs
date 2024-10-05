#[macro_use]
extern crate maplit;

mod routes;

use crate::routes::{bestiary, encounter, health, shop};
use actix_cors::Cors;
use actix_web::http::header::{CacheControl, CacheDirective};
use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use std::env;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod db;
mod models;
mod services;

#[derive(Clone)]
pub struct AppState {
    conn: Pool<Sqlite>,
}

#[derive(Default)]
pub enum StartupState {
    #[default]
    Clean,
    Persistent,
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

fn get_service_ip() -> String {
    env::var("SERVICE_IP").unwrap_or_else(|_| "0.0.0.0".to_string())
}

fn get_service_startup_state() -> StartupState {
    env::var("SERVICE_STARTUP_STATE").unwrap_or_default().into()
}

fn get_service_port() -> u16 {
    match env::var("SERVICE_PORT").ok() {
        None => 25566,
        Some(port) => port.parse::<u16>().unwrap_or(25566),
    }
}

fn init_docs(openapi: &mut utoipa::openapi::OpenApi) {
    health::init_docs(openapi);
    bestiary::init_docs(openapi);
    encounter::init_docs(openapi);
    shop::init_docs(openapi);
}

#[actix_web::main]
pub async fn start() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    // get env vars
    let db_url = get_service_db_url();
    let service_ip = get_service_ip();
    let service_port = get_service_port();
    let startup_state: StartupState = get_service_startup_state();

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
        service_port.to_string()
    );

    // Swagger initialization
    #[derive(OpenApi)]
    #[openapi(paths(index))]
    struct ApiDoc;

    let mut openapi = ApiDoc::openapi();
    init_docs(&mut openapi);

    // Configure endpoints
    HttpServer::new(move || {
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
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            .app_data(web::Data::new(AppState { conn: pool.clone() }))
    })
    .bind((get_service_ip(), get_service_port()))?
    .run()
    .await
}
