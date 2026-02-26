use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, PartialEq, ToSchema, Eq, Hash, Clone, Debug)]
pub struct Resistance {
    pub core: CoreResistanceData,
    pub double_vs: Vec<String>,
    pub exception_vs: Vec<String>,
}

#[derive(Serialize, Deserialize, FromRow, ToSchema, PartialEq, Eq, Hash, Clone, Debug)]
pub struct CoreResistanceData {
    pub id: i64,
    pub name: String,
    pub value: i64,
}
