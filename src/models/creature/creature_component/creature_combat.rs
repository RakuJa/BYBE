use crate::models::creature::creature_metadata::variant_enum::CreatureVariant;
use crate::models::item::armor_struct::Armor;
use crate::models::item::shield_struct::Shield;
use crate::models::item::weapon_struct::{DamageData, Weapon};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq)]
pub struct SavingThrows {
    pub fortitude: i64,
    pub reflex: i64,
    pub will: i64,
    pub fortitude_detail: Option<String>,
    pub reflex_detail: Option<String>,
    pub will_detail: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq)]
pub struct CreatureCombatData {
    pub weapons: Vec<Weapon>,
    pub armors: Vec<Armor>,
    pub shields: Vec<Shield>,
    pub resistances: BTreeMap<String, i16>,
    pub immunities: Vec<String>,
    pub weaknesses: BTreeMap<String, i16>,
    pub saving_throws: SavingThrows,
    pub ac: i8,
}

impl CreatureCombatData {
    fn add_mod_to_saving_throws_and_ac_and_wp_to_hit(self, modifier: i64) -> CreatureCombatData {
        let mut com_data = self;
        let weapons: Vec<Weapon> = com_data
            .weapons
            .into_iter()
            .map(|mut wp| {
                wp.weapon_data.to_hit_bonus =
                    wp.weapon_data.to_hit_bonus.map(|to_hit| to_hit + modifier);
                wp
            })
            .collect();
        com_data.ac = (com_data.ac as i64 + modifier) as i8;
        com_data.saving_throws.fortitude += modifier;
        com_data.saving_throws.reflex += modifier;
        com_data.saving_throws.will += modifier;
        com_data.weapons = weapons;
        com_data
    }

    fn add_mod_to_dmg(self, modifier: i64) -> CreatureCombatData {
        let mut com_data = self;
        let weapons: Vec<Weapon> = com_data
            .weapons
            .into_iter()
            .map(|mut wp| {
                wp.weapon_data.splash_dmg = wp.weapon_data.splash_dmg.map(|dmg| dmg + modifier);
                wp.weapon_data.damage_data = wp
                    .weapon_data
                    .damage_data
                    .iter()
                    .map(|x| DamageData {
                        id: x.id,
                        bonus_dmg: x.bonus_dmg + modifier,
                        dmg_type: x.dmg_type.clone(),
                        dice: x.dice.clone(),
                    })
                    .collect();
                wp
            })
            .collect();
        com_data.weapons = weapons;
        com_data
    }

    /// Lowers saving throws, weapon to hit bonus, and ac by the given pwl_mod
    pub fn convert_from_base_to_pwl(self, pwl_mod: u64) -> CreatureCombatData {
        self.add_mod_to_saving_throws_and_ac_and_wp_to_hit(-(pwl_mod as i64))
    }

    /// Increase/Decrease the damage of its Strikes and other offensive abilities by 2.
    /// If the creature has limits on how many times or how often it can use an ability
    /// (such as a spellcaster’s spells or a dragon’s breath), decrease the damage by 4 instead.
    /// Increase/Decrease the creature’s AC, attack modifiers, DCs, saving throws by 2.
    pub fn convert_from_base_to_variant(self, variant: &CreatureVariant) -> CreatureCombatData {
        let modifier = variant.to_adjustment_modifier();
        self.add_mod_to_saving_throws_and_ac_and_wp_to_hit(modifier)
            .add_mod_to_dmg(modifier)
    }
}
