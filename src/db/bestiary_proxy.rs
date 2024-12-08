use crate::models::creature::creature_struct::Creature;
use std::collections::{HashMap, HashSet};

use crate::db::data_providers::creature_fetcher::fetch_traits_associated_with_creatures;
use crate::db::data_providers::{creature_fetcher, generic_fetcher};
use crate::models::bestiary_structs::{BestiaryPaginatedRequest, CreatureSortEnum};
use crate::models::creature::creature_component::creature_core::CreatureCoreData;
use crate::models::creature::creature_filter_enum::{CreatureFilter, FieldsUniqueValuesStruct};
use crate::models::creature::creature_metadata::alignment_enum::AlignmentEnum;
use crate::models::creature::creature_metadata::creature_role::CreatureRoleEnum;
use crate::models::creature::creature_metadata::type_enum::CreatureTypeEnum;
use crate::models::creature::creature_metadata::variant_enum::CreatureVariant;
use crate::models::pf_version_enum::PathfinderVersionEnum;
use crate::models::response_data::ResponseDataModifiers;
use crate::models::routers_validator_structs::{CreatureFieldFilters, OrderEnum};
use crate::AppState;
use anyhow::Result;
use cached::proc_macro::once;
use strum::IntoEnumIterator;

pub async fn get_creature_by_id(
    app_state: &AppState,
    id: i64,
    variant: CreatureVariant,
    response_data_mods: &ResponseDataModifiers,
) -> Option<Creature> {
    creature_fetcher::fetch_creature_by_id(&app_state.conn, variant, response_data_mods, id)
        .await
        .ok()
}

pub async fn get_weak_creature_by_id(
    app_state: &AppState,
    id: i64,
    optional_data: &ResponseDataModifiers,
) -> Option<Creature> {
    get_creature_by_id(app_state, id, CreatureVariant::Weak, optional_data).await
}
pub async fn get_elite_creature_by_id(
    app_state: &AppState,
    id: i64,
    optional_data: &ResponseDataModifiers,
) -> Option<Creature> {
    get_creature_by_id(app_state, id, CreatureVariant::Elite, optional_data).await
}

pub async fn get_paginated_creatures(
    app_state: &AppState,
    filters: &CreatureFieldFilters,
    pagination: &BestiaryPaginatedRequest,
) -> Result<(u32, Vec<Creature>)> {
    let list = get_list(app_state, CreatureVariant::Base).await;

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
        .take(pagination.paginated_request.page_size.unsigned_abs() as usize)
        .cloned()
        .collect();

    Ok((total_creature_count as u32, curr_slice))
}

pub async fn get_creatures_passing_all_filters(
    app_state: &AppState,
    key_value_filters: HashMap<CreatureFilter, HashSet<String>>,
    fetch_weak: bool,
    fetch_elite: bool,
) -> Result<Vec<Creature>> {
    let mut creature_vec = Vec::new();
    let level_vec = key_value_filters
        .get(&CreatureFilter::Level)
        .unwrap_or(&HashSet::new())
        .clone();
    let modified_filters =
        prepare_filters_for_db_communication(key_value_filters, fetch_weak, fetch_elite);
    for core in
        creature_fetcher::fetch_creatures_core_data_with_filters(&app_state.conn, &modified_filters)
            .await?
    {
        // We have fetched creature with level +1 if weak is allowed or level-1 if elite is allowed
        // (or both). Now we catalogue correctly giving them the elite or weak variant, this does not
        // mean that if we have [0,1,2,3] in the filter and allow_elite => [-1,0,1,2,3] then
        // a creature of level 1 will always be considered the elite variant of level 0. We'll
        // duplicate the data and will have a base 0 for level 0 and elite 0 for level 1
        if fetch_weak && level_vec.contains(&(core.essential.base_level - 1).to_string()) {
            creature_vec.push(Creature::from_core_with_variant(
                core.clone(),
                CreatureVariant::Weak,
            ));
        }
        if fetch_elite && level_vec.contains(&(core.essential.base_level + 1).to_string()) {
            creature_vec.push(Creature::from_core_with_variant(
                core.clone(),
                CreatureVariant::Elite,
            ));
        }
        creature_vec.push(Creature::from_core(core));
    }
    Ok(creature_vec)
}

