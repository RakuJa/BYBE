use crate::db::db_proxy::{fetch_creatures_passing_all_filters, order_list_by_level};
use crate::models::creature::Creature;
use crate::models::creature_filter_enum::CreatureFilter;
use crate::models::creature_metadata_enums::{AlignmentEnum, RarityEnum, SizeEnum};
use crate::models::encounter_structs::{EncounterChallengeEnum, EncounterParams};
use crate::models::routers_validator_structs::RandomEncounterData;
use crate::services::encounter_handler::encounter_calculator;
use crate::services::encounter_handler::encounter_calculator::calculate_encounter_scaling_difficulty;
use anyhow::{ensure, Result};
use counter::Counter;
use log::warn;
use rand::seq::SliceRandom;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct EncounterInfoResponse {
    experience: i16,
    challenge: EncounterChallengeEnum,
    encounter_exp_levels: HashMap<EncounterChallengeEnum, i16>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct RandomEncounterGeneratorResponse {
    results: Option<Vec<Creature>>,
    count: usize,
    encounter_info: EncounterInfoResponse,
}

pub fn get_encounter_info(enc_params: EncounterParams) -> EncounterInfoResponse {
    let enc_exp = encounter_calculator::calculate_encounter_exp(
        &enc_params.party_levels,
        &enc_params.enemy_levels,
    );

    let scaled_exp = calculate_encounter_scaling_difficulty(enc_params.party_levels.len());

    let enc_diff = encounter_calculator::calculate_encounter_difficulty(enc_exp, &scaled_exp);
    EncounterInfoResponse {
        experience: enc_exp,
        challenge: enc_diff,
        encounter_exp_levels: scaled_exp,
    }
}

pub fn generate_random_encounter(
    enc_data: RandomEncounterData,
    party_levels: Vec<i16>,
) -> RandomEncounterGeneratorResponse {
    let encounter_data = calculate_random_encounter(enc_data, party_levels);
    match encounter_data {
        Err(error) => {
            warn!(
                "Could not generate a random encounter, reason: {}",
                error.to_string()
            );
            RandomEncounterGeneratorResponse {
                results: None,
                count: 0,
                encounter_info: EncounterInfoResponse {
                    experience: 0,
                    challenge: Default::default(),
                    encounter_exp_levels: Default::default(),
                },
            }
        }
        Ok(enc_data) => enc_data,
    }
}

// Private method, does not handle failure. For that we use a public method
fn calculate_random_encounter(
    enc_data: RandomEncounterData,
    party_levels: Vec<i16>,
) -> Result<RandomEncounterGeneratorResponse> {
    let enc_diff = enc_data.encounter_challenge.unwrap_or(rand::random());

    let (exp, lvl_combinations) =
        encounter_calculator::calculate_lvl_combination_for_encounter(&enc_diff, &party_levels);
    let unique_levels = HashSet::from_iter(
        lvl_combinations
            .clone()
            .into_iter()
            .flatten()
            .map(|lvl| lvl.to_string()),
    );
    ensure!(
        !unique_levels.is_empty(),
        "There are no valid levels to chose from. Encounter could not be built"
    );
    let filter_map = build_filter_map(
        enc_data.family,
        enc_data.rarity,
        enc_data.size,
        enc_data.alignment,
        unique_levels,
    );
    let filtered_creatures = fetch_creatures_passing_all_filters(filter_map)?;
    ensure!(
        !filtered_creatures.is_empty(),
        "No creatures have been fetched"
    );
    let chosen_encounter =
        choose_random_creatures_combination(filtered_creatures, lvl_combinations)?;

    let scaled_exp_levels = calculate_encounter_scaling_difficulty(party_levels.len());

    Ok(RandomEncounterGeneratorResponse {
        count: chosen_encounter.len(),
        results: Some(chosen_encounter),
        encounter_info: EncounterInfoResponse {
            experience: exp,
            challenge: enc_diff,
            encounter_exp_levels: scaled_exp_levels,
        },
    })
}

fn choose_random_creatures_combination(
    filtered_creatures: HashSet<Creature>,
    lvl_combinations: HashSet<Vec<i16>>,
) -> Result<Vec<Creature>> {
    // Chooses an id combination, could be (1, 1, 2). Admits duplicates
    let creatures_ordered_by_level = order_list_by_level(filtered_creatures);
    let mut list_of_levels: Vec<i16> = Vec::new();
    creatures_ordered_by_level
        .keys()
        .for_each(|key| list_of_levels.push(*key));
    //let list_of_levels: Vec<i16> = filtered_creatures
    //  .iter()
    //.map(|curr_creature| curr_creature.level)
    //.collect();
    let existing_levels = filter_non_existing_levels(list_of_levels, lvl_combinations);
    let tmp = Vec::from_iter(existing_levels.iter());
    let random_combo = tmp[rand::thread_rng().gen_range(0..tmp.len())];
    // let random_combo = tmp.choose(&mut rand::thread_rng());
    // Now, having chosen the combo, we may have only x filtered creature with level y but
    // x+1 instances of level y. We need to create a vector with duplicates to fill it up to
    // the number of instances of the required level
    ensure!(!random_combo.is_empty(), "No valid combo found");
    let creature_count = random_combo.iter().collect::<Counter<_>>();
    let mut result_vec: Vec<Creature> = Vec::new();
    for (level, required_number_of_creatures_with_level) in creature_count {
        // Fill if there are not enough creatures
        let curr_lvl_values = creatures_ordered_by_level.get(level).unwrap();
        let filled_vec_of_creatures = fill_vector_if_it_does_not_contain_enough_elements(
            curr_lvl_values,
            required_number_of_creatures_with_level,
        )?;
        // Now, we choose. This is in case that there are more creatures
        for curr_chosen_creature in filled_vec_of_creatures.choose_multiple(
            &mut rand::thread_rng(),
            required_number_of_creatures_with_level,
        ) {
            result_vec.push(curr_chosen_creature.clone())
        }
    }
    Ok(result_vec)
}

fn fill_vector_if_it_does_not_contain_enough_elements(
    curr_lvl_values: &Vec<Creature>,
    required_number_of_creatures_with_level: usize,
) -> Result<Vec<Creature>> {
    ensure!(
        !curr_lvl_values.is_empty(),
        "No creatures for the chosen level"
    );
    let mut lvl_vec = curr_lvl_values.clone();
    let creature_with_required_level = lvl_vec.len();
    if creature_with_required_level < required_number_of_creatures_with_level {
        while lvl_vec.len() < required_number_of_creatures_with_level {
            // I could do choose multiples but it does not allow repetition
            // this is bad because i increase the probability of the same one getting picked
            // example [A,B] => [A,B,A] => [A,B,A,A] etc
            lvl_vec.push(lvl_vec.choose(&mut rand::thread_rng()).unwrap().clone());
        }
    }
    Ok(lvl_vec)
}

fn filter_non_existing_levels(
    creatures_levels: Vec<i16>,
    level_combinations: HashSet<Vec<i16>>,
) -> HashSet<Vec<i16>> {
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

fn build_filter_map(
    family: Option<String>,
    rarity: Option<RarityEnum>,
    size: Option<SizeEnum>,
    alignment: Option<AlignmentEnum>,
    lvl_combinations: HashSet<String>,
) -> HashMap<CreatureFilter, HashSet<String>> {
    let mut filter_map = HashMap::new();
    family.map(|el| filter_map.insert(CreatureFilter::Family, hashset![el]));
    rarity.map(|el| filter_map.insert(CreatureFilter::Rarity, hashset![el.to_string()]));
    size.map(|el| filter_map.insert(CreatureFilter::Size, hashset![el.to_string()]));
    alignment.map(|el| filter_map.insert(CreatureFilter::Alignment, hashset![el.to_string()]));
    filter_map.insert(CreatureFilter::Level, lvl_combinations);
    filter_map
}
