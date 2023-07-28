use crate::services::bestiary_service;
use actix_web::{get, web, Responder, Result};

#[get("/bestiary/{creature_id}")]
pub async fn get_creature(creature_id: web::Path<String>) -> Result<impl Responder> {
    Ok(web::Json(
        bestiary_service::get_creature(&creature_id).await,
    ))
}

#[get("/bestiary/")]
pub async fn get_bestiary() -> Result<impl Responder> {
    Ok(web::Json(
        bestiary_service::get_bestiary(vec!["1".to_string(), "2".to_string()]).await,
    ))
}

#[get("/keys")]
pub async fn get_keys() -> Result<impl Responder> {
    Ok(web::Json(
        bestiary_service::get_keys().await
    ))
}
