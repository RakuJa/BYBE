use crate::models::creature::items::spell::Spell;
use crate::models::creature::items::spell_caster_entry::SpellCasterEntry;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq)]
pub struct CreatureSpellCasterData {
    pub spells: Vec<Spell>,
    pub spell_caster_entry: SpellCasterEntry,
}
