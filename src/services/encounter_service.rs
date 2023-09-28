use crate::models::encounter_structs::{EncounterDifficultyEnum, EncounterParams};
use crate::services::encounter_handler::encounter_calculator;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct EncounterInfoResponse {
    experience: i16,
    difficulty: EncounterDifficultyEnum,
    encounter_exp_levels: HashMap<EncounterDifficultyEnum, i16>,
}

pub fn get_encounter_info(enc_params: EncounterParams) -> EncounterInfoResponse {
    let enc_exp = encounter_calculator::calculate_encounter_exp(
        &enc_params.party_levels,
        &enc_params.enemy_levels,
    );

    let scaled_exp =
        encounter_calculator::calculate_encounter_scaling_difficulty(enc_params.party_levels.len());

    let enc_diff = encounter_calculator::calculate_encounter_difficulty(enc_exp, &scaled_exp);
    EncounterInfoResponse {
        experience: enc_exp,
        difficulty: enc_diff,
        encounter_exp_levels: scaled_exp,
    }
}
