use crate::models::encounter_structs::EncounterDifficultyEnum;
use crate::services::encounter_handler::difficulty_utilities::scale_difficulty_exp;
use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};
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
            scale_difficulty_exp(&curr_diff, party_size as i16),
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

pub fn calculate_lvl_combination_for_encounter(
    difficulty: &EncounterDifficultyEnum,
    party_levels: &Vec<i16>,
) -> (i16, HashSet<Vec<i16>>) {
    // Given an encounter difficulty it calculates all possible encounter permutations
    let exp = scale_difficulty_exp(difficulty, party_levels.len() as i16);
    let party_avg = party_levels.iter().sum::<i16>() as f32 / party_levels.len() as f32;
    (
        exp,
        calculate_lvl_combinations_for_given_exp(exp, party_avg.floor()),
    )
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
            scale_difficulty_exp(&EncounterDifficultyEnum::Impossible, party_size as i16)
        })
}

fn calculate_lvl_combinations_for_given_exp(experience: i16, party_lvl: f32) -> HashSet<Vec<i16>> {
    // Given an encounter experience it calculates all possible encounter permutations
    let exp_list = LVL_AND_EXP_MAP.values().cloned().collect::<Vec<i16>>();

    find_combinations(exp_list, experience)
        .iter()
        .map(|curr_combination| {
            curr_combination
                .iter()
                .map(|curr_exp| convert_exp_to_lvl_diff(*curr_exp))
                .filter(|a| a.is_some())
                .map(|lvl_diff| party_lvl as i16 + lvl_diff.unwrap())
                .filter(|lvl_combo| *lvl_combo >= -1) // there are no creature with level<-1
                .collect::<Vec<i16>>()
            // I'mma gonna puke mamma mia
        })
        .filter(|x| !x.is_empty())
        .collect::<HashSet<Vec<i16>>>()
}

fn convert_exp_to_lvl_diff(experience: i16) -> Option<i16> {
    LVL_AND_EXP_MAP
        .iter()
        .find_map(|(key, &exp)| if exp == experience { Some(*key) } else { None })
}

fn find_combinations(candidates: Vec<i16>, target: i16) -> Vec<Vec<i16>> {
    // Find all the combination of numbers in the candidates vector
    // that sums up to the target. I.e coin changing problem
    fn backtrack(
        candidates: &Vec<i16>,
        target: i16,
        start: usize,
        path: &mut Vec<i16>,
        result: &mut Vec<Vec<i16>>,
    ) {
        if target == 0 {
            // If target is reached, add the current path to results list
            result.push(path.clone());
        }

        if target > 0 {
            // Iterate through the candidates starting from the given index
            for i in start..candidates.len() {
                path.push(candidates[i]);
                backtrack(candidates, target - candidates[i], i, path, result);
                path.pop();
            }
        }

        if target < 1 {
            // If target is negative or 0 no need to continue as
            // adding more numbers will exceed the target
        }
    }

    let mut result = Vec::new(); // List to store all combinations
    let mut path = Vec::new(); // Sort the candidates list for optimization
                               // Start the backtracking from the first index
    backtrack(&candidates, target, 0, &mut path, &mut result);

    result
}
