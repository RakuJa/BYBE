use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, PartialEq, Eq, Hash, Clone)]
pub struct RawResistance {
    pub name: String,
    pub value: i64,
}
