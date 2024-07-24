use crate::db::bestiary_proxy::{get_creatures_passing_all_filters, order_list_by_level};
use crate::models::creature::creature_component::filter_struct::FilterStruct;
use crate::models::creature::creature_filter_enum::CreatureFilter;
use crate::models::creature::creature_struct::Creature;
use crate::models::encounter_structs::{
    AdventureGroupEnum, EncounterChallengeEnum, EncounterParams, RandomEncounterData,
};
use crate::models::response_data::ResponseCreature;
use crate::services::encounter_handler::encounter_calculator;
use crate::services::encounter_handler::encounter_calculator::calculate_encounter_scaling_difficulty;
use crate::AppState;
use anyhow::{ensure, Result};
use counter::Counter;
use log::warn;
use rand::seq::IndexedRandom;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct EncounterInfoResponse {
    experience: i64,
    challenge: EncounterChallengeEnum,
    encounter_exp_levels: BTreeMap<EncounterChallengeEnum, i64>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct RandomEncounterGeneratorResponse {
    results: Option<Vec<ResponseCreature>>,
    count: usize,
    encounter_info: EncounterInfoResponse,
}

pub fn get_encounter_info(enc_params: EncounterParams) -> EncounterInfoResponse {
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

pub async fn generate_random_encounter(
    app_state: &AppState,
    enc_data: RandomEncounterData,
) -> RandomEncounterGeneratorResponse {
    let party_levels = enc_data.party_levels.clone();
    let encounter_data = calculate_random_encounter(app_state, enc_data, party_levels).await;
    encounter_data.unwrap_or_else(|error| {
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
    })
}

/// Private method, does not handle failure. For that we use a public method
async fn calculate_random_encounter(
    app_state: &AppState,
    enc_data: RandomEncounterData,
    party_levels: Vec<i64>,
) -> Result<RandomEncounterGeneratorResponse> {
    let is_pwl_on = enc_data.is_pwl_on;
    let filtered_lvl_combinations = get_lvl_combinations(&enc_data, &party_levels);
    let unique_levels = HashSet::from_iter(
        filtered_lvl_combinations
            .clone()
            .into_iter()
            .flatten()
            .map(|lvl| lvl.to_string()),
    );
    ensure!(
        !unique_levels.is_empty(),
        "There are no valid levels to chose from. Encounter could not be built"
    );
    let filter_map = build_filter_map(FilterStruct {
        families: enc_data.families,
        traits: enc_data.traits,
        rarities: enc_data.rarities,
        sizes: enc_data.sizes,
        alignments: enc_data.alignments,
        creature_types: enc_data.creature_types,
        creature_roles: enc_data.creature_roles,
        lvl_combinations: unique_levels,
        pathfinder_version: enc_data.pathfinder_version.unwrap_or_default(),
    });

    let filtered_creatures = get_filtered_creatures(
        app_state,
        filter_map,
        enc_data.allow_weak_variants.is_some_and(|x| x),
        enc_data.allow_elite_variants.is_some_and(|x| x),
    )
    .await?;

    ensure!(
        !filtered_creatures.is_empty(),
        "No creatures have been fetched"
    );
    let chosen_encounter =
        choose_random_creatures_combination(filtered_creatures, filtered_lvl_combinations)?;

    Ok(RandomEncounterGeneratorResponse {
        count: chosen_encounter.len(),
        results: Some(
            chosen_encounter
                .clone()
                .into_iter()
                .map(ResponseCreature::from)
                .collect(),
        ),
        encounter_info: get_encounter_info(EncounterParams {
            party_levels,
            enemy_levels: chosen_encounter
                .iter()
                .map(|cr| cr.variant_data.level)
                .collect(),
            is_pwl_on,
        }),
    })
}

fn choose_random_creatures_combination(
    filtered_creatures: Vec<Creature>,
    lvl_combinations: HashSet<Vec<i64>>,
) -> Result<Vec<Creature>> {
    // Chooses an id combination, could be (1, 1, 2). Admits duplicates
    let creatures_ordered_by_level = order_list_by_level(filtered_creatures);
    let mut list_of_levels = Vec::new();
    creatures_ordered_by_level
        .keys()
        .for_each(|key| list_of_levels.push(*key));
    let existing_levels = filter_non_existing_levels(list_of_levels, lvl_combinations);
    let tmp = Vec::from_iter(existing_levels.iter());
    let random_combo = tmp[rand::thread_rng().gen_range(0..tmp.len())];
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
            // I could do choose multiples but it does not allow repetition
            // this is bad because i increase the probability of the same one getting picked
            // example [A,B] => [A,B,A] => [A,B,A,A] etc
            lvl_vec.push(lvl_vec.choose(&mut rand::thread_rng()).unwrap().clone());
        }
    }
    Ok(lvl_vec)
}

