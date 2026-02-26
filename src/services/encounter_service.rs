use crate::AppState;
use crate::models::bestiary_structs::{BestiaryFilterQuery, CreatureTableFieldsFilter};
use crate::models::encounter_structs::{
    CreatureEncounterParams, EncounterChallengeEnum, EncounterParams, ExpRange,
    HazardEncounterElement, HazardEncounterParams, RandomCreatureData, RandomEncounterData,
    RandomHazardData,
};
use crate::models::hazard::hazard_listing_struct::{HazardFilterQuery, HazardTableFieldsFilter};
use crate::models::response_data::{
    EncounterContent, EncounterInfoResponse, RandomEncounterGeneratorResponse, ResponseCreature,
    ResponseHazard,
};
use crate::models::shared::game_system_enum::GameSystem;
use crate::services::bestiary_service::get_filtered_creatures;
use crate::services::encounter_handler::encounter_calculator::{
    choose_random_combination, get_creature_lvl_combinations, get_encounter_info,
    get_hazard_lvl_combinations, get_scaled_exp,
};
use crate::services::hazard_service::get_filtered_hazards;
use anyhow::{bail, ensure};
use itertools::Itertools;
use std::collections::{BTreeMap, HashSet};
use tracing::warn;

#[derive(Debug)]
struct RandomCreatureGeneratorResponse {
    results: Option<Vec<ResponseCreature>>,
    count: usize,
}

#[derive(Debug)]
struct RandomHazardGeneratorResponse {
    results: Option<Vec<ResponseHazard>>,
    count: usize,
}

pub async fn generate_random_encounter(
    app_state: &AppState,
    enc_data: RandomEncounterData,
    gs: &GameSystem,
) -> RandomEncounterGeneratorResponse {
    let encounter_data = calculate_random_encounter(app_state, enc_data, gs).await;
    encounter_data.unwrap_or_else(|error| {
        warn!("Could not generate a random encounter, reason: {error}");
        RandomEncounterGeneratorResponse {
            results: EncounterContent {
                creatures: None,
                hazards: None,
            },
            count: 0,
            encounter_info: EncounterInfoResponse {
                experience: 0,
                challenge: EncounterChallengeEnum::default(),
                encounter_exp_levels: BTreeMap::default(),
            },
            game: *gs,
        }
    })
}

async fn calculate_random_creature_encounter(
    app_state: &AppState,
    enc_data: RandomCreatureData,
    party_levels: &[i64],
    exp_range: ExpRange,
    gs: &GameSystem,
) -> anyhow::Result<RandomCreatureGeneratorResponse> {
    let is_pwl_on = enc_data.is_pwl_on;

    let filtered_lvl_combinations = get_creature_lvl_combinations(
        party_levels,
        exp_range,
        is_pwl_on,
        enc_data.min_creatures,
        enc_data.max_creatures,
        enc_data.adventure_group,
    );
    let list_of_unique_levels = filtered_lvl_combinations
        .clone()
        .into_iter()
        .flatten()
        .sorted()
        .dedup()
        .collect::<Vec<_>>();
    ensure!(
        !list_of_unique_levels.is_empty(),
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
                level_filter: list_of_unique_levels,
            },
            trait_whitelist_filter: enc_data.trait_whitelist_filter.unwrap_or_default(),
            trait_blacklist_filter: enc_data.trait_blacklist_filter.unwrap_or_default(),
        },
        enc_data.allow_weak_variants.is_some_and(|x| x),
        enc_data.allow_elite_variants.is_some_and(|x| x),
        gs,
    )
    .await?;

    ensure!(
        !filtered_creatures.is_empty(),
        "No creatures have been fetched"
    );
    let chosen_encounter =
        choose_random_combination(&filtered_creatures, filtered_lvl_combinations)?;

    Ok(RandomCreatureGeneratorResponse {
        count: chosen_encounter.len(),
        results: Some(
            chosen_encounter
                .into_iter()
                .map(ResponseCreature::from)
                .collect(),
        ),
    })
}

