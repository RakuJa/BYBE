use crate::db::db_communicator;
use crate::models::creature::{check_creature_pass_filters, Creature};
use std::collections::{HashMap, HashSet};

use crate::db::db_cache::from_db_data_to_filter_cache;
use crate::models::creature_fields_enum::CreatureField;
use crate::models::creature_filter_enum::CreatureFilter;
use crate::models::routers_validator_structs::{FieldFilters, PaginatedRequest};
use crate::AppState;
use anyhow::Result;

pub async fn get_creature_by_id(app_state: &AppState, id: i32) -> Option<Creature> {
    let list = get_list(app_state).await;
    list.iter().find(|creature| creature.id == id).cloned()
}

pub async fn get_paginated_creatures(
    app_state: &AppState,
    filters: &FieldFilters,
    pagination: &PaginatedRequest,
) -> Result<(u32, Vec<Creature>)> {
    let list = get_list(app_state).await;

    let filtered_list: Vec<Creature> = list
        .into_iter()
        .filter(|x| check_creature_pass_filters(x, filters))
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
    key_value_filters: HashMap<CreatureFilter, HashSet<String>>,
) -> Result<HashSet<Creature>> {
    let creature_list = get_list(app_state).await;
    let mut intersection = HashSet::from_iter(creature_list.iter().cloned());
    key_value_filters
        .iter()
        .map(|(curr_field_filter, curr_value_filter)| {
            fetch_creatures_passing_single_filter(
                &creature_list,
                curr_field_filter,
                curr_value_filter,
            )
        })
        .for_each(|curr| intersection = intersection.intersection(&curr).cloned().collect());
    Ok(intersection)
}

fn fetch_creatures_passing_single_filter(
    creature_list: &[Creature],
    field_in_which_to_filter: &CreatureFilter,
    filter_vec: &HashSet<String>,
) -> HashSet<Creature> {
    let cr_iterator = creature_list.iter().cloned();
    match field_in_which_to_filter {
        CreatureFilter::Id => cr_iterator
            .filter(|creature| filter_vec.contains(creature.id.to_string().as_str()))
            .collect(),
        CreatureFilter::Level => cr_iterator
            .filter(|creature| filter_vec.contains(creature.level.to_string().as_str()))
            .collect(),
        CreatureFilter::Family => cr_iterator
            .filter(|creature| {
                filter_vec.contains(creature.family.clone().unwrap_or_default().as_str())
            })
            .collect(),
        CreatureFilter::Traits => cr_iterator
            .filter(|creature| {
                filter_vec
                    .iter()
                    .any(|curr_trait| creature.clone().traits.contains(curr_trait))
            })
            .collect(),
        CreatureFilter::CreatureTypes => cr_iterator
            .filter(|creature| filter_vec.contains(creature.creature_type.to_string().as_str()))
            .collect(),
        CreatureFilter::Alignment => cr_iterator
            .filter(|creature| filter_vec.contains(creature.alignment.to_string().as_str()))
            .collect(),
        CreatureFilter::Size => cr_iterator
            .filter(|creature| filter_vec.contains(creature.size.to_string().as_str()))
            .collect(),
        CreatureFilter::Rarity => cr_iterator
            .filter(|creature| filter_vec.contains(creature.rarity.to_string().as_str()))
            .collect(),
        CreatureFilter::Melee => cr_iterator
            .filter(|creature| filter_vec.contains(creature.is_melee.to_string().as_str()))
            .collect(),
        CreatureFilter::Ranged => cr_iterator
            .filter(|creature| filter_vec.contains(creature.is_ranged.to_string().as_str()))
            .collect(),
        CreatureFilter::SpellCaster => cr_iterator
            .filter(|creature| filter_vec.contains(creature.is_spell_caster.to_string().as_str()))
            .collect(),
    }
}

pub async fn get_keys(app_state: &AppState, field: CreatureField) -> Vec<String> {
    if let Some(db_data) = fetch_data_from_database(app_state).await {
        let runtime_fields_values = from_db_data_to_filter_cache(app_state, db_data);
        let mut x = match field {
            CreatureField::Id => runtime_fields_values.list_of_ids,
            CreatureField::Size => runtime_fields_values.list_of_sizes,
            CreatureField::Rarity => runtime_fields_values.list_of_rarities,
            CreatureField::Ranged => vec![true.to_string(), false.to_string()],
            CreatureField::Melee => vec![true.to_string(), false.to_string()],
            CreatureField::SpellCaster => vec![true.to_string(), false.to_string()],
            CreatureField::Family => runtime_fields_values.list_of_families,
            CreatureField::Traits => runtime_fields_values.list_of_traits,
            CreatureField::Alignment => runtime_fields_values.list_of_alignments,
            CreatureField::Level => runtime_fields_values.list_of_levels,
            CreatureField::CreatureTypes => runtime_fields_values.list_of_creature_types,

            _ => vec![],
        };
        x.sort();
        return x;
    }
    vec![]
}

async fn fetch_data_from_database(app_state: &AppState) -> Option<Vec<Creature>> {
    if let Some(creature_vec) = fetch_creatures(app_state).await {
        return Some(creature_vec);
    }
    None
}

async fn fetch_creatures(app_state: &AppState) -> Option<Vec<Creature>> {
    let cache = &app_state.creature_cache.clone();
    if let Some(creatures) = cache.get(&0) {
        return Some(creatures);
    } else if let Ok(creatures) = db_communicator::fetch_creatures(&app_state.conn).await {
        cache.insert(0, creatures.clone());
        return Some(creatures);
    }
    None
}

async fn get_list(app_state: &AppState) -> Vec<Creature> {
    if let Some(db_data) = fetch_data_from_database(app_state).await {
        return db_data;
    }
    vec![]
}

pub fn order_list_by_level(creature_list: HashSet<Creature>) -> HashMap<i16, Vec<Creature>> {
    let mut ordered_by_level = HashMap::new();
    creature_list.iter().for_each(|creature| {
        ordered_by_level
            .entry(creature.level as i16)
            .or_insert_with(Vec::new)
            .push(creature.clone());
    });
    ordered_by_level
}
