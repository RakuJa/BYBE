use crate::models::encounter_structs::{
    CreatureEncounterParams, EncounterChallengeEnum, ExpRange, HazardEncounterParams,
};
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Neg;
// Used to explicitly tell about the iter trait
use crate::models::hazard::hazard_field_filter::HazardComplexityEnum;
use crate::services::encounter_handler::difficulty_utilities::scale_difficulty_exp;
use strum::IntoEnumIterator;

fn get_creature_encounter_lvl_and_exp_map(is_pwl_on: bool) -> HashMap<i64, i64> {
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

fn get_hazard_encounter_lvl_and_exp_map(
    hazard_complexity_enum: HazardComplexityEnum,
) -> HashMap<HazardComplexityEnum, HashMap<i64, i64>> {
    let simple_lvl_and_exp_map = hashmap! {
        -4 => 2,
        -3 => 3,
        -2 => 4,
        -1 => 6,
        0 => 8,
        1 => 12,
        2 => 16,
        3 => 24,
        4 => 30,
    };
    let complex_lvl_and_exp_map = hashmap! {
        -4 => 10,
        -3 => 15,
        -2 => 20,
        -1 => 30,
        0 => 40,
        1 => 60,
        2 => 80,
        3 => 120,
        4 => 150,
    };
    match hazard_complexity_enum {
        HazardComplexityEnum::Simple => {
            hashmap! {HazardComplexityEnum::Simple => simple_lvl_and_exp_map}
        }
        HazardComplexityEnum::Complex => {
            hashmap! {HazardComplexityEnum::Complex => complex_lvl_and_exp_map}
        }
        HazardComplexityEnum::Any => hashmap! {
                HazardComplexityEnum::Complex => complex_lvl_and_exp_map,
                HazardComplexityEnum::Simple => simple_lvl_and_exp_map,
        },
    }
}

fn average_level(levels: &[i64]) -> f64 {
    levels.iter().sum::<i64>() as f64 / levels.len() as f64
}

pub fn calculate_encounter_exp(
    party_levels: &[i64],
    creature_encounter_params: &Option<CreatureEncounterParams>,
    hazard_encounter_params: &Option<HazardEncounterParams>,
) -> i64 {
    let party_avg = average_level(party_levels);

    let cr_exp = creature_encounter_params.as_ref().map_or(0, |params| {
        calculate_creature_encounter_exp(
            party_avg,
            party_levels.len(),
            &params.enemy_levels,
            params.is_pwl_on,
        )
    });
    let hz_exp = hazard_encounter_params.as_ref().map_or(0, |hz_params| {
        calculate_hazard_encounter_exp(party_avg, hz_params.clone())
    });
    cr_exp + hz_exp
}

fn calculate_lvl_diff(party_lvl: f64, enemy_lvl: f64) -> f64 {
    if enemy_lvl < 0. && enemy_lvl < party_lvl {
        (enemy_lvl - party_lvl).abs().neg()
    } else {
        enemy_lvl - party_lvl
    }
}

/// Calculates the total XP awarded for defeating a set of hazards.
///
/// XP is computed by:
/// - Comparing each hazard level against the party average level
/// - Converting the level difference into XP using the hazard XP table
/// - Using a canonical party size of 4 (hazards do not scale with party size)
///
/// Returns the sum of XP for all hazards in the encounter.
///
/// # Panics
///
/// Panics if the internal hazard XP table does not contain an entry
/// for a hazard's complexity.
fn calculate_hazard_encounter_exp(party_avg: f64, hazard_params: HazardEncounterParams) -> i64 {
    let exp_map = get_hazard_encounter_lvl_and_exp_map(HazardComplexityEnum::Any);
    hazard_params
        .hazards
        .into_iter()
        .map(|hazard| {
            convert_lvl_diff_into_exp(
                calculate_lvl_diff(party_avg, hazard.level as f64),
                4,
                exp_map
                    .get(&hazard.complexity)
                    .expect("Map should contain all complexities"),
            )
        })
        .sum()
}

/// Calculates the total XP awarded for defeating a group of creatures.
///
/// XP is computed by:
/// - Comparing each creature level against the party average level
/// - Converting the level difference into XP using the creature XP table
/// - Scaling XP according to the provided party size
///
/// `is_pwl_on` selects the appropriate XP table variant.
///
/// Returns the sum of XP for all creatures in the encounter.
fn calculate_creature_encounter_exp(
    party_avg: f64,
    party_size: usize,
    enemy_levels: &[i64],
    is_pwl_on: bool,
) -> i64 {
    let exp_map = get_creature_encounter_lvl_and_exp_map(is_pwl_on);
    enemy_levels
        .iter()
        .map(|&curr_enemy_lvl| {
            convert_lvl_diff_into_exp(
                calculate_lvl_diff(party_avg, curr_enemy_lvl as f64),
                party_size,
                &exp_map,
            )
        })
        .sum()
}

pub fn calculate_encounter_scaling_difficulty(
    party_size: usize,
) -> HashMap<EncounterChallengeEnum, i64> {
    // Given the party size, it scales and calculates the threshold for the various difficulty levels
    EncounterChallengeEnum::iter()
        .map(|diff| {
            let exp = scale_difficulty_exp(diff, i64::try_from(party_size).unwrap_or(i64::MAX))
                .lower_bound;
            (diff, exp)
        })
        .collect()
}

pub fn calculate_encounter_difficulty(
    encounter_exp: i64,
    scaled_exp_levels: &HashMap<EncounterChallengeEnum, i64>,
) -> EncounterChallengeEnum {
    EncounterChallengeEnum::iter()
        .rev()
        .find(|diff| encounter_exp >= *scaled_exp_levels.get(diff).expect("Creature encounter exp map should contain all possible encounter challenges as key"))
        .unwrap_or(EncounterChallengeEnum::Trivial)
}

/// Calculates all possible enemy level combinations for an encounter of a given difficulty.
///
/// # Arguments
///
/// * `difficulty` - The target encounter difficulty.
/// * `party_levels` - The levels of each party member.
/// * `is_pwl_on` - Whether "Proficiency Without Level" variant rule is enabled.
///
/// # Returns
///
/// A `HashSet` of enemy level combinations, where each combination is a `Vec<i64>` of enemy levels
/// that would produce an encounter matching the requested difficulty.
pub fn calculate_lvl_combination_for_creature_encounter(
    exp_range: ExpRange,
    party_levels: &[i64],
    is_pwl_on: bool,
) -> HashSet<Vec<i64>> {
    let party_avg = average_level(party_levels);
    calculate_lvl_combinations_for_given_exp(
        exp_range,
        party_avg.floor() as i64,
        &get_creature_encounter_lvl_and_exp_map(is_pwl_on),
    )
}

/// Calculates all possible hazard level combinations for an encounter of a given difficulty.
///
/// # Arguments
///
/// * `difficulty` - The target encounter difficulty.
/// * `hazard_levels` - The levels of each hazard member.
/// * `is_pwl_on` - Whether "Proficiency Without Level" variant rule is enabled.
///
/// # Returns
///
/// A `HashSet` of enemy level combinations, where each combination is a `Vec<i64>` of enemy levels
/// that would produce an encounter matching the requested difficulty.
pub fn calculate_lvl_combination_for_hazard_encounter(
    exp_range: ExpRange,
    hazard_levels: &[i64],
    hazard_complexity: HazardComplexityEnum,
) -> HashSet<Vec<(HazardComplexityEnum, i64)>> {
    let party_avg = average_level(hazard_levels);
    calculate_hazard_lvl_combinations_for_given_exp(
        exp_range,
        party_avg.floor() as i64,
        &get_hazard_encounter_lvl_and_exp_map(hazard_complexity),
    )
}

pub fn filter_combinations_outside_range<T>(
    combinations: HashSet<Vec<T>>,
    lower_bound: Option<u8>,
    upper_bound: Option<u8>,
) -> HashSet<Vec<T>>
where
    T: Eq + Hash + Debug,
{
    let (lower, upper) = match (lower_bound, upper_bound) {
        (None, None) | (Some(0), Some(0)) => return combinations,
        (Some(l), None) => (l as i64, l as i64),
        (None, Some(u)) => (u as i64, u as i64),
        (Some(l), Some(u)) => (l as i64, u as i64),
    };
    combinations
        .into_iter()
        .filter(|curr_combo| {
            curr_combo.len() >= lower.unsigned_abs() as usize
                && curr_combo.len() <= upper.unsigned_abs() as usize
        })
        .collect()
}

fn convert_lvl_diff_into_exp(
    lvl_diff: f64,
    party_size: usize,
    lvl_and_exp_map: &HashMap<i64, i64>,
) -> i64 {
    let lvl_diff_rounded_down = lvl_diff.floor() as i64;
    lvl_and_exp_map.get(&lvl_diff_rounded_down).map_or_else(
        || {
            if lvl_diff_rounded_down
                < lvl_and_exp_map.keys().min().map_or_else(
                    || panic!("No valid lvl and exp map was passed. Abort"),
                    |max_lvl_diff| *max_lvl_diff,
                )
            {
                0
            } else {
                // To avoid the party of 50 level 1 pg destroying a lvl 20
                scale_difficulty_exp(
                    EncounterChallengeEnum::Impossible,
                    i64::try_from(party_size).unwrap_or(i64::MAX),
                )
                .lower_bound
            }
        },
        |value| value.abs(),
    )
}

/// Calculates all possible enemy(creature/hazard) level combinations that fit within a given XP budget.
///
/// Finds every combination of XP values that falls within `experience_range`, then maps
/// each XP value back to an absolute enemy level using the party's average level as an anchor.
/// Combinations containing invalid mappings or levels below `-1` are discarded,
/// as no creatures exist below that level.
///
/// # Arguments
///
/// * `experience_range` - The valid XP budget range for the encounter.
/// * `party_lvl` - The party's average level, used to convert relative level differences to absolute levels.
/// * `lvl_and_exp_map` - A map of relative level difference → XP value, used for conversion in both directions.
///
/// # Returns
///
/// A `HashSet` of enemy level combinations, deduplicated, each represented as a `Vec<i64>` of absolute enemy levels.
fn calculate_lvl_combinations_for_given_exp(
    experience_range: ExpRange,
    party_lvl: i64,
    lvl_and_exp_map: &HashMap<i64, i64>,
) -> HashSet<Vec<i64>> {
    let exp_list = lvl_and_exp_map.values().copied().collect::<Vec<i64>>();
    find_combinations(&exp_list, experience_range)
        .iter()
        .map(|curr_combination| {
            curr_combination
                .iter()
                .filter_map(|curr_exp| convert_exp_to_lvl_diff(*curr_exp, lvl_and_exp_map))
                .map(|lvl_diff| party_lvl + lvl_diff)
                .collect()
        })
        .filter(|x: &Vec<i64>| !x.is_empty())
        // there are no creature or hazards with level<-1
        .filter(|x| x.iter().all(|curr_lvl| *curr_lvl >= -1))
        .collect::<HashSet<Vec<i64>>>()
}

/// Calculates all possible hazard level combinations that fit within a given XP budget,
/// mixing both simple and complex hazards.
///
/// Flattens the nested `lvl_and_exp_map` into a unified list of `(complexity, exp)` pairs,
/// finds every combination of those pairs fitting within `experience_range`, then maps
/// each entry back to an absolute hazard level. Combinations with invalid mappings or
/// levels below `-1` are discarded.
///
/// # Arguments
///
/// * `experience_range` - The valid XP budget range for the encounter.
/// * `party_lvl` - The party's average level, used to convert relative level differences to absolute levels.
/// * `lvl_and_exp_map` - A nested map of `HazardComplexityEnum` → (level difference → XP value).
///
/// # Returns
///
/// A `HashSet` of hazard combinations, each represented as a `Vec<(HazardComplexityEnum, i64)>`
/// of `(complexity, absolute_level)` pairs.
fn calculate_hazard_lvl_combinations_for_given_exp(
    experience_range: ExpRange,
    party_lvl: i64,
    lvl_and_exp_map: &HashMap<HazardComplexityEnum, HashMap<i64, i64>>,
) -> HashSet<Vec<(HazardComplexityEnum, i64)>> {
    // Flatten into (complexity, lvl_diff, exp) triples, preserving full context
    let entries: Vec<(HazardComplexityEnum, i64, i64)> = lvl_and_exp_map
        .iter()
        .flat_map(|(complexity, inner_map)| {
            inner_map
                .iter()
                .map(|(&lvl_diff, &exp)| (*complexity, lvl_diff, exp))
                .collect::<Vec<_>>()
        })
        .collect();

    let exp_values: Vec<i64> = entries.iter().map(|(_, _, exp)| *exp).collect();

    // find_combinations returns combinations of values from exp_values.
    // We need index-based combinations to correctly resolve collisions like Simple(4) == Complex(-1) == 30xp
    find_combinations_by_index(&exp_values, experience_range)
        .iter()
        .map(|index_combination| {
            index_combination
                .iter()
                .filter_map(|&idx| {
                    let (complexity, lvl_diff, _) = entries.get(idx)?;
                    Some((*complexity, party_lvl + lvl_diff))
                })
                .collect::<Vec<_>>()
        })
        .filter(|x| !x.is_empty())
        // there are no hazards with level < -1
        .filter(|x| x.iter().all(|(_, lvl)| *lvl >= -1))
        .collect::<HashSet<Vec<(HazardComplexityEnum, i64)>>>()
}

fn find_combinations_by_index(exp_list: &[i64], experience_range: ExpRange) -> Vec<Vec<usize>> {
    let mut results = Vec::new();
    find_combinations_by_index_recursive(
        exp_list,
        experience_range,
        0,
        0,
        &mut vec![],
        &mut results,
    );
    results
}

const MAX_COMBINATION_SIZE: usize = 10;
fn find_combinations_by_index_recursive(
    exp_list: &[i64],
    experience_range: ExpRange,
    start: usize,
    current_sum: i64,
    current: &mut Vec<usize>,
    results: &mut Vec<Vec<usize>>,
) {
    if current_sum >= experience_range.lower_bound && current_sum <= experience_range.upper_bound {
        results.push(current.clone());
    }
    if current_sum >= experience_range.upper_bound || current.len() >= MAX_COMBINATION_SIZE {
        return;
    }
    for i in start..exp_list.len() {
        current.push(i);
        find_combinations_by_index_recursive(
            exp_list,
            experience_range,
            i, // allow reuse of same index for multiple hazards of the same type
            current_sum + exp_list[i],
            current,
            results,
        );
        current.pop();
    }
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

    let mut result = Vec::new();
    let mut path = Vec::new();
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn get_test_map() -> HashMap<HazardComplexityEnum, HashMap<i64, i64>> {
        get_hazard_encounter_lvl_and_exp_map(HazardComplexityEnum::default())
    }

    fn make_range(lower: i64, upper: i64) -> ExpRange {
        ExpRange {
            lower_bound: lower,
            upper_bound: upper,
        }
    }

    #[test]
    fn test_empty_when_exp_range_unreachable() {
        // With MAX_COMBINATION_SIZE=10 and max single XP=150 (Complex lvl 4),
        // the maximum reachable XP is 10*150=1500. So lower_bound > 1500 guarantees empty.
        let result = calculate_hazard_lvl_combinations_for_given_exp(
            make_range(9999, 10000),
            5,
            &get_test_map(),
        );
        assert!(result.is_empty());
    }

    #[test]
    fn test_no_levels_below_minus_one() {
        let result =
            calculate_hazard_lvl_combinations_for_given_exp(make_range(10, 40), 0, &get_test_map());
        assert!(
            result
                .iter()
                .all(|combo| combo.iter().all(|(_, lvl)| *lvl >= -1))
        );
    }

    #[test]
    fn test_collision_simple_4_and_complex_minus_one_both_present() {
        // Simple(lvl_diff=4) == 30 XP, Complex(lvl_diff=-1) == 30 XP
        // A range that fits exactly 30 XP should return BOTH as separate combinations
        let result =
            calculate_hazard_lvl_combinations_for_given_exp(make_range(30, 30), 5, &get_test_map());

        let has_simple = result.contains(&vec![(HazardComplexityEnum::Simple, 9)]); // party_lvl(5) + lvl_diff(4)
        let has_complex = result.contains(&vec![(HazardComplexityEnum::Complex, 4)]); // party_lvl(5) + lvl_diff(-1)
        assert!(has_simple, "Expected Simple(4) -> lvl 9 to be present");
        assert!(has_complex, "Expected Complex(-1) -> lvl 4 to be present");
    }

    #[test]
    fn test_combinations_sum_within_range() {
        let range = make_range(10, 20);
        let result =
            calculate_hazard_lvl_combinations_for_given_exp(range.clone(), 5, &get_test_map());
        // For every combination, re-derive its total XP and assert it falls within range
        let map = get_test_map();
        for combo in &result {
            let total_exp: i64 = combo
                .iter()
                .map(|(complexity, abs_lvl)| {
                    let lvl_diff = abs_lvl - 5;
                    map[complexity][&lvl_diff]
                })
                .sum();
            assert!(
                total_exp >= range.lower_bound && total_exp <= range.upper_bound,
                "Combination {:?} has total XP {} outside range {:?}",
                combo,
                total_exp,
                range
            );
        }
    }

    #[test]
    fn test_allows_multiple_hazards_of_same_type() {
        // Two Simple(-4) hazards = 2+2 = 4 XP
        let result =
            calculate_hazard_lvl_combinations_for_given_exp(make_range(4, 4), 5, &get_test_map());
        let two_simple = vec![
            (HazardComplexityEnum::Simple, 1), // 5 + (-4)
            (HazardComplexityEnum::Simple, 1),
        ];
        assert!(
            result.contains(&two_simple),
            "Expected two Simple(-4) hazards to be a valid combination"
        );
    }

    #[test]
    fn test_mixed_complexity_combination() {
        // Complex(-4)=10 + Simple(-4)=2 = 12 XP
        let result =
            calculate_hazard_lvl_combinations_for_given_exp(make_range(12, 12), 5, &get_test_map());
        assert!(!result.is_empty());
        assert!(result.iter().any(|combo| {
            combo.contains(&(HazardComplexityEnum::Complex, 1)) && // 5 + (-4)
                combo.contains(&(HazardComplexityEnum::Simple, 1))
        }));
    }
}
