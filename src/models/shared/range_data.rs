use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::{Error, FromRow, Row, Type};
use utoipa::ToSchema;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize, ToSchema, Type)]
pub struct RangeData {
    pub id: i64,
    #[schema(example = "30 feet")]
    pub value: String,
    #[schema(example = "30 feet")]
    pub increment: Option<String>,
    #[schema(example = "")]
    pub max: Option<String>,
}

impl<'r> FromRow<'r, PgRow> for RangeData {
    fn from_row(row: &'r PgRow) -> Result<Self, Error> {
        // If range_id is NULL == melee, so no range data
        let id: Option<i64> = row.try_get("range_id")?;
        let id = id.ok_or_else(|| Error::ColumnNotFound("range_id".to_string()))?;

        Ok(Self {
            id,
            value: row.try_get("range_value")?,
            increment: row.try_get("range_increment")?,
            max: row.try_get("range_max")?,
        })
    }
}
