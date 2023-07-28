use crate::models::enums::{AlignmentEnum, RarityEnum, SizeEnum};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Creature {
    pub id: String,
    pub name: String,
    pub hp: i16,
    pub level: i16,
    pub alignment: AlignmentEnum,
    pub size: SizeEnum,
    pub family: Option<String>,
    pub rarity: RarityEnum,
    pub is_melee: bool,
    pub is_ranged: bool,
    pub is_spell_caster: bool,
    pub source: Vec<String>,
}
