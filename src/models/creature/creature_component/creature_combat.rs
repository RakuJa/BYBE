use crate::models::item::armor_struct::Armor;
use crate::models::item::shield_struct::Shield;
use crate::models::item::weapon_struct::Weapon;
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
                    wp.weapon_data.to_hit_bonus.map(|to_hit| to_hit - modifier);
                wp
            })
            .collect();
        com_data.ac = (com_data.ac as i64 - modifier) as i8;
        com_data.saving_throws.fortitude -= modifier;
        com_data.saving_throws.reflex -= modifier;
        com_data.saving_throws.will -= modifier;
        com_data.weapons = weapons;
        com_data
    }

    /// Lowers saving throws, weapon to hit bonus, and ac by the given pwl_mod
    pub fn convert_from_base_to_pwl(self, pwl_mod: u64) -> CreatureCombatData {
        self.add_mod_to_saving_throws_and_ac_and_wp_to_hit(-(pwl_mod as i64))
    }
}
