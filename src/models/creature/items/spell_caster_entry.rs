use crate::models::creature::items::spell::Spell;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq)]
pub struct SpellcasterData {
    pub id: i64,
    pub spell_casting_name: String,
    pub is_spell_casting_flexible: Option<bool>,
    pub type_of_spell_caster: String,
    #[schema(example = 10)]
    pub spell_casting_dc_mod: Option<i64>,
    #[schema(example = 10)]
    pub spell_casting_atk_mod: Option<i64>,
    pub spell_casting_tradition: String,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq)]
pub struct SpellcasterEntry {
    pub spellcaster_data: SpellcasterData,
    pub spells: BTreeMap<i64, Vec<Spell>>,
}
