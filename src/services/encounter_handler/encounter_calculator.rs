use crate::models::encounter_structs::{EncounterChallengeEnum, ExpRange};
use crate::services::encounter_handler::difficulty_utilities::scale_difficulty_exp;
use std::collections::{HashMap, HashSet};
use std::ops::Neg;
// Used to explicitly tell about the iter trait
use strum::IntoEnumIterator;
use validator::HasLen;

fn calculate_max_lvl_diff(lvl_and_exp_map: &HashMap<i16, i16>) -> i16 {
    match lvl_and_exp_map.keys().min() {
        None => panic!("No valid lvl and exp map was passed. Abort"),
        Some(max_lvl_diff) => *max_lvl_diff,
    }
}

fn calculate_lvl_and_exp_map(is_pwl_on: bool) -> HashMap<i16, i16> {
    // PWL stands for proficiency without level
    if is_pwl_on {
        hashmap! {
            -7 => 9,
            -6 => 12,
            -5 => 14,
            -4 => 18,
            -3 => 21,
            -2 => 26,
            -1 => 32,
            0 => 40,
            1 => 48,
            2 => 60,
            3 => 72,
            4 => 90,
            5 => 108,
            6 => 135,
            7 => 160,
        }
    } else {
        hashmap! {
            -4 => 10,
            -3 => 15,
            -2 => 20,
            -1 => 30,
            0 => 40,
            1 => 60,
            2 => 80,
            3 => 120,
            4 => 160,
        }
    }
}

pub fn calculate_encounter_exp(party_levels: &[i16], enemy_levels: &[i16], is_pwl_on: bool) -> i16 {
    // Given a party and enemy party, it calculates the exp that the
    // party will get from defeating the enemy
    let party_avg = party_levels.iter().sum::<i16>() as f32 / party_levels.len() as f32;
    let exp_sum = enemy_levels
        .iter()
        .map(|&curr_enemy_lvl| {
            let enemy_lvl = curr_enemy_lvl as f32;
            let lvl_diff = if enemy_lvl < 0f32 && enemy_lvl < party_avg {
                ((enemy_lvl - party_avg).abs()).neg()
            } else {
                enemy_lvl - party_avg
            };
            convert_lvl_diff_into_exp(
                lvl_diff,
                party_levels.len(),
                &calculate_lvl_and_exp_map(is_pwl_on),
            )
        })
        .sum();
    exp_sum
}

pub fn calculate_encounter_scaling_difficulty(
    party_size: usize,
) -> HashMap<EncounterChallengeEnum, i16> {
    // Given the party size, it scales and calculates the threshold for the various difficulty levels
    let mut diff_scaled_exp_map = HashMap::new();
    for curr_diff in EncounterChallengeEnum::iter() {
        diff_scaled_exp_map.insert(
            curr_diff.clone(),
            scale_difficulty_exp(&curr_diff, party_size as i16).lower_bound,
        );
    }
    diff_scaled_exp_map
}

pub fn calculate_encounter_difficulty(
    encounter_exp: i16,
    scaled_exp_levels: &HashMap<EncounterChallengeEnum, i16>,
) -> EncounterChallengeEnum {
    // This method is ugly, it's 1:1 from python and as such needs refactor
    if &encounter_exp < scaled_exp_levels.get(&EncounterChallengeEnum::Low).unwrap() {
        return EncounterChallengeEnum::Trivial;
    } else if &encounter_exp
        < scaled_exp_levels
            .get(&EncounterChallengeEnum::Moderate)
            .unwrap()
    {
        return EncounterChallengeEnum::Low;
    } else if &encounter_exp
        < scaled_exp_levels
            .get(&EncounterChallengeEnum::Severe)
            .unwrap()
    {
        return EncounterChallengeEnum::Moderate;
    } else if &encounter_exp
        < scaled_exp_levels
            .get(&EncounterChallengeEnum::Extreme)
            .unwrap()
    {
        return EncounterChallengeEnum::Severe;
    } else if &encounter_exp
        < scaled_exp_levels
            .get(&EncounterChallengeEnum::Impossible)
            .unwrap()
    {
        return EncounterChallengeEnum::Extreme;
    }
    EncounterChallengeEnum::Impossible
}

pub fn calculate_lvl_combination_for_encounter(
    difficulty: &EncounterChallengeEnum,
    party_levels: &[i16],
    is_pwl_on: bool,
) -> HashSet<Vec<i16>> {
    // Given an encounter difficulty it calculates all possible encounter permutations
    let exp_range = scale_difficulty_exp(difficulty, party_levels.len() as i16);
    let party_avg = party_levels.iter().sum::<i16>() as f32 / party_levels.len() as f32;
    calculate_lvl_combinations_for_given_exp(
        exp_range,
        party_avg.floor(),
        calculate_lvl_and_exp_map(is_pwl_on),
    )
}

