use crate::db::db_communicator;
use crate::models::creature::{check_creature_pass_filters, Creature};
use redis::RedisError;
use std::thread::sleep;
use std::time::Duration;

use crate::db::db_cache::{from_db_data_to_filter_cache, from_db_data_to_sorted_vectors, DbCache};
use crate::models::creature_fields_enum::CreatureField;
use crate::models::creature_sort_enums::{OrderEnum, SortEnum};
use crate::models::routers_validator_structs::{FieldFilters, PaginatedRequest, SortData};
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
        sleep(Duration::from_secs(60));
    }
}

pub fn get_paginated_creatures(
    sort: &SortData,
    filters: &FieldFilters,
    pagination: &PaginatedRequest,
) -> Option<(u32, Vec<Creature>)> {
    match get_list(sort.sort_key, sort.order_by) {
        Some(list) => {
            let filtered_list: Vec<Creature> = list
                .into_iter()
                .filter(|x| check_creature_pass_filters(x, filters))
                .collect();

            let next_cursor = pagination.cursor + pagination.page_size as u32;
            let curr_slice: Vec<Creature> = filtered_list
                .iter()
                .skip(pagination.cursor as usize)
                .take(next_cursor as usize)
                .cloned()
                .collect();

            Some((curr_slice.len() as u32, curr_slice))
        }
        None => None,
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

fn get_list(sort_field: Option<SortEnum>, order_by: Option<OrderEnum>) -> Option<Vec<Creature>> {
    if let Ok(cache) = CACHE.lock() {
        match (sort_field.unwrap_or_default(), order_by.unwrap_or_default()) {
            (SortEnum::Id, OrderEnum::Ascending) => {
                Some(cache.lists.order_by_id_ascending.to_owned())
            }
            (SortEnum::Id, OrderEnum::Descending) => {
                Some(cache.lists.order_by_id_descending.to_owned())
            }

            (SortEnum::Hp, OrderEnum::Ascending) => {
                Some(cache.lists.order_by_hp_ascending.to_owned())
            }
            (SortEnum::Hp, OrderEnum::Descending) => {
                Some(cache.lists.order_by_hp_descending.to_owned())
            }

            (SortEnum::Family, OrderEnum::Ascending) => {
                Some(cache.lists.order_by_family_ascending.to_owned())
            }
            (SortEnum::Family, OrderEnum::Descending) => {
                Some(cache.lists.order_by_family_descending.to_owned())
            }

            (SortEnum::Alignment, OrderEnum::Ascending) => {
                Some(cache.lists.order_by_alignment_ascending.to_owned())
            }
            (SortEnum::Alignment, OrderEnum::Descending) => {
                Some(cache.lists.order_by_alignment_descending.to_owned())
            }

            (SortEnum::Level, OrderEnum::Ascending) => {
                Some(cache.lists.order_by_level_ascending.to_owned())
            }
            (SortEnum::Level, OrderEnum::Descending) => {
                Some(cache.lists.order_by_level_descending.to_owned())
            }

            (SortEnum::Name, OrderEnum::Ascending) => {
                Some(cache.lists.order_by_name_ascending.to_owned())
            }
            (SortEnum::Name, OrderEnum::Descending) => {
                Some(cache.lists.order_by_name_descending.to_owned())
            }

            (SortEnum::Rarity, OrderEnum::Ascending) => {
                Some(cache.lists.order_by_rarity_ascending.to_owned())
            }
            (SortEnum::Rarity, OrderEnum::Descending) => {
                Some(cache.lists.order_by_rarity_descending.to_owned())
            }

            (SortEnum::Size, OrderEnum::Ascending) => {
                Some(cache.lists.order_by_size_ascending.to_owned())
            }
            (SortEnum::Size, OrderEnum::Descending) => {
                Some(cache.lists.order_by_rarity_descending.to_owned())
            }
        }
    } else {
        None
    }
}
