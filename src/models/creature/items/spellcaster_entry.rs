use crate::models::creature::items::spell::Spell;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq, Debug)]
pub struct SpellcasterData {
    pub id: i64,
    pub spellcasting_name: String,
    pub is_spellcasting_flexible: Option<bool>,
    pub type_of_spellcaster: String,
    #[schema(example = 10)]
    pub spellcasting_dc_mod: i64,
    #[schema(example = 10)]
    pub spellcasting_atk_mod: i64,
    pub spellcasting_tradition: String,
    pub heighten_level: i64,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq, Debug)]
pub struct SpellcasterEntry {
    pub spellcaster_data: SpellcasterData,
    pub spells: Vec<Spell>,
}
