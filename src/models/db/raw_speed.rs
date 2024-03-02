use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, PartialEq, Eq, Hash, Clone)]
pub struct RawSpeed {
    pub name: String,
    pub value: i64,
}
