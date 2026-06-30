use crate::models::shared::range_data::RangeData;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::{Error, FromRow, Row};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Clone, Eq, Hash, PartialEq, Debug)]
pub struct Sense {
    pub id: i64,
    pub name: String,
    pub range: Option<RangeData>,
    pub acuity: Option<String>,
}

impl<'r> FromRow<'r, PgRow> for Sense {
    fn from_row(row: &'r PgRow) -> Result<Self, Error> {
        Ok(Self {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            acuity: row.try_get("acuity")?,
            range: RangeData::from_row(row).ok(),
        })
    }
}
