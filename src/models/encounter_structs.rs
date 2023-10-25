use crate::models::creature_metadata_enums::{
    AlignmentEnum, CreatureTypeEnum, RarityEnum, SizeEnum,
};
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use serde::{Deserialize, Serialize};
use strum::EnumIter;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Serialize, Deserialize, ToSchema, Validate)]
pub struct EncounterParams {
    #[validate(length(min = 1))]
    pub party_levels: Vec<i8>,
    #[validate(length(min = 1))]
    pub enemy_levels: Vec<i8>,
}

#[derive(Serialize, Deserialize, ToSchema, Validate)]
pub struct RandomEncounterData {
    pub family: Option<String>,
    pub traits: Option<Vec<String>>,
    pub rarity: Option<RarityEnum>,
    pub size: Option<SizeEnum>,
    pub alignment: Option<AlignmentEnum>,
    pub creature_types: Option<Vec<CreatureTypeEnum>>,
    pub challenge: Option<EncounterChallengeEnum>,
    #[validate(length(min = 1))]
    pub party_levels: Vec<i16>,
}

#[derive(Serialize, Deserialize, ToSchema, Default, EnumIter, Eq, PartialEq, Hash, Clone)]
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
