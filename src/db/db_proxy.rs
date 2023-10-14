use crate::db::db_communicator;
use crate::models::creature::{check_creature_pass_filters, Creature};
use redis::RedisError;
use std::collections::{HashMap, HashSet};
use std::thread::sleep;
use std::time::Duration;

use crate::db::db_cache::{from_db_data_to_filter_cache, from_db_data_to_sorted_vectors, DbCache};
use crate::models::creature_fields_enum::CreatureField;
use crate::models::creature_filter_enum::CreatureFilter;
use crate::models::creature_sort_enums::{OrderEnum, SortEnum};
use crate::models::routers_validator_structs::{FieldFilters, PaginatedRequest, SortData};
use anyhow::{ensure, Result};
use lazy_static::lazy_static;

lazy_static! {
    static ref CACHE: std::sync::Mutex<DbCache> = std::sync::Mutex::new(DbCache::default());
}

pub async fn update_cache() {
    loop {
        log::info!("Starting cache update...");
        match CACHE.lock() {
            Ok(mut cache) => {
                let db_data = fetch_data_from_database();
                *cache = DbCache {
                    lists: from_db_data_to_sorted_vectors(&db_data),
                    filters: from_db_data_to_filter_cache(&db_data),
                };
                log::info!("Cache updated");
            }
            Err(_) => log::error!("Error occurred while updating db"),
        }
        sleep(Duration::from_secs(3600));
    }
}

pub fn get_paginated_creatures(
    sort: &SortData,
    filters: &FieldFilters,
    pagination: &PaginatedRequest,
) -> Result<(u32, Vec<Creature>)> {
    let list = get_list(sort.sort_key, sort.order_by)?;

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

pub fn fetch_creatures_passing_all_filters(
    key_value_filters: HashMap<CreatureFilter, HashSet<String>>,
) -> Result<HashSet<Creature>> {
    let creature_list = get_list(None, None)?;
    let mut x: HashSet<Creature> = HashSet::new();
    key_value_filters
        .iter()
        .map(|(curr_field_filter, curr_value_filter)| {
            fetch_creatures_passing_single_filter(
                &creature_list,
                curr_field_filter,
                curr_value_filter,
            )
        })
        .for_each(|curr| x.extend(curr));
    Ok(x)
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
                filter_vec.contains(creature.clone().family.unwrap_or_default().as_str())
            })
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

pub fn get_keys(field: CreatureField) -> Vec<String> {
    if let Ok(cache) = CACHE.lock() {
        let mut x = match field {
            CreatureField::Id => cache
                .filters
                .filtered_by_id
                .keys()
                .map(|value| value.to_string())
                .collect(),
            CreatureField::Size => cache
                .filters
                .filtered_by_size
                .keys()
                .map(|value| value.to_string())
                .collect(),
            CreatureField::Rarity => cache
                .filters
                .filtered_by_rarity
                .keys()
                .map(|value| value.to_string())
                .collect(),
            CreatureField::Ranged => cache
                .filters
                .filtered_by_ranged
                .keys()
                .map(|value| value.to_string())
                .collect(),
            CreatureField::Melee => cache
                .filters
                .filtered_by_melee
                .keys()
                .map(|value| value.to_string())
                .collect(),
            CreatureField::SpellCaster => cache
                .filters
                .filtered_by_spell_caster
                .keys()
                .map(|value| value.to_string())
                .collect(),
            CreatureField::Family => cache
                .filters
                .filtered_by_family
                .keys()
                .map(|value| value.clone().unwrap_or("-".to_string()))
                .collect(),
            CreatureField::Alignment => cache
                .filters
                .filtered_by_alignment
                .keys()
                .map(|value| value.to_string())
                .collect(),
            CreatureField::Level => cache
                .filters
                .filtered_by_level
                .keys()
                .map(|value| value.to_string())
                .collect(),
            _ => vec![],
        };
        x.sort();
        return x;
    }
    vec![]
}

fn fetch_data_from_database() -> Result<Vec<Creature>, RedisError> {
    db_communicator::get_creatures_by_ids(db_communicator::fetch_and_parse_all_keys(
        &"creature:".to_string(),
    )?)
}

fn get_list(sort_field: Option<SortEnum>, order_by: Option<OrderEnum>) -> Result<Vec<Creature>> {
    let x = CACHE.lock();
    ensure!(x.is_ok(), "Could not get lock to fetch creatures");
    let cache = x.unwrap();
    match (sort_field.unwrap_or_default(), order_by.unwrap_or_default()) {
        (SortEnum::Id, OrderEnum::Ascending) => Ok(cache.lists.order_by_id_ascending.to_owned()),
        (SortEnum::Id, OrderEnum::Descending) => Ok(cache.lists.order_by_id_descending.to_owned()),

        (SortEnum::Hp, OrderEnum::Ascending) => Ok(cache.lists.order_by_hp_ascending.to_owned()),
        (SortEnum::Hp, OrderEnum::Descending) => Ok(cache.lists.order_by_hp_descending.to_owned()),

        (SortEnum::Family, OrderEnum::Ascending) => {
            Ok(cache.lists.order_by_family_ascending.to_owned())
        }
        (SortEnum::Family, OrderEnum::Descending) => {
            Ok(cache.lists.order_by_family_descending.to_owned())
        }

        (SortEnum::Alignment, OrderEnum::Ascending) => {
            Ok(cache.lists.order_by_alignment_ascending.to_owned())
        }
        (SortEnum::Alignment, OrderEnum::Descending) => {
            Ok(cache.lists.order_by_alignment_descending.to_owned())
        }

        (SortEnum::Level, OrderEnum::Ascending) => {
            Ok(cache.lists.order_by_level_ascending.to_owned())
        }
        (SortEnum::Level, OrderEnum::Descending) => {
            Ok(cache.lists.order_by_level_descending.to_owned())
        }

        (SortEnum::Name, OrderEnum::Ascending) => {
            Ok(cache.lists.order_by_name_ascending.to_owned())
        }
        (SortEnum::Name, OrderEnum::Descending) => {
            Ok(cache.lists.order_by_name_descending.to_owned())
        }

        (SortEnum::Rarity, OrderEnum::Ascending) => {
            Ok(cache.lists.order_by_rarity_ascending.to_owned())
        }
        (SortEnum::Rarity, OrderEnum::Descending) => {
            Ok(cache.lists.order_by_rarity_descending.to_owned())
        }

        (SortEnum::Size, OrderEnum::Ascending) => {
            Ok(cache.lists.order_by_size_ascending.to_owned())
        }
        (SortEnum::Size, OrderEnum::Descending) => {
            Ok(cache.lists.order_by_rarity_descending.to_owned())
        }
    }
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
