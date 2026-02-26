use crate::db::bestiary_proxy::order_list_by_level;
use crate::models::encounter_structs::{
    AdventureGroupEnum, EncounterChallengeEnum, EncounterParams, ExpRange,
};
use crate::models::hazard::hazard_field_filter::HazardComplexityEnum;
use crate::models::response_data::EncounterInfoResponse;
use crate::services::encounter_handler::difficulty_utilities::scale_difficulty_exp;
use crate::services::encounter_handler::encounter_math;
use crate::services::encounter_handler::encounter_math::calculate_encounter_scaling_difficulty;
use crate::traits::has_level::HasLevel;
use anyhow::{Result, ensure};
use counter::Counter;
use nanorand::{Rng, WyRand};
use std::collections::HashSet;

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
    let existing_levels: Vec<_> = lvl_combinations
        .into_iter()
        .filter(|combo| !combo.is_empty() && combo.iter().all(|lvl| list_of_levels.contains(lvl)))
        .collect();
    let tmp = existing_levels;
    ensure!(
        !tmp.is_empty(),
        "No valid level combinations to randomly choose from"
    );
    let mut rng = WyRand::new();

    // do not remove ensure. the random picker will panic if tmp is empty
    let random_combo = &tmp[rng.generate_range(..tmp.len())];
    let level_count = random_combo.iter().collect::<Counter<_>>();
    // Now, having chosen the combo, we may have only x filtered creature with level y but
    // x+1 instances of level y. We need to create a vector with duplicates to fill it up to
    // the number of instances of the required level

    let mut result_vec: Vec<T> = Vec::new();
    for (level, required_count) in level_count {
        let curr_lvl_values = items_ordered_by_level.get(level).unwrap();
        ensure!(
            !curr_lvl_values.is_empty(),
            "No elements for the chosen level"
        );
        let mut filled: Vec<_> = curr_lvl_values
            .iter()
            .cycle()
            .take(required_count)
            .cloned()
            .collect();
        rng.shuffle(&mut filled);
        result_vec.extend(filled.into_iter().take(required_count));
    }

    Ok(result_vec)
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