pub async fn get_all_possible_values_of_filter(
    app_state: &AppState,
    field: CreatureFilter,
) -> Vec<String> {
    let runtime_fields_values = get_all_keys(app_state).await;
    let mut x = match field {
        CreatureFilter::Size => runtime_fields_values.list_of_sizes,
        CreatureFilter::Rarity => runtime_fields_values.list_of_rarities,
        CreatureFilter::Ranged | CreatureFilter::Melee | CreatureFilter::SpellCaster => {
            vec![true.to_string(), false.to_string()]
        }
        CreatureFilter::Family => runtime_fields_values.list_of_families,
        CreatureFilter::Traits => runtime_fields_values.list_of_traits,
        CreatureFilter::Sources => runtime_fields_values.list_of_sources,
        CreatureFilter::Alignment => AlignmentEnum::iter().map(|x| x.to_string()).collect(),
        CreatureFilter::Level => runtime_fields_values.list_of_levels,
        CreatureFilter::CreatureTypes => CreatureTypeEnum::iter().map(|x| x.to_string()).collect(),
        CreatureFilter::CreatureRoles => CreatureRoleEnum::iter().map(|x| x.to_string()).collect(),
        CreatureFilter::PathfinderVersion => PathfinderVersionEnum::iter()
            .map(|x| x.to_string())
            .collect(),
    };
    x.sort();
    x
}

/// Gets all the runtime keys (each table column unique values). It will cache the result
#[once(sync_writes = true)]
async fn get_all_keys(app_state: &AppState) -> FieldsUniqueValuesStruct {
    FieldsUniqueValuesStruct {
        list_of_levels: generic_fetcher::fetch_unique_values_of_field(
            &app_state.conn,
            "CREATURE_CORE",
            "level",
        )
        .await
        .unwrap_or_default(),
        list_of_families: generic_fetcher::fetch_unique_values_of_field(
            &app_state.conn,
            "CREATURE_CORE",
            "family",
        )
        .await
        .unwrap(),
        list_of_traits: fetch_traits_associated_with_creatures(&app_state.conn)
            .await
            .unwrap_or_default(),
        list_of_sources: generic_fetcher::fetch_unique_values_of_field(
            &app_state.conn,
            "CREATURE_CORE",
            "source",
        )
        .await
        .unwrap_or_default(),
        list_of_sizes: generic_fetcher::fetch_unique_values_of_field(
            &app_state.conn,
            "CREATURE_CORE",
            "size",
        )
        .await
        .unwrap_or_default(),
        list_of_rarities: generic_fetcher::fetch_unique_values_of_field(
            &app_state.conn,
            "CREATURE_CORE",
            "rarity",
        )
        .await
        .unwrap_or_default(),
    }
}

/// Gets all the creature core data from the DB. It will not fetch data outside of variant and core.
/// It will cache the result.
#[once(sync_writes = true, result = true)]
async fn get_all_creatures_from_db(app_state: &AppState) -> Result<Vec<CreatureCoreData>> {
    creature_fetcher::fetch_creatures_core_data(&app_state.conn, 0, -1).await
}

/// Infallible method, it will expose a vector representing the values fetched from db or empty vec
async fn get_list(app_state: &AppState, variant: CreatureVariant) -> Vec<Creature> {
    if let Ok(creatures) = get_all_creatures_from_db(app_state).await {
        return match variant {
            CreatureVariant::Base => creatures.into_iter().map(Creature::from_core).collect(),
            _ => creatures
                .into_iter()
                .map(|cr| Creature::from_core_with_variant(cr, variant))
                .collect(),
        };
    }
    vec![]
}

pub fn order_list_by_level(creature_list: &[Creature]) -> HashMap<i64, Vec<Creature>> {
    let mut ordered_by_level = HashMap::new();

    for creature in creature_list {
        ordered_by_level
            .entry(creature.variant_data.level)
            .or_insert_with(Vec::new)
            .push(creature.clone());
    }
    ordered_by_level
}

/// Used to prepare the filters for db communication.
/// The level must be adjusted if elite/weak must be fetched.
///Example if we allow weak then we can fetch creature with level +1 => weak = level
fn prepare_filters_for_db_communication(
    key_value_filters: HashMap<CreatureFilter, HashSet<String>>,
    fetch_weak: bool,
    fetch_elite: bool,
) -> HashMap<CreatureFilter, HashSet<String>> {
    key_value_filters
        .into_iter()
        .map(|(key, values)| match key {
            CreatureFilter::Level => {
                let mut new_values = HashSet::new();
                for str_level in values {
                    if let Ok(level) = str_level.parse::<i64>() {
                        if fetch_weak {
                            new_values.insert((level + 1).to_string());
                        }
                        if fetch_elite {
                            new_values.insert((level - 1).to_string());
                        }
                        new_values.insert(level.to_string());
                    }
                }
                (key, new_values)
            }
            _ => (key, values),
        })
        .collect()
}
