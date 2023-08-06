#[macro_use]
extern crate maplit;
extern crate lazy_static;
extern crate tokio;

mod routes;

use crate::routes::{bestiary, health};
use actix_web::{get, middleware, App, HttpResponse, HttpServer, Responder};
use std::env;

mod db;
mod models;
mod services;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

fn get_service_ip() -> String {
    env::var("SERVICE_IP").unwrap_or_else(|_| "0.0.0.0".to_string())
}

fn get_service_port() -> u16 {
    match env::var("SERVICE_PORT").ok() {
        None => 25566,
        Some(port) => port.parse::<u16>().unwrap_or(25566),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let service_ip = get_service_ip();
    let service_port = get_service_port();
    log::info!(
        "starting HTTP server at http://{}:{}",
        service_ip.as_str(),
        service_port.to_string()
    );
    // async running in the background
    tokio::task::spawn(db::db_proxy::update_cache());

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(index)
            .configure(health::init_endpoints)
            .configure(bestiary::init_endpoints)
    })
    .bind((get_service_ip(), get_service_port()))?
    .run()
    .await
}
