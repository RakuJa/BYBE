use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, FromRow, ToSchema, Clone, Eq, Hash, PartialEq, Debug)]
pub struct Sense {
    pub id: i64,
    pub name: String,
    pub range: Option<i64>,
    pub acuity: Option<String>,
}
