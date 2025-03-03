use crate::models::encounter_structs::{EncounterChallengeEnum, ExpRange};
use crate::services::encounter_handler::difficulty_utilities::scale_difficulty_exp;
use std::collections::{HashMap, HashSet};
use std::ops::Neg;
// Used to explicitly tell about the iter trait
use strum::IntoEnumIterator;

fn calculate_max_lvl_diff(lvl_and_exp_map: &HashMap<i64, i64>) -> i64 {
    lvl_and_exp_map.keys().min().map_or_else(
        || panic!("No valid lvl and exp map was passed. Abort"),
        |max_lvl_diff| *max_lvl_diff,
    )
}

fn calculate_lvl_and_exp_map(is_pwl_on: bool) -> HashMap<i64, i64> {
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

pub fn calculate_encounter_exp(party_levels: &[i64], enemy_levels: &[i64], is_pwl_on: bool) -> i64 {
    // Given a party and enemy party, it calculates the exp that the
    // party will get from defeating the enemy
    let party_avg = party_levels.iter().sum::<i64>() as f64 / party_levels.len() as f64;
    let exp_sum = enemy_levels
        .iter()
        .map(|&curr_enemy_lvl| {
            let enemy_lvl = curr_enemy_lvl as f64;
            let lvl_diff = if enemy_lvl < 0. && enemy_lvl < party_avg {
                (enemy_lvl - party_avg).abs().neg()
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
) -> HashMap<EncounterChallengeEnum, i64> {
    // Given the party size, it scales and calculates the threshold for the various difficulty levels
    let mut diff_scaled_exp_map = HashMap::new();
    for curr_diff in EncounterChallengeEnum::iter() {
        diff_scaled_exp_map.insert(
            curr_diff.clone(),
            scale_difficulty_exp(&curr_diff, i64::try_from(party_size).unwrap_or(i64::MAX))
                .lower_bound,
        );
    }
    diff_scaled_exp_map
}

pub fn calculate_encounter_difficulty(
    encounter_exp: i64,
    scaled_exp_levels: &HashMap<EncounterChallengeEnum, i64>,
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
    party_levels: &[i64],
    is_pwl_on: bool,
) -> HashSet<Vec<i64>> {
    // Given an encounter difficulty it calculates all possible encounter permutations
    let exp_range = scale_difficulty_exp(
        difficulty,
        i64::try_from(party_levels.len()).unwrap_or(i64::MAX),
    );
    let party_avg: f64 = party_levels.iter().sum::<i64>() as f64 / party_levels.len() as f64;
    calculate_lvl_combinations_for_given_exp(
        exp_range,
        party_avg.floor() as i64,
        &calculate_lvl_and_exp_map(is_pwl_on),
    )
}

pub fn filter_combinations_outside_range(
    combinations: HashSet<Vec<i64>>,
    lower_bound: Option<u8>,
    upper_bound: Option<u8>,
) -> HashSet<Vec<i64>> {
    let mut lower = i64::from(lower_bound.unwrap_or(0));
    let mut upper = i64::from(upper_bound.unwrap_or(0));
    if lower != 0 && upper == 0 {
        upper = lower;
    } else if lower == 0 && upper != 0 {
        lower = upper;
    } else if lower == 0 && upper == 0 {
        return combinations;
    }
    combinations
        .into_iter()
        .filter(|curr_combo| {
            curr_combo.len() >= lower.unsigned_abs() as usize
                && curr_combo.len() <= upper.unsigned_abs() as usize
        })
        .collect::<HashSet<Vec<i64>>>()
}

fn convert_lvl_diff_into_exp(
    lvl_diff: f64,
    party_size: usize,
    lvl_and_exp_map: &HashMap<i64, i64>,
) -> i64 {
    let lvl_diff_rounded_down = lvl_diff.floor() as i64;
    lvl_and_exp_map.get(&lvl_diff_rounded_down).map_or_else(
        || {
            if lvl_diff_rounded_down < calculate_max_lvl_diff(lvl_and_exp_map) {
                0
            } else {
                // To avoid the party of 50 level 1 pg destroying a lvl 20
                scale_difficulty_exp(
                    &EncounterChallengeEnum::Impossible,
                    i64::try_from(party_size).unwrap_or(i64::MAX),
                )
                .lower_bound
            }
        },
        |value| value.abs(),
    )
}

fn calculate_lvl_combinations_for_given_exp(
    experience_range: ExpRange,
    party_lvl: i64,
    lvl_and_exp_map: &HashMap<i64, i64>,
) -> HashSet<Vec<i64>> {
    // Given an encounter experience it calculates all possible encounter permutations
    let exp_list = lvl_and_exp_map.values().copied().collect::<Vec<i64>>();
    find_combinations(&exp_list, experience_range)
        .iter()
        .map(|curr_combination| {
            curr_combination
                .iter()
                .map(|curr_exp| convert_exp_to_lvl_diff(*curr_exp, lvl_and_exp_map))
                .filter(Option::is_some)
                .map(|lvl_diff| party_lvl + lvl_diff.unwrap())
                .collect()
        })
        .filter(|x: &Vec<i64>| !x.is_empty())
        // there are no creature with level<-1
        .filter(|x| x.iter().all(|curr_lvl| *curr_lvl >= -1))
        .collect::<HashSet<Vec<i64>>>()
}

fn convert_exp_to_lvl_diff(experience: i64, lvl_and_exp_map: &HashMap<i64, i64>) -> Option<i64> {
    lvl_and_exp_map
        .iter()
        .find_map(|(key, &exp)| if exp == experience { Some(*key) } else { None })
}

fn find_combinations(candidates: &[i64], target_range: ExpRange) -> Vec<Vec<i64>> {
    // Find all the combination of numbers in the candidates vector
    // that sums up to the target. I.e coin changing problem
    fn backtrack(
        candidates: &[i64],
        lb_target: i64,
        ub_target: i64,
        start: usize,
        path: &mut Vec<i64>,
        result: &mut Vec<Vec<i64>>,
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
        candidates,
        target_range.lower_bound,
        target_range.upper_bound,
        0,
        &mut path,
        &mut result,
    );

    result
}