async fn calculate_random_hazard_encounter(
    app_state: &AppState,
    enc_data: RandomHazardData,
    party_levels: &[i64],
    exp_range: ExpRange,
    gs: &GameSystem,
) -> anyhow::Result<RandomHazardGeneratorResponse> {
    let filtered_lvl_combinations = get_hazard_lvl_combinations(
        party_levels,
        exp_range,
        enc_data.hazard_complexity.unwrap_or_default(),
        enc_data.min_hazards,
        enc_data.max_hazards,
    );
    let list_of_unique_levels = filtered_lvl_combinations
        .clone()
        .into_iter()
        .flatten()
        .sorted()
        .dedup()
        .collect::<Vec<_>>();
    ensure!(
        !list_of_unique_levels.is_empty(),
        "There are no valid levels to chose from. Encounter could not be built"
    );
    let filtered_hazards = get_filtered_hazards(
        app_state,
        &HazardFilterQuery {
            hazard_table_fields_filter: HazardTableFieldsFilter {
                source_filter: enc_data.source_filter.unwrap_or_default(),
                rarity_filter: enc_data.rarity_filter.unwrap_or_default(),
                size_filter: enc_data.size_filter.unwrap_or_default(),
                supported_version: enc_data
                    .game_system_version
                    .unwrap_or_default()
                    .to_db_value(),
                level_filter: list_of_unique_levels,
                min_ac: enc_data.min_ac,
                max_ac: enc_data.max_ac,
                min_hardness: enc_data.min_hardness,
                max_hardness: enc_data.max_hardness,
                min_hp: enc_data.min_hp,
                max_hp: enc_data.max_hp,
                min_will: enc_data.min_will,
                max_will: enc_data.max_will,
                min_reflex: enc_data.min_reflex,
                max_reflex: enc_data.max_reflex,
                min_fortitude: enc_data.min_fortitude,
                max_fortitude: enc_data.max_fortitude,
            },
            trait_whitelist_filter: enc_data.trait_whitelist_filter.unwrap_or_default(),
            trait_blacklist_filter: enc_data.trait_blacklist_filter.unwrap_or_default(),
        },
        gs,
    )
    .await?;

    let filtered_levels: HashSet<Vec<i64>> = filtered_lvl_combinations
        .into_iter()
        .map(|combo| combo.into_iter().map(|c| c.1).collect())
        .collect();

    ensure!(!filtered_hazards.is_empty(), "No hazards have been fetched");
    let chosen_encounter = choose_random_combination(&filtered_hazards, filtered_levels)?;

    Ok(RandomHazardGeneratorResponse {
        count: chosen_encounter.len(),
        results: Some(
            chosen_encounter
                .into_iter()
                .map(|x| ResponseHazard::from((x, *gs)))
                .collect(),
        ),
    })
}

/// Private method, does not handle failure. For that we use a public method.
/// It calculates both a random creature list w.r.t. cr encounter data and
/// a random hazard list w.r.t. hazard encounter data. It prepares data,
/// splitting exp between encounter and hazard and then
/// calls the standalone method for each generation
async fn calculate_random_encounter(
    app_state: &AppState,
    enc_data: RandomEncounterData,
    gs: &GameSystem,
) -> anyhow::Result<RandomEncounterGeneratorResponse> {
    let cr_encounter_data = enc_data
        .creature_data
        .ok_or_else(|| anyhow::anyhow!("creature_data is required"))?;
    let hz_encounter_data = enc_data
        .hazard_data
        .ok_or_else(|| anyhow::anyhow!("hazard_data is required"))?;

    let is_pwl_on = cr_encounter_data.is_pwl_on;

    let party_len = i64::try_from(enc_data.party_levels.len()).unwrap_or(i64::MAX);
    let cr_percentage = enc_data.creature_percentage.unwrap_or(100) as i64;
    let hz_percentage = enc_data
        .hazard_percentage
        .unwrap_or(100 - cr_percentage as u8) as i64;

    let challenge = cr_encounter_data
        .challenge
        .unwrap_or_else(EncounterChallengeEnum::rand);
    let exp_range = get_scaled_exp(challenge, party_len);

    let scale = |pct: i64| ExpRange {
        lower_bound: exp_range.lower_bound * pct / 100,
        upper_bound: exp_range.upper_bound * pct / 100,
    };
    println!("creature exp range: {:?}", scale(cr_percentage));
    println!("hazard exp range: {:?}", scale(hz_percentage));
    let (creature_result, hazard_result) = tokio::join!(
        calculate_random_creature_encounter(
            app_state,
            cr_encounter_data,
            &enc_data.party_levels,
            scale(cr_percentage),
            gs
        ),
        calculate_random_hazard_encounter(
            app_state,
            hz_encounter_data,
            &enc_data.party_levels,
            scale(hz_percentage),
            gs
        ),
    );

    let (creatures, cr_count) = creature_result
        .inspect_err(|e| println!("Failed to calculate create encounter: {e}"))
        .map_or((None, 0), |e| (e.results, e.count));

    let (hazards, hz_count) = hazard_result
        .inspect_err(|e| println!("Failed to calculate hazard encounter: {e}"))
        .map_or((None, 0), |e| (e.results, e.count));

    if creatures.is_none() && hazards.is_none() {
        bail!("Both creature and hazard encounter failed to generate");
    }

    Ok(RandomEncounterGeneratorResponse {
        count: cr_count + hz_count,
        encounter_info: get_encounter_info(&EncounterParams {
            party_levels: enc_data.party_levels,
            creatures_params: Some(CreatureEncounterParams {
                enemy_levels: creatures
                    .clone()
                    .unwrap_or_default()
                    .iter()
                    .map(|cr| cr.variant_data.level)
                    .collect(),
                is_pwl_on,
            }),
            hazards_params: Some(HazardEncounterParams {
                hazards: hazards
                    .clone()
                    .unwrap_or_default()
                    .iter()
                    .map(|x| HazardEncounterElement {
                        complexity: x.core_hazard.essential.kind,
                        level: x.core_hazard.essential.level,
                    })
                    .collect(),
            }),
        }),
        game: *gs,
        results: EncounterContent { creatures, hazards },
    })
}
