use crate::models::db::raw_creature::RawCreature;
use crate::models::items::spell::Spell;
use crate::models::items::spell_caster_entry::SpellCasterEntry;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq)]
pub struct CreatureSpellCasterData {
    pub spells: Vec<Spell>,
    pub spell_caster_entry: SpellCasterEntry,
}

impl From<(RawCreature, Vec<Spell>)> for CreatureSpellCasterData {
    fn from(tuple: (RawCreature, Vec<Spell>)) -> Self {
        let raw_cr = tuple.0;
        Self {
            spells: tuple.1,
            spell_caster_entry: SpellCasterEntry {
                spell_casting_name: raw_cr.spell_casting_name,
                is_spell_casting_flexible: raw_cr.is_spell_casting_flexible,
                type_of_spell_caster: raw_cr.type_of_spell_caster,
                spell_casting_dc_mod: raw_cr.spell_casting_dc_mod.map(|x| x as i8),
                spell_casting_atk_mod: raw_cr.spell_casting_atk_mod.map(|x| x as i8),
                spell_casting_tradition: raw_cr.spell_casting_tradition,
            },
        }
    }
}
