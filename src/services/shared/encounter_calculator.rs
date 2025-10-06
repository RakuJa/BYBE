use crate::db::bestiary_proxy::order_list_by_level;
use crate::models::creature::creature_struct::Creature;
use crate::models::encounter_structs::{
    AdventureGroupEnum, EncounterChallengeEnum, EncounterParams, RandomEncounterData,
};
use crate::models::response_data::ResponseCreature;
use crate::models::shared::game_system_enum::GameSystem;
use crate::services::encounter_handler::encounter_calculator;
use crate::services::encounter_handler::encounter_calculator::calculate_encounter_scaling_difficulty;
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
pub struct RandomEncounterGeneratorResponse {
    pub(crate) results: Option<Vec<ResponseCreature>>,
    pub(crate) count: usize,
    pub(crate) encounter_info: EncounterInfoResponse,
    pub(crate) game_system: GameSystem,
}

pub fn get_encounter_info(enc_params: &EncounterParams) -> EncounterInfoResponse {
    let enc_exp = encounter_calculator::calculate_encounter_exp(
        &enc_params.party_levels,
        &enc_params.enemy_levels,
        enc_params.is_pwl_on,
    );

    let scaled_exp = calculate_encounter_scaling_difficulty(enc_params.party_levels.len());

    let enc_diff = encounter_calculator::calculate_encounter_difficulty(enc_exp, &scaled_exp);
    EncounterInfoResponse {
        experience: enc_exp,
        challenge: enc_diff,
        encounter_exp_levels: scaled_exp.into_iter().collect(),
    }
}

pub fn choose_random_creatures_combination(
    filtered_creatures: &[Creature],
    lvl_combinations: HashSet<Vec<i64>>,
) -> Result<Vec<Creature>> {
    // Chooses an id combination, could be (1, 1, 2). Admits duplicates
    let creatures_ordered_by_level = order_list_by_level(filtered_creatures);
    let mut list_of_levels = Vec::new();
    for key in creatures_ordered_by_level.keys() {
        list_of_levels.push(*key);
    }
    let existing_levels = filter_non_existing_levels(&list_of_levels, lvl_combinations);
    let tmp = existing_levels.iter().collect::<Vec<_>>();
    ensure!(
        !tmp.is_empty(),
        "No valid level combinations to randomly choose from"
    );
    // do not remove ensure. the random picker will panic if tmp is empty
    let random_combo = tmp[WyRand::new().generate_range(..tmp.len())];
    // Now, having chosen the combo, we may have only x filtered creature with level y but
    // x+1 instances of level y. We need to create a vector with duplicates to fill it up to
    // the number of instances of the required level

    let creature_count = random_combo.iter().collect::<Counter<_>>();
    let mut result_vec: Vec<Creature> = Vec::new();
    for (level, required_number_of_creatures_with_level) in creature_count {
        // Fill if there are not enough creatures
        let curr_lvl_values = creatures_ordered_by_level.get(level).unwrap();
        let mut filled_vec_of_creatures = fill_vector_if_it_does_not_contain_enough_elements(
            curr_lvl_values,
            required_number_of_creatures_with_level,
        )?;
        // Now, we choose. This is in case that there are more creatures
        WyRand::new().shuffle(&mut filled_vec_of_creatures);
        for curr_chosen_creature in filled_vec_of_creatures
            .iter()
            .take(required_number_of_creatures_with_level)
        {
            result_vec.push(curr_chosen_creature.clone());
        }
    }
    Ok(result_vec)
}

fn fill_vector_if_it_does_not_contain_enough_elements(
    curr_lvl_values: &[Creature],
    required_number_of_creatures_with_level: usize,
) -> Result<Vec<Creature>> {
    ensure!(
        !curr_lvl_values.is_empty(),
        "No creatures for the chosen level"
    );
    let mut lvl_vec = curr_lvl_values.to_vec();
    let creature_with_required_level = lvl_vec.len();
    if creature_with_required_level < required_number_of_creatures_with_level {
        while lvl_vec.len() < required_number_of_creatures_with_level {
            // We could do choose multiples, but it does not allow repetition
            // this is bad because it increases the probability of the same one getting picked
            // example [A,B] => [A,B,A] => [A,B,A,A] etc
            lvl_vec.push(
                lvl_vec
                    .get(WyRand::new().generate_range(..lvl_vec.len()))
                    .unwrap()
                    .clone(),
            );
        }
    }
    Ok(lvl_vec)
}

fn filter_non_existing_levels(
    creatures_levels: &[i64],
    level_combinations: HashSet<Vec<i64>>,
) -> HashSet<Vec<i64>> {
    // Removes the vec with levels that are not found in any creature
    let mut result_vec = HashSet::new();
    for curr_combo in level_combinations {
        if !curr_combo.is_empty() && curr_combo.iter().all(|lvl| creatures_levels.contains(lvl)) {
            // Check if there is one of the curr_combo level that is not found in the creature.level field
            result_vec.insert(curr_combo);
        }
    }
    result_vec
}

pub fn get_lvl_combinations(
    enc_data: &RandomEncounterData,
    party_levels: &[i64],
) -> HashSet<Vec<i64>> {
    enc_data.adventure_group.as_ref().map_or_else(
        || get_standard_lvl_combinations(enc_data, party_levels),
        |adv_group| get_adventure_group_lvl_combinations(adv_group, party_levels),
    )
}

fn get_standard_lvl_combinations(
    enc_data: &RandomEncounterData,
    party_levels: &[i64],
) -> HashSet<Vec<i64>> {
    let enc_diff = enc_data
        .challenge
        .clone()
        .unwrap_or_else(EncounterChallengeEnum::rand);
    let lvl_combinations = encounter_calculator::calculate_lvl_combination_for_encounter(
        &enc_diff,
        party_levels,
        enc_data.is_pwl_on,
    );
    encounter_calculator::filter_combinations_outside_range(
        lvl_combinations,
        enc_data.min_creatures,
        enc_data.max_creatures,
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
