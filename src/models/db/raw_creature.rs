use crate::models::creature_metadata::rarity_enum::RarityEnum;
use crate::models::creature_metadata::size_enum::SizeEnum;
use crate::models::creature_metadata::type_enum::CreatureTypeEnum;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct RawCreature {
    pub id: i64,
    pub aon_id: Option<i64>,
    pub name: String,
    pub charisma: i64,
    pub constitution: i64,
    pub dexterity: i64,
    pub intelligence: i64,
    pub strength: i64,
    pub wisdom: i64,
    pub ac: i64,
    pub hp: i64,
    pub hp_detail: String,
    pub ac_detail: String,
    pub language_detail: Option<String>,
    pub level: i64,
    pub license: String,
    pub remaster: bool,
    pub source: String,
    pub initiative_ability: String,
    pub perception: i64,
    pub perception_detail: String,
    pub fortitude: i64,
    pub reflex: i64,
    pub will: i64,
    pub fortitude_detail: String,
    pub reflex_detail: String,
    pub will_detail: String,
    pub rarity: RarityEnum,
    pub size: SizeEnum,
    pub cr_type: CreatureTypeEnum,
    pub family: Option<String>,

    pub spell_casting_name: Option<String>,
    pub is_spell_casting_flexible: Option<bool>,
    pub type_of_spell_caster: Option<String>,
    pub spell_casting_dc_mod: Option<i64>,
    pub spell_casting_atk_mod: Option<i64>,
    pub spell_casting_tradition: Option<String>,
}