pub fn filter_combinations_outside_range(
    combinations: HashSet<Vec<i16>>,
    lower_bound: Option<u8>,
    upper_bound: Option<u8>,
) -> HashSet<Vec<i16>> {
    let mut lower = lower_bound.unwrap_or(0);
    let mut upper = upper_bound.unwrap_or(0);
    if lower != 0 && upper == 0 {
        upper = lower;
    } else if lower == 0 && upper != 0 {
        lower = upper;
    } else if lower == 0 && upper == 0 {
        return combinations;
    }
    let mut filtered_combo = HashSet::new();
    combinations.iter().for_each(|curr_combo| {
        if curr_combo.length() >= lower as u64 && curr_combo.length() <= upper as u64 {
            filtered_combo.insert(curr_combo.clone());
        }
    });
    filtered_combo
}

fn convert_lvl_diff_into_exp(
    lvl_diff: f32,
    party_size: usize,
    lvl_and_exp_map: &HashMap<i16, i16>,
) -> i16 {
    let lvl_diff_rounded_down = lvl_diff.floor() as i16;
    lvl_and_exp_map
        .get(&lvl_diff_rounded_down)
        .map(|value| value.abs())
        .unwrap_or(
            if lvl_diff_rounded_down < calculate_max_lvl_diff(lvl_and_exp_map) {
                0i16
            } else {
                // To avoid the party of 50 level 1 pg destroying a lvl 20
                scale_difficulty_exp(&EncounterChallengeEnum::Impossible, party_size as i16)
                    .lower_bound
            },
        )
}

fn calculate_lvl_combinations_for_given_exp(
    experience_range: ExpRange,
    party_lvl: f32,
    lvl_and_exp_map: HashMap<i16, i16>,
) -> HashSet<Vec<i16>> {
    // Given an encounter experience it calculates all possible encounter permutations
    let exp_list = lvl_and_exp_map.values().cloned().collect::<Vec<i16>>();
    find_combinations(exp_list, experience_range)
        .iter()
        .map(|curr_combination| {
            curr_combination
                .iter()
                .map(|curr_exp| convert_exp_to_lvl_diff(*curr_exp, &lvl_and_exp_map))
                .filter(|a| a.is_some())
                .map(|lvl_diff| party_lvl as i16 + lvl_diff.unwrap())
                .collect::<Vec<i16>>()
        })
        .filter(|x| !x.is_empty())
        // there are no creature with level<-1
        .filter(|x| x.iter().all(|curr_lvl| *curr_lvl >= -1))
        .collect::<HashSet<Vec<i16>>>()
}

fn convert_exp_to_lvl_diff(experience: i16, lvl_and_exp_map: &HashMap<i16, i16>) -> Option<i16> {
    lvl_and_exp_map
        .iter()
        .find_map(|(key, &exp)| if exp == experience { Some(*key) } else { None })
}

fn find_combinations(candidates: Vec<i16>, target_range: ExpRange) -> Vec<Vec<i16>> {
    // Find all the combination of numbers in the candidates vector
    // that sums up to the target. I.e coin changing problem
    fn backtrack(
        candidates: &Vec<i16>,
        lb_target: i16,
        ub_target: i16,
        start: usize,
        path: &mut Vec<i16>,
        result: &mut Vec<Vec<i16>>,
    ) {
        if lb_target == 0 || (lb_target < 0 && ub_target > 0) {
            // If target is reached OR we exceeded lower bound but still not have reached upper bound,
            // AKA we did not go over the current difficulty level,
            // add the current path to results list
            result.push(path.clone());
        }

        if lb_target > 0 {
            // Iterate through the candidates starting from the given index
            for i in start..candidates.len() {
                path.push(candidates[i]);
                backtrack(
                    candidates,
                    lb_target - candidates[i],
                    ub_target - candidates[i],
                    i,
                    path,
                    result,
                );
                path.pop();
            }
        }

        if lb_target < 1 {
            // If target is negative or 0 no need to continue as
            // adding more numbers will exceed the target
        }
    }

    let mut result = Vec::new(); // List to store all combinations
    let mut path = Vec::new(); // Sort the candidates list for optimization
                               // Start the backtracking from the first index
    backtrack(
        &candidates,
        target_range.lower_bound,
        target_range.upper_bound,
        0,
        &mut path,
        &mut result,
    );

    result
}
