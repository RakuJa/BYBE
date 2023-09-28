use crate::models::encounter_structs::EncounterDifficultyEnum;
use crate::services::encounter_handler::difficulty_utilities::scale_difficulty_exp;
use lazy_static::lazy_static;
use std::collections::HashMap;
// Used to explicitly tell about the iter trait
use strum::IntoEnumIterator;

static MAX_LVL_DIFF: i16 = -4;
lazy_static! {
    static ref LVL_AND_EXP_MAP: HashMap<i16, i16> = hashmap! {
        -4 => 10,
        -3 => 15,
        -2 => 20,
        -1 => 30,
        0 => 40,
        1 => 60,
        2 => 80,
        3 => 120,
        4 => 160,
    };
}
pub fn calculate_encounter_exp(party_levels: &[i8], enemy_levels: &[i8]) -> i16 {
    // Given a party and enemy party, it calculates the exp that the
    // party will get from defeating the enemy
    let party_avg = party_levels.iter().sum::<i8>() as f32 / party_levels.len() as f32;

    let exp_sum = enemy_levels
        .iter()
        .map(|&curr_enemy_lvl| {
            let enemy_lvl = curr_enemy_lvl as f32;
            convert_lvl_diff_into_exp(
                (enemy_lvl - party_avg).abs() * enemy_lvl.signum(),
                party_levels.len(),
            )
        })
        .sum();
    exp_sum
}

pub fn calculate_encounter_scaling_difficulty(
    party_size: usize,
) -> HashMap<EncounterDifficultyEnum, i16> {
    // Given the party size, it scales and calculates the threshold for the various difficulty levels
    let mut diff_scaled_exp_map = HashMap::new();
    for curr_diff in EncounterDifficultyEnum::iter() {
        diff_scaled_exp_map.insert(
            curr_diff.clone(),
            scale_difficulty_exp(&curr_diff, party_size as u8),
        );
    }
    diff_scaled_exp_map
}

pub fn calculate_encounter_difficulty(
    encounter_exp: i16,
    scaled_exp_levels: &HashMap<EncounterDifficultyEnum, i16>,
) -> EncounterDifficultyEnum {
    // This method is ugly, it's 1:1 from python and as such needs refactor
    if &encounter_exp
        < scaled_exp_levels
            .get(&EncounterDifficultyEnum::Low)
            .unwrap()
    {
        return EncounterDifficultyEnum::Trivial;
    } else if &encounter_exp
        < scaled_exp_levels
            .get(&EncounterDifficultyEnum::Moderate)
            .unwrap()
    {
        return EncounterDifficultyEnum::Low;
    } else if &encounter_exp
        < scaled_exp_levels
            .get(&EncounterDifficultyEnum::Severe)
            .unwrap()
    {
        return EncounterDifficultyEnum::Moderate;
    } else if &encounter_exp
        < scaled_exp_levels
            .get(&EncounterDifficultyEnum::Extreme)
            .unwrap()
    {
        return EncounterDifficultyEnum::Severe;
    } else if &encounter_exp
        < scaled_exp_levels
            .get(&EncounterDifficultyEnum::Impossible)
            .unwrap()
    {
        return EncounterDifficultyEnum::Extreme;
    }
    EncounterDifficultyEnum::Impossible
}

fn convert_lvl_diff_into_exp(lvl_diff: f32, party_size: usize) -> i16 {
    let lvl_diff_rounded_down = lvl_diff.floor() as i16;
    LVL_AND_EXP_MAP
        .get(&lvl_diff_rounded_down)
        .map(|value| value.abs())
        .unwrap_or(if lvl_diff_rounded_down < MAX_LVL_DIFF {
            0i16
        } else {
            // To avoid the party of 50 level 1 pg destroying a lvl 20
            scale_difficulty_exp(&EncounterDifficultyEnum::Impossible, party_size as u8)
        })
}
