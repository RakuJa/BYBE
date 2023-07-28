#[macro_use]
extern crate maplit;
extern crate lazy_static;

mod routes;

use actix_web::{get, middleware, App, HttpResponse, HttpServer, Responder};
use std::env;

mod db;
mod models;
mod services;
use crate::routes::bestiary::{get_bestiary, get_creature, get_keys};
use crate::routes::health::get_health;

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
        Some(port) => match port.parse::<u16>() {
            Err(_) => 25566,
            Ok(n) => n,
        },
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

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(index)
            .service(get_health)
            .service(get_bestiary)
            .service(get_creature)
            .service(get_keys)
    })
    .bind((get_service_ip(), get_service_port()))?
    .run()
    .await
}
