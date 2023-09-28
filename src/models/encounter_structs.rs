use crate::models::party_member::PartyMember;
use serde::{Deserialize, Serialize};
use strum::EnumIter;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Serialize, Deserialize, IntoParams, Validate)]
pub struct Party {
    party_members: Vec<PartyMember>,
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
    #[default]
    Random,
    Trivial,
    Low,
    Moderate,
    Severe,
    Extreme,
    Impossible,
    // Impossible = 320 with chara adjust 60, invented by me but what else can I do?
    // Pathfinder 2E thinks that a GM will only try out extreme encounter at maximum
    // I have to introduce a level for impossible things, Needs balancing Paizo help
}
