use crate::models::creature_metadata::alignment_enum::AlignmentEnum;
use crate::models::creature_metadata::rarity_enum::RarityEnum;
use crate::models::creature_metadata::size_enum::SizeEnum;
use crate::models::creature_metadata::type_enum::CreatureTypeEnum;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq, FromRow)]
pub struct CreatureCoreData {
    // If they ever do a valid flatten for sqlx, derive it for nested struct
    pub essential: EssentialData,
    pub derived: DerivedData,
    pub traits: Vec<String>,
    pub alignment: AlignmentEnum,
}
#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq, FromRow)]
pub struct EssentialData {
    pub id: i64,
    pub aon_id: Option<i64>,
    pub name: String,
    pub hp: i64,
    pub level: i64,
    pub size: SizeEnum,
    pub family: String,
    pub rarity: RarityEnum,
    pub license: String,
    pub remaster: bool,
    pub source: String,
    pub cr_type: CreatureTypeEnum,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq, FromRow)]
pub struct DerivedData {
    pub archive_link: Option<String>,

    pub is_melee: bool,
    pub is_ranged: bool,
    pub is_spell_caster: bool,

    pub brute_percentage: i64,
    pub magical_striker_percentage: i64,
    pub skill_paragon_percentage: i64,
    pub skirmisher_percentage: i64,
    pub sniper_percentage: i64,
    pub soldier_percentage: i64,
    pub spell_caster_percentage: i64,
}
