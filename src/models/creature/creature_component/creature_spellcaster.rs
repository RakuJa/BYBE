use crate::models::creature::creature_metadata::variant_enum::CreatureVariant;
use crate::models::creature::items::spellcaster_entry::{SpellcasterData, SpellcasterEntry};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq)]
pub struct CreatureSpellcasterData {
    pub spellcaster_entries: Vec<SpellcasterEntry>,
}

impl CreatureSpellcasterData {
    pub fn get_total_n_of_spells(&self) -> usize {
        self.spellcaster_entries
            .iter()
            .map(|sce| sce.spells.len())
            .collect::<Vec<_>>()
            .iter()
            .sum()
    }
    pub fn get_highest_spell_dc_mod(&self) -> Option<i64> {
        self.spellcaster_entries
            .iter()
            .map(|x| x.spellcaster_data.spellcasting_dc_mod)
            .max()
    }
    pub fn add_mod_to_spellcaster_atk_and_dc(self, modifier: i64) -> Self {
        Self {
            spellcaster_entries: self
                .spellcaster_entries
                .into_iter()
                .map(|entry| {
                    let sce = entry.spellcaster_data;
                    SpellcasterEntry {
                        spellcaster_data: SpellcasterData {
                            id: sce.id,
                            spellcasting_name: sce.spellcasting_name.clone(),
                            is_spellcasting_flexible: sce.is_spellcasting_flexible,
                            type_of_spellcaster: sce.type_of_spellcaster.clone(),
                            spellcasting_dc_mod: sce.spellcasting_dc_mod + modifier,
                            spellcasting_atk_mod: sce.spellcasting_atk_mod + modifier,
                            spellcasting_tradition: sce.spellcasting_tradition,
                            heighten_level: sce.heighten_level,
                        },
                        spells: entry.spells,
                    }
                })
                .collect(),
        }
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
