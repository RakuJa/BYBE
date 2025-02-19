use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq)]
pub struct Spell {
    pub id: i64,
    pub name: String,
    pub area_type: Option<String>,
    #[schema(example = 5)]
    pub area_value: Option<i64>,
    pub counteraction: bool,

    pub saving_throw_is_basic: Option<bool>,
    pub saving_throw_statistic: Option<String>,
    pub sustained: bool,

    pub duration: Option<String>,
    #[schema(example = 1)]
    pub level: i64,
    pub range: String,
    pub target: String,
    pub action: String,

    pub license: String,
    pub remaster: bool,
    pub source: String,
    pub rarity: String, // use rarityenum

    pub slot: i64,
    pub creature_id: i64,
    pub spellcasting_entry_id: i64,
}
