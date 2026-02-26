use crate::models::shared::rarity_enum::RarityEnum;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq, Debug)]
pub struct Action {
    pub id: i64,
    pub name: String,
    pub action_type: String,
    #[schema(example = 1)]
    pub n_of_actions: Option<i64>,
    pub category: Option<String>,
    pub description: String,

    pub license: String,
    pub remaster: bool,
    pub source: String,

    pub slug: Option<String>,
    pub rarity: RarityEnum,
}
