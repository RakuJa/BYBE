use crate::models::creature::creature_metadata::variant_enum::CreatureVariant;
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
    pub fn add_mod_to_spellcaster_atk_and_dc(self, modifier: i64) -> Self {
        let mut spell_data = self;

        spell_data.spell_caster_entry.spell_casting_atk_mod = spell_data
            .spell_caster_entry
            .spell_casting_atk_mod
            .map(|x| x + modifier);

        spell_data.spell_caster_entry.spell_casting_dc_mod = spell_data
            .spell_caster_entry
            .spell_casting_dc_mod
            .map(|x| x + modifier);

        spell_data
    }

    /// Lowers spell caster atk and dc
    pub fn convert_from_base_to_pwl(self, pwl_mod: u64) -> Self {
        self.add_mod_to_spellcaster_atk_and_dc(-i64::try_from(pwl_mod).unwrap_or(i64::MAX))
    }

    /// Increase/Decrease the damage of its Strikes and other offensive abilities by 2.
    /// If the creature has limits on how many times or how often it can use an ability
    /// (such as a spellcaster’s spells or a dragon’s breath), decrease the damage by 4 instead.
    pub fn convert_from_base_to_variant(self, variant: CreatureVariant) -> Self {
        self.add_mod_to_spellcaster_atk_and_dc(variant.to_adjustment_modifier())
    }
}
