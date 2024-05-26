use crate::models::creature_metadata::alignment_enum::AlignmentEnum;
use crate::models::creature_metadata::creature_role::CreatureRoleEnum;
use crate::models::creature_metadata::rarity_enum::RarityEnum;
use crate::models::creature_metadata::size_enum::SizeEnum;
use crate::models::creature_metadata::type_enum::CreatureTypeEnum;
use crate::models::pf_version_enum::PathfinderVersionEnum;
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
    pub min_creatures: Option<u8>,
    pub max_creatures: Option<u8>,
    #[validate(length(min = 1))]
    pub party_levels: Vec<i64>,
    pub allow_elite_variants: Option<bool>,
    pub allow_weak_variants: Option<bool>,
    pub is_pwl_on: bool,
    pub pathfinder_versions: Option<PathfinderVersionEnum>,
}

#[derive(
    Serialize, Deserialize, ToSchema, Default, EnumIter, Eq, PartialEq, Hash, Ord, PartialOrd, Clone,
)]
pub enum EncounterChallengeEnum {
    Trivial,
    Low,
    #[default]
    Moderate,
    Severe,
    Extreme,
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

pub struct ExpRange {
    pub lower_bound: i64,
    pub upper_bound: i64,
}
