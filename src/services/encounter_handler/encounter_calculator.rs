use crate::db::bestiary_proxy::order_list_by_level;
use crate::models::encounter_structs::{
    AdventureGroupEnum, EncounterChallengeEnum, EncounterParams, ExpRange,
};
use crate::models::hazard::hazard_field_filter::HazardComplexityEnum;
use crate::models::response_data::{ResponseCreature, ResponseHazard};
use crate::models::shared::game_system_enum::GameSystem;
use crate::services::encounter_handler::difficulty_utilities::scale_difficulty_exp;
use crate::services::encounter_handler::encounter_math;
use crate::services::encounter_handler::encounter_math::calculate_encounter_scaling_difficulty;
use crate::traits::has_level::HasLevel;
use anyhow::{Result, ensure};
use counter::Counter;
use nanorand::{Rng, WyRand};
use serde::{Deserialize, Serialize};
#[allow(unused_imports)] // it's used for Schema
use serde_json::json;
use std::collections::{BTreeMap, HashSet};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct EncounterInfoResponse {
    #[schema(minimum = 0, example = 40)]
    pub(crate) experience: i64,
    pub(crate) challenge: EncounterChallengeEnum,
    #[schema(example = json!({EncounterChallengeEnum::Trivial: 40, EncounterChallengeEnum::Low: 60, EncounterChallengeEnum::Moderate: 80, EncounterChallengeEnum::Severe: 120, EncounterChallengeEnum::Extreme: 160, EncounterChallengeEnum::Impossible: 320}))]
    pub(crate) encounter_exp_levels: BTreeMap<EncounterChallengeEnum, i64>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct EncounterContent {
    pub(crate) creatures: Option<Vec<ResponseCreature>>,
    pub(crate) hazards: Option<Vec<ResponseHazard>>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct RandomEncounterGeneratorResponse {
    pub(crate) results: EncounterContent,
    pub(crate) count: usize,
    pub(crate) encounter_info: EncounterInfoResponse,
    pub(crate) game: GameSystem,
}

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

pub fn choose_random_combination<T>(
    elements_used_to_filter_lvl_combinations: &[T],
    lvl_combinations: HashSet<Vec<i64>>,
) -> Result<Vec<T>>
where
    T: HasLevel + Clone,
{
    let items_ordered_by_level = order_list_by_level(elements_used_to_filter_lvl_combinations);
    let list_of_levels = items_ordered_by_level.keys().copied().collect::<Vec<_>>();

    let existing_levels = filter_non_existing_levels(&list_of_levels, lvl_combinations);
    let tmp = existing_levels.iter().collect::<Vec<_>>();
    ensure!(
        !tmp.is_empty(),
        "No valid level combinations to randomly choose from"
    );

    // do not remove ensure. the random picker will panic if tmp is empty
    let random_combo = tmp[WyRand::new().generate_range(..tmp.len())];
    let level_count = random_combo.iter().collect::<Counter<_>>();
    // Now, having chosen the combo, we may have only x filtered creature with level y but
    // x+1 instances of level y. We need to create a vector with duplicates to fill it up to
    // the number of instances of the required level

    let mut result_vec: Vec<T> = Vec::new();
    for (level, required_count) in level_count {
        let curr_lvl_values = items_ordered_by_level.get(level).unwrap();
        let mut filled =
            fill_vector_if_it_does_not_contain_enough_elements(curr_lvl_values, required_count)?;
        WyRand::new().shuffle(&mut filled);
        result_vec.extend(filled.into_iter().take(required_count));
    }

    Ok(result_vec)
}

fn fill_vector_if_it_does_not_contain_enough_elements<T: Clone>(
    elements: &[T],
    n_of_required_elements: usize,
) -> Result<Vec<T>> {
    ensure!(!elements.is_empty(), "No elements for the chosen level");
    let mut vec = elements.to_vec();
    while vec.len() < n_of_required_elements {
        vec.push(
            // We could do choose multiples, but it does not allow repetition
            // this is bad because it increases the probability of the same one getting picked
            // example [A,B] => [A,B,A] => [A,B,A,A] etc
            vec.get(WyRand::new().generate_range(..vec.len()))
                .unwrap()
                .clone(),
        );
    }
    Ok(vec)
}

fn filter_non_existing_levels(
    existing_levels: &[i64],
    level_combinations: HashSet<Vec<i64>>,
) -> HashSet<Vec<i64>> {
    let mut result_vec = HashSet::new();
    for curr_combo in level_combinations {
        if !curr_combo.is_empty() && curr_combo.iter().all(|lvl| existing_levels.contains(lvl)) {
            result_vec.insert(curr_combo);
        }
    }
    result_vec
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
