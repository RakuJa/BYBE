use sqlx::postgres::PgRow;
use sqlx::{Error, Row};

pub fn get_i32_as_i64(row: &PgRow, col: &str) -> Result<i64, Error> {
    Ok(row.try_get::<i32, _>(col)? as i64)
}

pub fn get_opt_i32_as_i64(row: &PgRow, col: &str) -> Option<i64> {
    row.try_get::<Option<i32>, _>(col)
        .ok()
        .flatten()
        .map(|v| v as i64)
}

pub fn get_opt_i32_as_i16(row: &PgRow, col: &str) -> Option<i16> {
    row.try_get::<Option<i32>, _>(col)
        .ok()
        .flatten()
        .map(|v| v as i16)
}
