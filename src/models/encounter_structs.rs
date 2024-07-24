use crate::models::creature::creature_metadata::alignment_enum::AlignmentEnum;
use crate::models::creature::creature_metadata::creature_role::CreatureRoleEnum;
use crate::models::creature::creature_metadata::type_enum::CreatureTypeEnum;
use crate::models::pf_version_enum::PathfinderVersionEnum;
use crate::models::shared::rarity_enum::RarityEnum;
use crate::models::shared::size_enum::SizeEnum;
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use serde::{Deserialize, Serialize};
use strum::EnumIter;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Serialize, Deserialize, ToSchema, Validate)]
pub struct EncounterParams {
    #[validate(length(min = 1))]
    pub party_levels: Vec<i64>,
    #[validate(length(min = 1))]
    pub enemy_levels: Vec<i64>,
    pub is_pwl_on: bool,
}

#[derive(Serialize, Deserialize, ToSchema, Validate)]
pub struct RandomEncounterData {
    pub families: Option<Vec<String>>,
    pub traits: Option<Vec<String>>,
    pub rarities: Option<Vec<RarityEnum>>,
    pub sizes: Option<Vec<SizeEnum>>,
    pub alignments: Option<Vec<AlignmentEnum>>,
    pub creature_types: Option<Vec<CreatureTypeEnum>>,
    pub creature_roles: Option<Vec<CreatureRoleEnum>>,
    pub challenge: Option<EncounterChallengeEnum>,
    pub adventure_group: Option<AdventureGroupEnum>,
    #[validate(range(min = 1, max = 30))]
    pub min_creatures: Option<u8>,
    #[validate(range(min = 1, max = 30))]
    pub max_creatures: Option<u8>,
    #[validate(length(min = 1))]
    pub party_levels: Vec<i64>,
    pub allow_elite_variants: Option<bool>,
    pub allow_weak_variants: Option<bool>,
    pub is_pwl_on: bool,
    pub pathfinder_version: Option<PathfinderVersionEnum>,
}

#[derive(
    Serialize, Deserialize, ToSchema, Default, EnumIter, Eq, PartialEq, Hash, Ord, PartialOrd, Clone,
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

impl Distribution<EncounterChallengeEnum> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> EncounterChallengeEnum {
        match rng.gen_range(0..6) {
            0 => EncounterChallengeEnum::Trivial,
            1 => EncounterChallengeEnum::Low,
            2 => EncounterChallengeEnum::Moderate,
            3 => EncounterChallengeEnum::Severe,
            4 => EncounterChallengeEnum::Extreme,
            _ => EncounterChallengeEnum::Impossible,
        }
    }
}

#[derive(
    Serialize, Deserialize, ToSchema, Default, EnumIter, Eq, PartialEq, Hash, Ord, PartialOrd, Clone,
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

impl Distribution<AdventureGroupEnum> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> AdventureGroupEnum {
        match rng.gen_range(0..7) {
            0 => AdventureGroupEnum::BossAndLackeys,
            1 => AdventureGroupEnum::BossAndLieutenant,
            2 => AdventureGroupEnum::EliteEnemies,
            3 => AdventureGroupEnum::LieutenantAndLackeys,
            4 => AdventureGroupEnum::MatedPair,
            5 => AdventureGroupEnum::Troop,
            _ => AdventureGroupEnum::MookSquad,
        }
    }
}

pub struct ExpRange {
    pub lower_bound: i64,
    pub upper_bound: i64,
}
