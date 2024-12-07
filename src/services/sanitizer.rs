use actix_web::error;

pub fn sanitize_id(creature_id: &str) -> actix_web::Result<i64> {
    let id = creature_id.parse::<i64>();
    match id {
        Ok(s) => Ok(s),
        Err(e) => Err(error::ErrorNotFound(e)),
    }
}