fn filter_non_existing_levels(
    creatures_levels: Vec<i64>,
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

fn build_filter_map(filter_enum: FilterStruct) -> HashMap<CreatureFilter, HashSet<String>> {
    let mut filter_map = HashMap::new();

    filter_enum
        .families
        .map(|el| filter_map.insert(CreatureFilter::Family, HashSet::from_iter(el)));
    filter_enum
        .traits
        .map(|el| filter_map.insert(CreatureFilter::Traits, HashSet::from_iter(el)));
    // What no generic enum does to a mf (mother function)
    // it could also prob be bad programming by me
    if let Some(vec) = filter_enum.rarities {
        filter_map.insert(
            CreatureFilter::Rarity,
            HashSet::from_iter(vec.iter().map(|el| el.to_string())),
        );
    };
    if let Some(vec) = filter_enum.sizes {
        filter_map.insert(
            CreatureFilter::Size,
            HashSet::from_iter(vec.iter().map(|el| el.to_string())),
        );
    };
    if let Some(vec) = filter_enum.alignments {
        filter_map.insert(
            CreatureFilter::Alignment,
            HashSet::from_iter(vec.iter().map(|x| x.to_string())),
        );
    };
    if let Some(vec) = filter_enum.creature_types {
        filter_map.insert(
            CreatureFilter::CreatureTypes,
            HashSet::from_iter(vec.iter().map(|el| el.to_string())),
        );
    };
    if let Some(vec) = filter_enum.creature_roles {
        filter_map.insert(
            CreatureFilter::CreatureRoles,
            HashSet::from_iter(vec.iter().map(|el| el.to_db_column())),
        );
    };
    filter_map.insert(CreatureFilter::Level, filter_enum.lvl_combinations);
    filter_map.insert(
        CreatureFilter::PathfinderVersion,
        HashSet::from_iter(filter_enum.pathfinder_version.to_db_value()),
    );
    filter_map
}

async fn get_filtered_creatures(
    app_state: &AppState,
    filter_map: HashMap<CreatureFilter, HashSet<String>>,
    allow_weak: bool,
    allow_elite: bool,
) -> Result<Vec<Creature>> {
    get_creatures_passing_all_filters(app_state, filter_map, allow_weak, allow_elite).await
}

fn get_lvl_combinations(enc_data: &RandomEncounterData, party_levels: &[i64]) -> HashSet<Vec<i64>> {
    if let Some(adv_group) = enc_data.adventure_group.as_ref() {
        get_adventure_group_lvl_combinations(adv_group, party_levels)
    } else {
        get_standard_lvl_combinations(enc_data, party_levels)
    }
}

fn get_standard_lvl_combinations(
    enc_data: &RandomEncounterData,
    party_levels: &[i64],
) -> HashSet<Vec<i64>> {
    let enc_diff = enc_data.challenge.clone().unwrap_or(rand::random());
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
    let party_avg = party_levels.iter().sum::<i64>() / party_levels.len() as i64;
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
