use crate::models::shared::rarity_enum::RarityEnum;
use crate::models::shared::trait_data::TraitData;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq, Debug, FromRow)]
pub struct CoreAction {
    pub id: i64,
    pub name: String,
    pub action_type: String,
    #[schema(example = 1)]
    pub n_of_actions: Option<i32>,
    pub category: Option<String>,
    pub description: String,

    pub license: String,
    pub remaster: bool,
    pub source: String,

    pub slug: Option<String>,
    #[sqlx(try_from = "String")]
    pub rarity: RarityEnum,
}

#[derive(Serialize, Deserialize, Clone, Eq, Hash, PartialEq, Debug, ToSchema)]
pub struct Action {
    pub core_action: CoreAction,
    pub traits: Vec<TraitData>,
}
