use crate::models::encounter_structs::{
    AdventureGroupEnum, EncounterChallengeEnum, EncounterParams, ExpRange,
};
use crate::models::hazard::hazard_field_filter::HazardComplexityEnum;
use crate::models::response_data::EncounterInfoResponse;
use crate::services::encounter_handler::difficulty_utilities::scale_difficulty_exp;
use crate::services::encounter_handler::encounter_math;
use crate::services::encounter_handler::encounter_math::calculate_encounter_scaling_difficulty;
use crate::traits::has_complexity::HasComplexity;
use crate::traits::has_level::HasLevel;
use anyhow::{Context, Result, ensure};
use counter::Counter;
use nanorand::{Rng, WyRand};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub fn get_encounter_info(enc_params: &EncounterParams) -> EncounterInfoResponse {
    let enc_exp = encounter_math::calculate_encounter_exp(
        &enc_params.party_levels,
        &enc_params.creatures_params,
        &enc_params.hazards_params,
    );

    let scaled_exp = calculate_encounter_scaling_difficulty(enc_params.party_levels.len());

    let enc_diff = encounter_math::calculate_encounter_difficulty(enc_exp, &scaled_exp);
    EncounterInfoResponse {
        experience: enc_exp,
        challenge: enc_diff,
        encounter_exp_levels: scaled_exp.into_iter().collect(),
    }
}

fn order_list_by_key<T, K, F>(elements: &[T], key_fn: F) -> HashMap<K, Vec<T>>
where
    T: Clone,
    K: Eq + Hash,
    F: Fn(&T) -> K,
{
    let mut map: HashMap<K, Vec<T>> = HashMap::new();
    for el in elements {
        map.entry(key_fn(el)).or_default().push(el.clone());
    }
    map
}

fn choose_random_from_combinations<K>(
    available_keys: &HashSet<K>,
    lvl_combinations: HashSet<Vec<K>>,
) -> Result<Vec<K>>
where
    K: Eq + Hash + Clone,
{
    let existing: Vec<_> = lvl_combinations
        .into_iter()
        .filter(|combo| !combo.is_empty() && combo.iter().all(|k| available_keys.contains(k)))
        .collect();

    ensure!(
        !existing.is_empty(),
        "No valid level combinations to randomly choose from"
    );

    let mut rng = WyRand::new();
    Ok(existing[rng.generate_range(..existing.len())].clone())
}

fn fill_combination_generic<T, K>(
    elements: &[T],
    random_combo: &[K],
    key_fn: impl Fn(&T) -> K,
) -> Result<Vec<T>>
where
    T: Clone,
    K: Eq + Hash + Clone,
{
    let by_key = order_list_by_key(elements, key_fn);
    let key_count = random_combo.iter().collect::<Counter<_>>();
    let mut rng = WyRand::new();
    let mut result: Vec<T> = Vec::new();

    for (key, required_count) in key_count {
        let pool = by_key
            .get(key)
            .filter(|v| !v.is_empty())
            .with_context(|| "No elements for the chosen level")?;

        // Elements are cycled (with repetition) if the pool is smaller than required_count.
        let mut filled: Vec<_> = pool.iter().cycle().take(required_count).cloned().collect();
        rng.shuffle(&mut filled);
        result.extend(filled);
    }

    Ok(result)
}

fn choose_random_combination_generic<T, K>(
    elements: &[T],
    lvl_combinations: HashSet<Vec<K>>,
    key_fn: impl Fn(&T) -> K,
) -> Result<Vec<T>>
where
    T: Clone,
    K: Eq + Hash + Clone,
{
    let by_key = order_list_by_key(elements, &key_fn);
    let available_keys: HashSet<K> = by_key.keys().cloned().collect();
    let random_combo = choose_random_from_combinations(&available_keys, lvl_combinations)?;
    fill_combination_generic(elements, &random_combo, key_fn)
}

pub fn choose_random_combination<T>(
    elements: &[T],
    lvl_combinations: HashSet<Vec<i64>>,
) -> Result<Vec<T>>
where
    T: HasLevel + Clone,
{
    choose_random_combination_generic(elements, lvl_combinations, |el| el.level())
}

pub fn choose_hazard_random_combination<T>(
    elements: &[T],
    lvl_combinations: HashSet<Vec<(HazardComplexityEnum, i64)>>,
) -> Result<Vec<T>>
where
    T: HasLevel + HasComplexity + Clone,
{
    choose_random_combination_generic(elements, lvl_combinations, |el| {
        (el.complexity(), el.level())
    })
}

pub const fn get_scaled_exp(base_difficulty: EncounterChallengeEnum, party_size: i64) -> ExpRange {
    scale_difficulty_exp(base_difficulty, party_size)
}

pub fn get_creature_lvl_combinations(
    party_levels: &[i64],
    exp_range: ExpRange,
    is_pwl_on: bool,
    min_n_of_elements: Option<u8>,
    max_n_of_elements: Option<u8>,
    adventure_group: Option<AdventureGroupEnum>,
) -> HashSet<Vec<i64>> {
    adventure_group.as_ref().map_or_else(
        || {
            encounter_math::filter_combinations_outside_range(
                encounter_math::calculate_lvl_combination_for_creature_encounter(
                    exp_range,
                    party_levels,
                    is_pwl_on,
                ),
                min_n_of_elements,
                max_n_of_elements,
            )
        },
        |adv_group| get_adventure_group_lvl_combinations(adv_group, party_levels),
    )
}

pub fn get_hazard_lvl_combinations(
    party_levels: &[i64],
    exp_range: ExpRange,
    hazard_complexity: HazardComplexityEnum,
    min_n_of_elements: Option<u8>,
    max_n_of_elements: Option<u8>,
) -> HashSet<Vec<(HazardComplexityEnum, i64)>> {
    encounter_math::filter_combinations_outside_range(
        encounter_math::calculate_lvl_combination_for_hazard_encounter(
            exp_range,
            party_levels,
            hazard_complexity,
        ),
        min_n_of_elements,
        max_n_of_elements,
    )
}

fn get_adventure_group_lvl_combinations(
    adv_group: &AdventureGroupEnum,
    party_levels: &[i64],
) -> HashSet<Vec<i64>> {
    let party_avg =
        party_levels.iter().sum::<i64>() / i64::try_from(party_levels.len()).unwrap_or(i64::MAX);
    let mut result = HashSet::new();
    result.insert(match adv_group {
        AdventureGroupEnum::BossAndLackeys => {
            //One creature of party level + 2, four creatures of party level – 4
            vec![
                party_avg + 2,
                party_avg - 4,
                party_avg - 4,
                party_avg - 4,
                party_avg - 4,
            ]
        }
        AdventureGroupEnum::BossAndLieutenant => {
            //One creature of party level + 2, one creature of party level
            vec![party_avg + 2, party_avg]
        }
        AdventureGroupEnum::EliteEnemies => {
            //Three creatures of party level
            vec![party_avg; 3]
        }
        AdventureGroupEnum::LieutenantAndLackeys => {
            //One creature of party level, four creatures of party level – 4
            vec![
                party_avg,
                party_avg - 4,
                party_avg - 4,
                party_avg - 4,
                party_avg - 4,
            ]
        }
        AdventureGroupEnum::MatedPair => {
            //Two creatures of party level
            vec![party_avg; 2]
        }
        AdventureGroupEnum::Troop => {
            //One creature of party level, two creatures of party level – 2
            vec![party_avg, party_avg - 2, party_avg - 2]
        }
        AdventureGroupEnum::MookSquad => {
            //Six creatures of party level – 4
            vec![party_avg - 4; 6]
        }
    });
    result
}
