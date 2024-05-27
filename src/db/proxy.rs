use crate::models::creature::Creature;
use std::collections::{HashMap, HashSet};

use crate::db::data_providers::fetcher;
use crate::db::data_providers::fetcher::{
    fetch_traits_associated_with_creatures, fetch_unique_values_of_field,
};
use crate::models::creature_component::creature_core::CreatureCoreData;
use crate::models::creature_component::fields_unique_values_struct::FieldsUniqueValuesStruct;
use crate::models::creature_fields_enum::CreatureField;
use crate::models::creature_filter_enum::CreatureFilter;
use crate::models::creature_metadata::alignment_enum::AlignmentEnum;
use crate::models::creature_metadata::type_enum::CreatureTypeEnum;
use crate::models::creature_metadata::variant_enum::CreatureVariant;
use crate::models::response_data::OptionalData;
use crate::models::routers_validator_structs::{FieldFilters, PaginatedRequest};
use crate::AppState;
use anyhow::Result;
use cached::proc_macro::once;
use strum::IntoEnumIterator;

pub async fn get_creature_by_id(
    app_state: &AppState,
    id: i64,
    variant: &CreatureVariant,
    optional_data: &OptionalData,
) -> Option<Creature> {
    let creature = fetcher::fetch_creature_by_id(&app_state.conn, optional_data, id)
        .await
        .ok()?;
    Some(creature.convert_creature_to_variant(variant))
}

pub async fn get_weak_creature_by_id(
    app_state: &AppState,
    id: i64,
    optional_data: &OptionalData,
) -> Option<Creature> {
    get_creature_by_id(app_state, id, &CreatureVariant::Weak, optional_data).await
}
pub async fn get_elite_creature_by_id(
    app_state: &AppState,
    id: i64,
    optional_data: &OptionalData,
) -> Option<Creature> {
    get_creature_by_id(app_state, id, &CreatureVariant::Elite, optional_data).await
}

pub async fn get_paginated_creatures(
    app_state: &AppState,
    filters: &FieldFilters,
    pagination: &PaginatedRequest,
) -> Result<(u32, Vec<Creature>)> {
    let list = get_list(app_state, CreatureVariant::Base).await;

    let filtered_list: Vec<Creature> = list
        .into_iter()
        .filter(|x| Creature::is_passing_filters(x, filters))
        .collect();

    let curr_slice: Vec<Creature> = filtered_list
        .iter()
        .skip(pagination.cursor as usize)
        .take(pagination.page_size as usize)
        .cloned()
        .collect();

    Ok((curr_slice.len() as u32, curr_slice))
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
        fetcher::fetch_creatures_core_data_with_filters(&app_state.conn, &modified_filters).await?
    {
        // We have fetched creature with level +1 if weak is allowed or level-1 if elite is allowed
        // (or both). Now we catalogue correctly giving them the elite or weak variant, this does not
        // mean that if we have [0,1,2,3] in the filter and allow_elite => [-1,0,1,2,3] then
        // a creature of level 1 will always be considered the elite variant of level 0. We'll
        // duplicate the data and will have a base 0 for level 0 and elite 0 for level 1
        if fetch_weak && level_vec.contains(&(core.essential.level - 1).to_string()) {
            creature_vec.push(Creature::from_core_with_variant(
                core.clone(),
                &CreatureVariant::Weak,
            ));
        }
        if fetch_elite && level_vec.contains(&(core.essential.level + 1).to_string()) {
            creature_vec.push(Creature::from_core_with_variant(
                core.clone(),
                &CreatureVariant::Elite,
            ));
        }
        creature_vec.push(Creature::from_core(core));
    }
    Ok(creature_vec)
}

pub async fn get_keys(app_state: &AppState, field: CreatureField) -> Vec<String> {
    let runtime_fields_values = get_all_keys(app_state).await;
    let mut x = match field {
        CreatureField::Size => runtime_fields_values.list_of_sizes,
        CreatureField::Rarity => runtime_fields_values.list_of_rarities,
        CreatureField::Ranged => vec![true.to_string(), false.to_string()],
        CreatureField::Melee => vec![true.to_string(), false.to_string()],
        CreatureField::SpellCaster => vec![true.to_string(), false.to_string()],
        CreatureField::Family => runtime_fields_values.list_of_families,
        CreatureField::Traits => runtime_fields_values.list_of_traits,
        CreatureField::Sources => runtime_fields_values.list_of_sources,
        CreatureField::Alignment => AlignmentEnum::iter().map(|x| x.to_string()).collect(),
        CreatureField::Level => runtime_fields_values.list_of_levels,
        CreatureField::CreatureTypes => CreatureTypeEnum::iter().map(|x| x.to_string()).collect(),
        _ => vec![],
    };
    x.sort();
    x
}

/// Gets all the runtime keys (each table column unique values). It will cache the result
#[once(sync_writes = true)]
async fn get_all_keys(app_state: &AppState) -> FieldsUniqueValuesStruct {
    FieldsUniqueValuesStruct {
        list_of_levels: fetch_unique_values_of_field(&app_state.conn, "CREATURE_CORE", "level")
            .await
            .unwrap_or_default(),
        list_of_families: fetch_unique_values_of_field(&app_state.conn, "CREATURE_CORE", "family")
            .await
            .unwrap(),
        list_of_traits: fetch_traits_associated_with_creatures(&app_state.conn)
            .await
            .unwrap_or_default(),
        list_of_sources: fetch_unique_values_of_field(&app_state.conn, "CREATURE_CORE", "source")
            .await
            .unwrap_or_default(),
        list_of_sizes: fetch_unique_values_of_field(&app_state.conn, "CREATURE_CORE", "size")
            .await
            .unwrap_or_default(),
        list_of_rarities: fetch_unique_values_of_field(&app_state.conn, "CREATURE_CORE", "rarity")
            .await
            .unwrap_or_default(),
    }
}

/// Gets all the creature core data from the DB. It will not fetch data outside of variant and core.
/// It will cache the result.
#[once(sync_writes = true, result = true)]
async fn get_all_creatures_from_db(app_state: &AppState) -> Result<Vec<CreatureCoreData>> {
    fetcher::fetch_creatures_core_data(
        &app_state.conn,
        &PaginatedRequest {
            cursor: 0,
            page_size: -1,
        },
    )
    .await
}

/// Infallible method, it will expose a vector representing the values fetched from db or empty vec
async fn get_list(app_state: &AppState, variant: CreatureVariant) -> Vec<Creature> {
    if let Ok(creatures) = get_all_creatures_from_db(app_state).await {
        return match variant {
            CreatureVariant::Base => creatures.into_iter().map(Creature::from_core).collect(),
            _ => creatures
                .into_iter()
                .map(|cr| Creature::from_core_with_variant(cr, &variant))
                .collect(),
        };
    }
    vec![]
}

pub fn order_list_by_level(creature_list: Vec<Creature>) -> HashMap<i64, Vec<Creature>> {
    let mut ordered_by_level = HashMap::new();
    creature_list.iter().for_each(|creature| {
        ordered_by_level
            .entry(creature.variant_data.level)
            .or_insert_with(Vec::new)
            .push(creature.clone());
    });
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
