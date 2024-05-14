use crate::models::creature::Creature;
use std::collections::{HashMap, HashSet};

use crate::db::cache::from_db_data_to_filter_cache;
use crate::db::data_providers::fetcher;
use crate::db::data_providers::fetcher::get_creatures_core_data_with_filters;
use crate::models::creature_fields_enum::CreatureField;
use crate::models::creature_filter_enum::CreatureFilter;
use crate::models::creature_metadata::variant_enum::CreatureVariant;
use crate::models::response_data::OptionalData;
use crate::models::routers_validator_structs::{FieldFilters, PaginatedRequest};
use crate::services::url_calculator::add_boolean_query;
use crate::AppState;
use anyhow::Result;

fn hp_increase_by_level() -> HashMap<i64, i64> {
    hashmap! { 1 => 10, 2=> 15, 5=> 20, 20=> 30 }
}

pub async fn get_creature_by_id(
    app_state: &AppState,
    id: i64,
    variant: &CreatureVariant,
    optional_data: &OptionalData,
) -> Option<Creature> {
    let cr = fetcher::get_creature_by_id(&app_state.conn, optional_data, id).await;
    if cr.is_err() {
        return None;
    }
    let creature = cr.unwrap();
    Some(convert_creature_to_variant(
        &creature,
        variant.to_level_delta(),
    ))
}

fn convert_creature_to_variant(creature: &Creature, level_delta: i64) -> Creature {
    let mut cr = creature.clone();
    let hp_increase = hp_increase_by_level();
    let desired_key = hp_increase
        .keys()
        .filter(|&&lvl| cr.variant_data.level >= lvl)
        .max()
        .unwrap_or(hp_increase.keys().next().unwrap_or(&0));
    cr.core_data.essential.hp += *hp_increase.get(desired_key).unwrap_or(&0) * level_delta;
    cr.core_data.essential.hp = cr.core_data.essential.hp.max(1);

    cr.variant_data.level += level_delta;

    if level_delta >= 1 {
        cr.variant_data.variant = CreatureVariant::Elite
    } else if level_delta <= -1 {
        cr.variant_data.variant = CreatureVariant::Weak
    } else {
        cr.variant_data.variant = CreatureVariant::Base
    }
    if cr.variant_data.variant != CreatureVariant::Base {
        cr.variant_data.archive_link = add_boolean_query(
            creature.core_data.derived.archive_link.clone(),
            &cr.variant_data.variant.to_string(),
            true,
        );
    }
    cr
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
    let list = get_list(app_state, CreatureVariant::Base, pagination).await;

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

pub async fn fetch_creatures_passing_all_filters(
    app_state: &AppState,
    key_value_filters: &HashMap<CreatureFilter, HashSet<String>>,
) -> Result<Vec<Creature>> {
    Ok(
        get_creatures_core_data_with_filters(&app_state.conn, key_value_filters)
            .await?
            .into_iter()
            .map(Creature::from_core)
            .collect(),
    )
}

pub async fn get_keys(app_state: &AppState, field: CreatureField) -> Vec<String> {
    let runtime_fields_values = from_db_data_to_filter_cache(app_state).await;
    let mut x = match field {
        CreatureField::Size => runtime_fields_values.list_of_sizes,
        CreatureField::Rarity => runtime_fields_values.list_of_rarities,
        CreatureField::Ranged => vec![true.to_string(), false.to_string()],
        CreatureField::Melee => vec![true.to_string(), false.to_string()],
        CreatureField::SpellCaster => vec![true.to_string(), false.to_string()],
        CreatureField::Family => runtime_fields_values.list_of_families,
        CreatureField::Traits => runtime_fields_values.list_of_traits,
        CreatureField::Sources => runtime_fields_values.list_of_sources,
        CreatureField::Alignment => runtime_fields_values.list_of_alignments,
        CreatureField::Level => runtime_fields_values.list_of_levels,
        CreatureField::CreatureTypes => runtime_fields_values.list_of_creature_types,
        _ => vec![],
    };
    x.sort();
    x
}

async fn fetch_creatures_from_db(
    app_state: &AppState,
    variant: CreatureVariant,
    pagination: &PaginatedRequest,
) -> Option<Vec<Creature>> {
    let raw_cr = fetcher::get_creatures_core_data(&app_state.conn, pagination).await;
    if raw_cr.is_err() {
        None
    } else {
        let creatures = raw_cr
            .unwrap()
            .into_iter()
            .map(Creature::from_core)
            .collect();
        match variant {
            CreatureVariant::Base => Some(creatures),
            _ => Some(
                creatures
                    .iter()
                    .map(|cr| {
                        convert_creature_to_variant(cr, CreatureVariant::to_level_delta(&variant))
                    })
                    .collect(),
            ),
        }
    }
}

///
/// Infallible method, it will expose a vector representing the values fetched from db
async fn get_list(
    app_state: &AppState,
    variant: CreatureVariant,
    pagination: &PaginatedRequest,
) -> Vec<Creature> {
    if let Some(db_data) = fetch_creatures_from_db(app_state, variant, pagination).await {
        return db_data;
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
