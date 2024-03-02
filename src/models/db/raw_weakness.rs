use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, PartialEq, Eq, Hash, Clone)]
pub struct RawWeakness {
    pub name: String,
    pub value: i64,
}
