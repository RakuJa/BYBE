use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, FromRow, PartialEq, Eq, Hash, Clone, Debug, ToSchema)]
pub struct Weakness {
    pub id: i64,
    pub name: String,
    #[sqlx(try_from = "i32")]
    pub value: i64,
}
