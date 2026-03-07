use crate::models::creature::creature_metadata::creature_role::CreatureRoleEnum;
use crate::models::creature::creature_metadata::type_enum::CreatureTypeEnum;
use crate::models::hazard::hazard_field_filter::HazardComplexityEnum;
use crate::models::shared::alignment_enum::AlignmentEnum;
use crate::models::shared::pf_version_enum::GameSystemVersionEnum;
use crate::models::shared::rarity_enum::RarityEnum;
use crate::models::shared::size_enum::SizeEnum;
use nanorand::Rng;
use nanorand::WyRand;
use serde::{Deserialize, Serialize};
#[allow(unused_imports)] // Used in schema
use serde_json::json;
use std::collections::HashMap;
use strum::EnumCount;
use strum::EnumIter;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreatureEncounterParams {
    #[schema(min_items = 1, example = json!([4,4,4,4]))]
    pub enemy_levels: Vec<i64>,
    pub is_pwl_on: bool,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct HazardEncounterParams {
    pub hazards: Vec<HazardEncounterElement>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct HazardEncounterElement {
    pub complexity: HazardComplexityEnum,
    pub level: i64,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct EncounterParams {
    #[schema(min_items = 1, example = json!([4,4,4,4]))]
    pub party_levels: Vec<i64>,
    pub creatures_params: Option<CreatureEncounterParams>,
    pub hazards_params: Option<HazardEncounterParams>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct RandomEncounterData {
    #[schema(min_items = 1)]
    pub party_levels: Vec<i64>,
    #[schema(minimum = 0, maximum = 100, example = 100)]
    pub creature_percentage: Option<u8>,
    #[schema(minimum = 0, maximum = 100, example = 0)]
    pub hazard_percentage: Option<u8>,
    pub creature_data: Option<RandomCreatureData>,
    pub hazard_data: Option<RandomHazardData>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct RandomCreatureData {
    pub source_filter: Option<Vec<String>>,
    pub trait_whitelist_filter: Option<Vec<String>>,
    pub trait_blacklist_filter: Option<Vec<String>>,
    pub family_filter: Option<Vec<String>>,
    pub rarity_filter: Option<Vec<RarityEnum>>,
    pub size_filter: Option<Vec<SizeEnum>>,
    pub alignment_filter: Option<Vec<AlignmentEnum>>,
    pub type_filter: Option<Vec<CreatureTypeEnum>>,
    pub role_filter: Option<Vec<CreatureRoleEnum>>,
    pub attack_list: Option<HashMap<String, bool>>,
    pub role_lower_threshold: Option<u8>,
    pub role_upper_threshold: Option<u8>,
    pub challenge: Option<EncounterChallengeEnum>,
    pub adventure_group: Option<AdventureGroupEnum>,
    #[schema(minimum = 1, maximum = 30, example = 1)]
    pub min_creatures: Option<u8>,
    #[schema(minimum = 1, maximum = 30, example = 5)]
    pub max_creatures: Option<u8>,

    pub allow_elite_variants: Option<bool>,
    pub allow_weak_variants: Option<bool>,
    pub is_pwl_on: bool,
    pub game_system_version: Option<GameSystemVersionEnum>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct RandomHazardData {
    pub source_filter: Option<Vec<String>>,
    pub trait_whitelist_filter: Option<Vec<String>>,
    pub trait_blacklist_filter: Option<Vec<String>>,
    pub rarity_filter: Option<Vec<RarityEnum>>,
    pub size_filter: Option<Vec<SizeEnum>>,

    #[schema(minimum = 0, maximum = 30, example = 0)]
    pub min_level: Option<u8>,
    #[schema(minimum = 0, maximum = 30, example = 5)]
    pub max_level: Option<u8>,

    #[schema(example = 0)]
    pub min_ac: Option<i64>,
    #[schema(example = 5)]
    pub max_ac: Option<i64>,

    #[schema(example = 0)]
    pub min_hardness: Option<i64>,
    #[schema(example = 5)]
    pub max_hardness: Option<i64>,

    #[schema(example = 0)]
    pub min_hp: Option<i64>,
    #[schema(example = 5)]
    pub max_hp: Option<i64>,

    #[schema(example = 0)]
    pub min_will: Option<i64>,
    #[schema(example = 5)]
    pub max_will: Option<i64>,

    #[schema(example = 0)]
    pub min_reflex: Option<i64>,
    #[schema(example = 5)]
    pub max_reflex: Option<i64>,

    #[schema(example = 0)]
    pub min_fortitude: Option<i64>,
    #[schema(example = 5)]
    pub max_fortitude: Option<i64>,

    #[schema(minimum = 1, maximum = 30, example = 1)]
    pub min_hazards: Option<u8>,
    #[schema(minimum = 1, maximum = 30, example = 5)]
    pub max_hazards: Option<u8>,

    pub(crate) hazard_complexity: Option<HazardComplexityEnum>,

    pub game_system_version: Option<GameSystemVersionEnum>,
}

#[derive(
    Copy,
    Serialize,
    Deserialize,
    ToSchema,
    Default,
    EnumIter,
    Eq,
    PartialEq,
    Hash,
    Ord,
    PartialOrd,
    Clone,
    EnumCount,
    Debug,
)]
pub enum EncounterChallengeEnum {
    #[serde(alias = "trivial", alias = "TRIVIAL")]
    Trivial,
    #[serde(alias = "low", alias = "LOW")]
    Low,
    #[default]
    #[serde(alias = "moderate", alias = "MODERATE")]
    Moderate,
    #[serde(alias = "severe", alias = "SEVERE")]
    Severe,
    #[serde(alias = "extreme", alias = "EXTREME")]
    Extreme,
    #[serde(alias = "impossible", alias = "IMPOSSIBLE")]
    Impossible,
    // Impossible = 320 with chara adjust 60, invented by me but what else can I do?
    // Pathfinder 2E thinks that a GM will only try out extreme encounter at maximum
    // I have to introduce a level for impossible things, Needs balancing Paizo help
}

impl From<EncounterChallengeEnum> for String {
    fn from(value: EncounterChallengeEnum) -> Self {
        Self::from(match value {
            EncounterChallengeEnum::Trivial => "TRIVIAL",
            EncounterChallengeEnum::Low => "LOW",
            EncounterChallengeEnum::Moderate => "MODERATE",
            EncounterChallengeEnum::Severe => "SEVERE",
            EncounterChallengeEnum::Extreme => "EXTREME",
            EncounterChallengeEnum::Impossible => "IMPOSSIBLE",
        })
    }
}

impl EncounterChallengeEnum {
    pub fn rand() -> Self {
        match WyRand::new().generate_range(0..Self::COUNT) {
            0 => Self::Trivial,
            1 => Self::Low,
            2 => Self::Moderate,
            3 => Self::Severe,
            4 => Self::Extreme,
            _ => Self::Impossible,
        }
    }
}

#[derive(
    Serialize,
    Deserialize,
    ToSchema,
    Default,
    EnumIter,
    Eq,
    PartialEq,
    Hash,
    Ord,
    PartialOrd,
    Clone,
    EnumCount,
)]
pub enum AdventureGroupEnum {
    #[serde(alias = "boss_and_lackeys", alias = "BOSS_AND_LACKEYS", alias = "BALA")]
    //(120 XP): One creature of party level + 2, four creatures of party level – 4
    BossAndLackeys,
    #[serde(
        alias = "boss_and_lieutenant",
        alias = "BOSS_AND_LIEUTENANT",
        alias = "BALI"
    )]
    //(120 XP): One creature of party level + 2, one creature of party level
    BossAndLieutenant,
    #[default]
    #[serde(alias = "elite_enemies", alias = "ELITE_ENEMIES", alias = "EE")]
    //(120 XP): Three creatures of party level
    EliteEnemies,
    #[serde(
        alias = "lieutenant_and_lackeys",
        alias = "LIEUTENANT_AND_LACKEYS",
        alias = "LAL"
    )]
    //(80 XP): One creature of party level, four creatures of party level – 4
    LieutenantAndLackeys,
    #[serde(alias = "mated_pair", alias = "MATED_PAIR", alias = "MP")]
    //(80 XP): Two creatures of party level
    MatedPair,
    #[serde(alias = "troop", alias = "TROOP", alias = "T")]
    //(80 XP): One creature of party level, two creatures of party level – 2
    Troop,
    #[serde(alias = "mook_squad", alias = "MOOK_SQUAD", alias = "MS")]
    //(60 XP): Six creatures of party level – 4
    MookSquad,
}

#[derive(Copy, Clone, Debug)]
pub struct ExpRange {
    pub lower_bound: i64,
    pub upper_bound: i64,
}
