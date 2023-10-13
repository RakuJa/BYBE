use rand::distributions::{Distribution, Standard};
use rand::Rng;
use serde::{Deserialize, Serialize};
use strum::EnumIter;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Serialize, Deserialize, IntoParams, Validate)]
pub struct Party {
    #[validate(length(min = 1))]
    pub party_levels: Vec<i16>,
}

#[derive(Serialize, Deserialize, ToSchema, Validate)]
pub struct EncounterParams {
    #[validate(length(min = 1))]
    pub party_levels: Vec<i8>,
    #[validate(length(min = 1))]
    pub enemy_levels: Vec<i8>,
}

#[derive(Serialize, Deserialize, ToSchema, Default, EnumIter, Eq, PartialEq, Hash, Clone)]
pub enum EncounterDifficultyEnum {
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

impl Distribution<EncounterDifficultyEnum> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> EncounterDifficultyEnum {
        match rng.gen_range(0..6) {
            0 => EncounterDifficultyEnum::Trivial,
            1 => EncounterDifficultyEnum::Low,
            2 => EncounterDifficultyEnum::Moderate,
            3 => EncounterDifficultyEnum::Severe,
            4 => EncounterDifficultyEnum::Extreme,
            _ => EncounterDifficultyEnum::Impossible,
        }
    }
}
