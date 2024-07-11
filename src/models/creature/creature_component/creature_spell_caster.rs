use crate::models::creature::items::spell::Spell;
use crate::models::creature::items::spell_caster_entry::SpellCasterEntry;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq)]
pub struct CreatureSpellCasterData {
    pub spells: Vec<Spell>,
    pub spell_caster_entry: SpellCasterEntry,
}

impl CreatureSpellCasterData {
    pub fn add_mod_to_spellcaster_atk_and_dc(self, pwl_mod: i64) -> CreatureSpellCasterData {
        let mut spell_data = self;

        spell_data.spell_caster_entry.spell_casting_atk_mod = spell_data
            .spell_caster_entry
            .spell_casting_atk_mod
            .map(|x| x - pwl_mod);

        spell_data.spell_caster_entry.spell_casting_dc_mod = spell_data
            .spell_caster_entry
            .spell_casting_dc_mod
            .map(|x| x - pwl_mod);

        spell_data.clone()
    }

    /// Lowers spell caster atk and dc
    pub fn convert_from_base_to_pwl(self, pwl_mod: u64) -> CreatureSpellCasterData {
        self.add_mod_to_spellcaster_atk_and_dc(-(pwl_mod as i64))
    }
}
