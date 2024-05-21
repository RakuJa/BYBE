use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq)]
pub struct Spell {
    pub id: i64,
    pub name: String,
    pub area_type: Option<String>,
    pub area_value: Option<i64>,
    pub counteraction: bool,

    pub saving_throw_is_basic: Option<bool>,
    pub saving_throw_statistic: Option<String>,
    pub sustained: bool,

    pub duration: Option<String>,

    pub level: i64,
    pub range: String,
    pub target: String,
    pub action: String,

    pub license: String,
    pub remaster: bool,
    pub source: String,
    pub rarity: String, // use rarityenum
    pub creature_id: i64,
}
