use crate::models::creature::creature_struct::Creature;
use std::collections::HashMap;

use crate::AppState;
use crate::db::data_providers::creature_fetcher::fetch_traits_associated_with_creatures;
use crate::db::data_providers::{creature_fetcher, generic_fetcher};
use crate::models::bestiary_structs::{
    BestiaryFilterQuery, BestiaryPaginatedRequest, CreatureSortEnum,
};
use crate::models::creature::creature_component::creature_core::CreatureCoreData;
use crate::models::creature::creature_field_filter::CreatureFieldFilters;
use crate::models::creature::creature_filter_enum::{CreatureFilter, FieldsUniqueValuesStruct};
use crate::models::creature::creature_metadata::creature_role::CreatureRoleEnum;
use crate::models::creature::creature_metadata::type_enum::CreatureTypeEnum;
use crate::models::creature::creature_metadata::variant_enum::CreatureVariant;
use crate::models::response_data::CreatureResponseDataModifiers;
use crate::models::routers_validator_structs::OrderEnum;
use crate::models::shared::alignment_enum::AlignmentEnum;
use crate::models::shared::game_system_enum::GameSystem;
use crate::models::shared::pf_version_enum::GameSystemVersionEnum;
use crate::traits::has_level::HasLevel;
use anyhow::Result;
use cached::proc_macro::cached;
use itertools::Itertools;
use strum::IntoEnumIterator;

pub async fn get_creature_by_id(
    app_state: &AppState,
    gs: &GameSystem,
    id: i64,
    variant: CreatureVariant,
    response_data_mods: &CreatureResponseDataModifiers,
) -> Option<Creature> {
    creature_fetcher::fetch_creature_by_id(&app_state.conn, gs, variant, response_data_mods, id)
        .await
        .ok()
}

pub async fn get_weak_creature_by_id(
    app_state: &AppState,
    gs: &GameSystem,
    id: i64,
    optional_data: &CreatureResponseDataModifiers,
) -> Option<Creature> {
    get_creature_by_id(app_state, gs, id, CreatureVariant::Weak, optional_data).await
}
pub async fn get_elite_creature_by_id(
    app_state: &AppState,
    gs: &GameSystem,
    id: i64,
    optional_data: &CreatureResponseDataModifiers,
) -> Option<Creature> {
    get_creature_by_id(app_state, gs, id, CreatureVariant::Elite, optional_data).await
}

pub async fn get_paginated_creatures(
    app_state: &AppState,
    gs: &GameSystem,
    filters: &CreatureFieldFilters,
    pagination: &BestiaryPaginatedRequest,
) -> Result<(u32, Vec<Creature>)> {
    let list = get_list(app_state, gs, CreatureVariant::Base).await;

    let mut filtered_list: Vec<Creature> = list
        .into_iter()
        .filter(|x| Creature::is_passing_filters(x, filters))
        .collect();

    let total_creature_count = filtered_list.len();

    filtered_list.sort_by(|a, b| {
        let cmp = match pagination
            .bestiary_sort_data
            .sort_by
            .clone()
            .unwrap_or_default()
        {
            CreatureSortEnum::Id => a.core_data.essential.id.cmp(&b.core_data.essential.id),
            CreatureSortEnum::Name => a.core_data.essential.name.cmp(&b.core_data.essential.name),
            CreatureSortEnum::Level => a
                .core_data
                .essential
                .base_level
                .cmp(&b.core_data.essential.base_level),
            CreatureSortEnum::Trait => a
                .core_data
                .traits
                .join(", ")
                .cmp(&b.core_data.traits.join(", ")),
            CreatureSortEnum::Size => a.core_data.essential.size.cmp(&b.core_data.essential.size),
            CreatureSortEnum::Type => a
                .core_data
                .essential
                .cr_type
                .cmp(&b.core_data.essential.cr_type),
            CreatureSortEnum::Hp => a.core_data.essential.hp.cmp(&b.core_data.essential.hp),
            CreatureSortEnum::Rarity => a
                .core_data
                .essential
                .rarity
                .cmp(&b.core_data.essential.rarity),
            CreatureSortEnum::Family => a
                .core_data
                .essential
                .family
                .cmp(&b.core_data.essential.family),
            CreatureSortEnum::Alignment => a
                .core_data
                .essential
                .alignment
                .cmp(&b.core_data.essential.alignment),
            CreatureSortEnum::Attack => a
                .core_data
                .derived
                .attack_data
                .cmp(&b.core_data.derived.attack_data),
            CreatureSortEnum::Role => {
                let threshold = filters.role_threshold.unwrap_or(0);
                a.core_data
                    .derived
                    .role_data
                    .iter()
                    .filter(|(_, role_value)| **role_value > threshold)
                    .map(|(role, _)| role)
                    .collect::<Vec<_>>()
                    .cmp(
                        &b.core_data
                            .derived
                            .role_data
                            .iter()
                            .filter(|(_, role_affinity)| **role_affinity > threshold)
                            .map(|(x, _)| x)
                            .collect::<Vec<_>>(),
                    )
            }
        };
        match pagination
            .bestiary_sort_data
            .order_by
            .clone()
            .unwrap_or_default()
        {
            OrderEnum::Ascending => cmp,
            OrderEnum::Descending => cmp.reverse(),
        }
    });
    let curr_slice: Vec<Creature> = filtered_list
        .iter()
        .skip(pagination.paginated_request.cursor as usize)
        .take(if pagination.paginated_request.page_size >= 0 {
            pagination.paginated_request.page_size.unsigned_abs() as usize
        } else {
            usize::MAX
        })
        .cloned()
        .collect();

    Ok((total_creature_count as u32, curr_slice))
}

