use crate::AppState;
use crate::db::bestiary_proxy;
use crate::models::bestiary_structs::{
    BestiaryFilterQuery, BestiaryPaginatedRequest, CreatureTableFieldsFilter,
};
use crate::models::creature::creature_filter_enum::CreatureFilter;
use crate::models::creature::creature_metadata::creature_role::CreatureRoleEnum;
use crate::models::creature::creature_metadata::variant_enum::CreatureVariant;
use crate::models::creature::creature_struct::Creature;
use crate::models::encounter_structs::{
    EncounterChallengeEnum, EncounterParams, RandomEncounterData,
};
use crate::models::response_data::{ResponseCreature, ResponseDataModifiers};
use crate::models::routers_validator_structs::CreatureFieldFilters;
use crate::models::shared::game_system_enum::GameSystem;
use crate::services::pf::encounter_service::get_filtered_creatures;
use crate::services::shared::encounter_calculator::{
    EncounterInfoResponse, RandomEncounterGeneratorResponse, choose_random_creatures_combination,
    get_encounter_info, get_lvl_combinations,
};
use crate::services::shared::url_calculator::bestiary_next_url;
use anyhow::{Result, ensure};
use itertools::Itertools;
use log::warn;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct BestiaryResponse {
    results: Option<Vec<ResponseCreature>>,
    count: usize,
    total: usize,
    next: Option<String>,
}

pub async fn get_creature(
    app_state: &AppState,
    id: i64,
    response_data_mods: &ResponseDataModifiers,
) -> HashMap<String, Option<ResponseCreature>> {
    hashmap! {
        String::from("results") =>
        bestiary_proxy::get_creature_by_id(app_state, &GameSystem::Pathfinder, id, CreatureVariant::Base, response_data_mods).await.map(ResponseCreature::from)
    }
}

pub async fn get_elite_creature(
    app_state: &AppState,
    id: i64,
    response_data_mods: &ResponseDataModifiers,
) -> HashMap<String, Option<ResponseCreature>> {
    hashmap! {
        String::from("results") =>
        bestiary_proxy::get_elite_creature_by_id(app_state, &GameSystem::Pathfinder, id, response_data_mods).await.map(ResponseCreature::from)
    }
}

pub async fn get_weak_creature(
    app_state: &AppState,
    id: i64,
    response_data_mods: &ResponseDataModifiers,
) -> HashMap<String, Option<ResponseCreature>> {
    hashmap! {
        String::from("results") =>
        bestiary_proxy::get_weak_creature_by_id(app_state, &GameSystem::Pathfinder, id, response_data_mods).await.map(ResponseCreature::from)
    }
}

pub async fn get_bestiary_listing(
    app_state: &AppState,
    field_filter: &CreatureFieldFilters,
    pagination: &BestiaryPaginatedRequest,
) -> BestiaryResponse {
    convert_result_to_bestiary_response(
        field_filter,
        pagination,
        bestiary_proxy::get_paginated_creatures(
            app_state,
            &GameSystem::Pathfinder,
            field_filter,
            pagination,
        )
        .await,
    )
}

pub async fn get_families_list(app_state: &AppState) -> Vec<String> {
    bestiary_proxy::get_all_possible_values_of_filter(
        app_state,
        &GameSystem::Pathfinder,
        CreatureFilter::Family,
    )
    .await
}

pub async fn get_traits_list(app_state: &AppState) -> Vec<String> {
    bestiary_proxy::get_all_possible_values_of_filter(
        app_state,
        &GameSystem::Pathfinder,
        CreatureFilter::Traits,
    )
    .await
}

pub async fn get_sources_list(app_state: &AppState) -> Vec<String> {
    bestiary_proxy::get_all_possible_values_of_filter(
        app_state,
        &GameSystem::Pathfinder,
        CreatureFilter::Sources,
    )
    .await
}

pub async fn get_rarities_list(app_state: &AppState) -> Vec<String> {
    bestiary_proxy::get_all_possible_values_of_filter(
        app_state,
        &GameSystem::Pathfinder,
        CreatureFilter::Rarity,
    )
    .await
}

pub async fn get_sizes_list(app_state: &AppState) -> Vec<String> {
    bestiary_proxy::get_all_possible_values_of_filter(
        app_state,
        &GameSystem::Pathfinder,
        CreatureFilter::Size,
    )
    .await
}

pub async fn get_alignments_list(app_state: &AppState) -> Vec<String> {
    bestiary_proxy::get_all_possible_values_of_filter(
        app_state,
        &GameSystem::Pathfinder,
        CreatureFilter::Alignment,
    )
    .await
}

pub async fn get_creature_types_list(app_state: &AppState) -> Vec<String> {
    bestiary_proxy::get_all_possible_values_of_filter(
        app_state,
        &GameSystem::Pathfinder,
        CreatureFilter::CreatureTypes,
    )
    .await
}

