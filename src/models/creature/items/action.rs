use crate::models::creature::creature_metadata::rarity_enum::RarityEnum;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq)]
pub struct Action {
    pub id: i64,
    pub name: String,
    pub action_type: String,
    pub n_of_actions: Option<i64>,
    pub category: Option<String>,
    pub description: String,

    pub license: String,
    pub remaster: bool,
    pub source: String,

    pub slug: Option<String>,
    pub rarity: RarityEnum,
    pub creature_id: i64,
}
