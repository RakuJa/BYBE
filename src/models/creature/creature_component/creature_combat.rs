use crate::models::item::armor_struct::Armor;
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
    pub resistances: BTreeMap<String, i16>,
    pub immunities: Vec<String>,
    pub weaknesses: BTreeMap<String, i16>,
    pub saving_throws: SavingThrows,
    pub ac: i8,
}