pub fn get_creature_roles_list() -> Vec<String> {
    CreatureRoleEnum::list()
}
fn convert_result_to_bestiary_response(
    field_filters: &CreatureFieldFilters,
    pagination: &BestiaryPaginatedRequest,
    result: Result<(u32, Vec<Creature>)>,
) -> BestiaryResponse {
    match result {
        Ok(res) => {
            let cr: Vec<Creature> = res.1;
            let cr_length = cr.len();
            BestiaryResponse {
                results: Some(cr.into_iter().map(ResponseCreature::from).collect()),
                count: cr_length,
                next: if cr_length >= pagination.paginated_request.page_size.unsigned_abs() as usize
                {
                    Some(bestiary_next_url(
                        field_filters,
                        pagination,
                        cr_length as u32,
                    ))
                } else {
                    None
                },
                total: res.0 as usize,
            }
        }
        Err(_) => BestiaryResponse {
            results: None,
            count: 0,
            total: 0,
            next: None,
        },
    }
}

pub async fn generate_random_encounter(
    app_state: &AppState,
    enc_data: RandomEncounterData,
) -> RandomEncounterGeneratorResponse {
    let party_levels = enc_data.party_levels.clone();
    let encounter_data = calculate_random_encounter(app_state, enc_data, party_levels).await;
    encounter_data.unwrap_or_else(|error| {
        warn!("Could not generate a random encounter, reason: {error}");
        RandomEncounterGeneratorResponse {
            results: None,
            count: 0,
            encounter_info: EncounterInfoResponse {
                experience: 0,
                challenge: EncounterChallengeEnum::default(),
                encounter_exp_levels: BTreeMap::default(),
            },
            game_system: GameSystem::Pathfinder,
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
    let unique_levels = filtered_lvl_combinations
        .clone()
        .into_iter()
        .flatten()
        .sorted()
        .dedup()
        .collect::<Vec<_>>();
    ensure!(
        !unique_levels.is_empty(),
        "There are no valid levels to chose from. Encounter could not be built"
    );
    let filtered_creatures = get_filtered_creatures(
        app_state,
        &BestiaryFilterQuery {
            creature_table_fields_filter: CreatureTableFieldsFilter {
                source_filter: enc_data.source_filter.unwrap_or_default(),
                family_filter: enc_data.family_filter.unwrap_or_default(),
                alignment_filter: enc_data.alignment_filter.unwrap_or_default(),
                size_filter: enc_data.size_filter.unwrap_or_default(),
                rarity_filter: enc_data.rarity_filter.unwrap_or_default(),
                type_filter: enc_data.type_filter.unwrap_or_default(),
                role_filter: enc_data.role_filter.unwrap_or_default(),
                role_lower_threshold: enc_data
                    .role_lower_threshold
                    .unwrap_or(CreatureTableFieldsFilter::default_lower_threshold()),
                role_upper_threshold: enc_data
                    .role_upper_threshold
                    .unwrap_or(CreatureTableFieldsFilter::default_upper_threshold()),
                is_melee_filter: enc_data.attack_list.as_ref().map_or_else(
                    || vec![true, false],
                    |x| vec![*x.get("melee").unwrap_or(&false)],
                ),
                is_ranged_filter: enc_data.attack_list.as_ref().map_or_else(
                    || vec![true, false],
                    |x| vec![*x.get("ranged").unwrap_or(&false)],
                ),
                is_spellcaster_filter: enc_data.attack_list.map_or_else(
                    || vec![true, false],
                    |x| vec![*x.get("spellcaster").unwrap_or(&false)],
                ),
                supported_version: enc_data
                    .game_system_version
                    .unwrap_or_default()
                    .to_db_value(),
                level_filter: unique_levels,
            },
            trait_whitelist_filter: enc_data.trait_whitelist_filter.unwrap_or_default(),
            trait_blacklist_filter: enc_data.trait_blacklist_filter.unwrap_or_default(),
        },
        enc_data.allow_weak_variants.is_some_and(|x| x),
        enc_data.allow_elite_variants.is_some_and(|x| x),
    )
    .await?;

    ensure!(
        !filtered_creatures.is_empty(),
        "No creatures have been fetched"
    );
    let chosen_encounter =
        choose_random_creatures_combination(&filtered_creatures, filtered_lvl_combinations)?;

    Ok(RandomEncounterGeneratorResponse {
        count: chosen_encounter.len(),
        results: Some(
            chosen_encounter
                .clone()
                .into_iter()
                .map(ResponseCreature::from)
                .collect(),
        ),
        encounter_info: get_encounter_info(&EncounterParams {
            party_levels,
            enemy_levels: chosen_encounter
                .iter()
                .map(|cr| cr.variant_data.level)
                .collect(),
            is_pwl_on,
        }),
        game_system: GameSystem::Pathfinder,
    })
}