pub async fn get_creatures_passing_all_filters(
    app_state: &AppState,
    gs: &GameSystem,
    filters: &BestiaryFilterQuery,
    fetch_weak: bool,
    fetch_elite: bool,
) -> Result<Vec<Creature>> {
    let mut creature_vec = Vec::new();
    let level_vec = filters.creature_table_fields_filter.level_filter.clone();
    let mut modified_filters = filters.clone();
    modified_filters.creature_table_fields_filter.level_filter =
        prepare_level_filter_for_db_communication(
            filters
                .creature_table_fields_filter
                .level_filter
                .clone()
                .into_iter(),
            fetch_weak,
            fetch_elite,
        );

    for core in creature_fetcher::fetch_creatures_core_data_with_filters(
        &app_state.conn,
        gs,
        &modified_filters,
    )
    .await?
    {
        // We have fetched creature with level +1 if weak is allowed or level-1 if elite is allowed
        // (or both). Now we catalogue correctly giving them the elite or weak variant, this does not
        // mean that if we have [0,1,2,3] in the filter and allow_elite => [-1,0,1,2,3] then
        // a creature of level 1 will always be considered the elite variant of level 0. We'll
        // duplicate the data and will have a base 0 for level 0 and elite 0 for level 1
        if fetch_weak && level_vec.contains(&(core.essential.base_level - 1)) {
            creature_vec.push(Creature::from_core_with_variant(
                core.clone(),
                CreatureVariant::Weak,
                *gs,
            ));
        }
        if fetch_elite && level_vec.contains(&(core.essential.base_level + 1)) {
            creature_vec.push(Creature::from_core_with_variant(
                core.clone(),
                CreatureVariant::Elite,
                *gs,
            ));
        }
        creature_vec.push(Creature::from_core(core, *gs));
    }
    Ok(creature_vec)
}

pub async fn get_all_possible_values_of_filter(
    app_state: &AppState,
    gs: &GameSystem,
    field: CreatureFilter,
) -> Vec<String> {
    let runtime_fields_values = get_all_keys(app_state, gs).await;
    let mut x = match field {
        CreatureFilter::Size => runtime_fields_values.list_of_sizes,
        CreatureFilter::Rarity => runtime_fields_values.list_of_rarities,
        CreatureFilter::Ranged | CreatureFilter::Melee | CreatureFilter::Spellcaster => {
            vec![true.to_string(), false.to_string()]
        }
        CreatureFilter::Family => runtime_fields_values.list_of_families,
        CreatureFilter::Traits => runtime_fields_values.list_of_traits,
        CreatureFilter::Sources => runtime_fields_values.list_of_sources,
        CreatureFilter::Alignment => AlignmentEnum::iter().map(|x| x.to_string()).collect(),
        CreatureFilter::Level => runtime_fields_values.list_of_levels,
        CreatureFilter::CreatureTypes => CreatureTypeEnum::iter().map(|x| x.to_string()).collect(),
        CreatureFilter::CreatureRoles => CreatureRoleEnum::iter().map(|x| x.to_string()).collect(),
        CreatureFilter::PathfinderVersion => GameSystemVersionEnum::iter()
            .map(|x| x.to_string())
            .collect(),
    };
    x.sort();
    x
}

