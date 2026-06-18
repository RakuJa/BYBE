use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq, Debug, FromRow)]
pub struct TraitData {
    pub name: String,
    pub description: Option<String>,
    pub display_name: Option<String>,
}
