use crate::models::creature::creature_metadata::variant_enum::CreatureVariant;
use crate::models::creature::items::spellcaster_entry::{SpellcasterData, SpellcasterEntry};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq, Debug)]
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

    fn add_mod_to_spell_dmg(self, variant: CreatureVariant) -> Self {
        Self {
            spellcaster_entries: self
                .spellcaster_entries
                .into_iter()
                .map(|entry| SpellcasterEntry {
                    spellcaster_data: entry.spellcaster_data,
                    spells: entry
                        .spells
                        .into_iter()
                        .map(|spell| spell.convert_from_base_to_variant(variant))
                        .collect(),
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
        self.add_mod_to_spellcaster_atk_and_dc(variant.to_dmg_adjustment_modifier(false))
            .add_mod_to_spell_dmg(variant)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::creature::items::spell::{Spell, SpellDamageData};

    fn spellcaster_data() -> SpellcasterData {
        SpellcasterData {
            id: 1,
            spellcasting_name: "Innate".to_string(),
            is_spellcasting_flexible: None,
            type_of_spellcaster: "innate".to_string(),
            spellcasting_dc_mod: 20,
            spellcasting_atk_mod: 10,
            spellcasting_tradition: "arcane".to_string(),
            heighten_level: 1,
        }
    }

    fn spell_with_damage(bonus_dmg: i64) -> Spell {
        Spell {
            id: 1,
            name: "Fireball".to_string(),
            area_type: None,
            area_value: None,
            counteraction: false,
            basic_saving_throw: None,
            saving_throw: None,
            sustained: false,
            duration: None,
            level: 3,
            range: None,
            target: String::new(),
            actions: String::new(),
            license: String::new(),
            remaster: false,
            source: String::new(),
            rarity: "common".to_string(),
            slot: 3,
            creature_id: 1,
            spellcasting_entry_id: 1,
            description: String::default(),
            damage: vec![SpellDamageData {
                id: 1,
                bonus_dmg,
                dmg_type: Some("fire".to_string()),
                category: None,
                kinds: vec!["damage".to_string()],
                dice: None,
            }],
        }
    }

    // Rule 2 (DC/attack modifiers) is always +/-2 regardless of frequency; rule 3 (damage of
    // limited-use abilities, e.g. a spellcaster's spells) is +/-4 instead of the usual +/-2.
    // Both must apply at once when converting a whole spellcasting entry.
    #[test]
    fn convert_from_base_to_variant_applies_two_to_dc_atk_and_four_to_spell_damage() {
        let data = CreatureSpellcasterData {
            spellcaster_entries: vec![SpellcasterEntry {
                spellcaster_data: spellcaster_data(),
                spells: vec![spell_with_damage(2)],
            }],
        };
        let og_dc_mod = data.spellcaster_entries[0]
            .spellcaster_data
            .spellcasting_dc_mod;
        let og_atk_mod = data.spellcaster_entries[0]
            .spellcaster_data
            .spellcasting_atk_mod;
        let og_bonus_dmg = data.spellcaster_entries[0].spells[0].damage[0].bonus_dmg;

        let elite = data
            .clone()
            .convert_from_base_to_variant(CreatureVariant::Elite);
        let entry = &elite.spellcaster_entries[0];
        assert_eq!(og_dc_mod + 2, entry.spellcaster_data.spellcasting_dc_mod);
        assert_eq!(og_atk_mod + 2, entry.spellcaster_data.spellcasting_atk_mod);
        assert_eq!(og_bonus_dmg + 4, entry.spells[0].damage[0].bonus_dmg);

        let weak = data.convert_from_base_to_variant(CreatureVariant::Weak);
        let entry = &weak.spellcaster_entries[0];
        assert_eq!(og_dc_mod - 2, entry.spellcaster_data.spellcasting_dc_mod);
        assert_eq!(og_atk_mod - 2, entry.spellcaster_data.spellcasting_atk_mod);
        assert_eq!(og_bonus_dmg - 4, entry.spells[0].damage[0].bonus_dmg);
    }
}