/// Gets all the runtime keys (each table column unique values). It will cache the result
#[cached(key = "i64", convert = r##"{ gs.into() }"##)]
async fn get_all_keys(app_state: &AppState, gs: &GameSystem) -> FieldsUniqueValuesStruct {
    FieldsUniqueValuesStruct {
        list_of_levels: generic_fetcher::fetch_unique_values_of_field(
            &app_state.conn,
            format!("{gs}_creature_core").as_str(),
            "level",
        )
        .await
        .unwrap_or_default(),
        list_of_families: generic_fetcher::fetch_unique_values_of_field(
            &app_state.conn,
            format!("{gs}_creature_core").as_str(),
            "family",
        )
        .await
        .unwrap(),
        list_of_traits: fetch_traits_associated_with_creatures(&app_state.conn, gs)
            .await
            .unwrap_or_default(),
        list_of_sources: generic_fetcher::fetch_unique_values_of_field(
            &app_state.conn,
            format!("{gs}_creature_core").as_str(),
            "source",
        )
        .await
        .unwrap_or_default(),
        list_of_sizes: generic_fetcher::fetch_unique_values_of_field(
            &app_state.conn,
            format!("{gs}_creature_core").as_str(),
            "size",
        )
        .await
        .unwrap_or_default(),
        list_of_rarities: generic_fetcher::fetch_unique_values_of_field(
            &app_state.conn,
            format!("{gs}_creature_core").as_str(),
            "rarity",
        )
        .await
        .unwrap_or_default(),
    }
}

/// Gets all the creature core data from the DB. It will not fetch data outside of variant and core.
async fn get_all_creatures_from_db(
    app_state: &AppState,
    gs: &GameSystem,
) -> Result<Vec<CreatureCoreData>> {
    creature_fetcher::fetch_creatures_core_data(&app_state.conn, gs, 0, -1).await
}

/// Infallible method, it will expose a vector representing the values fetched from db or empty vec
/// It will cache result
#[cached(key = "i64", convert = r##"{ gs.into() }"##)]
async fn get_list(
    app_state: &AppState,
    gs: &GameSystem,
    variant: CreatureVariant,
) -> Vec<Creature> {
    if let Ok(creatures) = get_all_creatures_from_db(app_state, gs).await {
        return match variant {
            CreatureVariant::Base => creatures
                .into_iter()
                .map(|x| Creature::from_core(x, *gs))
                .collect(),
            _ => creatures
                .into_iter()
                .map(|cr| Creature::from_core_with_variant(cr, variant, *gs))
                .collect(),
        };
    }
    vec![]
}

pub fn order_list_by_level<T: HasLevel + Clone>(elements: &[T]) -> HashMap<i64, Vec<T>> {
    let mut ordered_by_level: HashMap<i64, Vec<T>> = HashMap::new();
    for el in elements {
        ordered_by_level
            .entry(el.level())
            .or_default()
            .push(el.clone());
    }
    ordered_by_level
}

/// Used to prepare the filters for db communication.
/// The level must be adjusted if elite/weak must be fetched.
///Example if we allow weak then we can fetch creature with level +1 => weak = level
fn prepare_level_filter_for_db_communication<I>(
    level_filter: I,
    fetch_weak: bool,
    fetch_elite: bool,
) -> Vec<i64>
where
    I: Iterator<Item = i64>,
{
    // do not remove sorted, it would break contract with merge and dedup
    let levels = level_filter.sorted().collect::<Vec<_>>();
    let levels_for_elite: Vec<i64> = if fetch_elite {
        levels.iter().map(|x| x - 1).collect()
    } else {
        vec![]
    };
    let levels_for_weak: Vec<i64> = if fetch_weak {
        levels.iter().map(|x| x + 1).collect()
    } else {
        vec![]
    };

    let x = itertools::merge(levels_for_elite, levels_for_weak).collect::<Vec<_>>();
    itertools::merge(x, levels).dedup().collect::<Vec<_>>()
}
