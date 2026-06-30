use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq, Debug, FromRow)]
pub struct ConditionData {
    pub name: String,
    pub rule: String,
    pub note: Option<String>,
    pub summary: Option<String>,
    pub license: String,
    pub remaster: bool,
    pub source: String,
    pub is_perpetual: bool,
    pub is_stackable: bool,
    pub condition_group: Option<String>,
}
