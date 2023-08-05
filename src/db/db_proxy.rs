use crate::db::db_communicator;
use crate::models::creature::Creature;
use redis::RedisError;
use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;

use crate::models::creature_metadata_enums::{AlignmentEnum, RarityEnum, SizeEnum};
use crate::models::creature_sort_enums::{OrderEnum, SortEnum};
use crate::models::routers_validator_structs::{FieldFilters, PaginatedRequest, SortData};
use lazy_static::lazy_static;

lazy_static! {
    static ref CACHE: std::sync::Mutex<DbCache> = std::sync::Mutex::new(DbCache::default());
}

#[derive(Debug)]
pub struct DbCache {
    lists: SortedVectorCache,
    filters: FiltersCache,
}

impl Default for DbCache {
    fn default() -> Self {
        DbCache {
            lists: SortedVectorCache::default(),
            filters: FiltersCache::default(),
        }
    }
}
#[derive(Debug)]
struct SortedVectorCache {
    pub unordered_creatures: Vec<Creature>,

    pub order_by_id_ascending: Vec<Creature>,
    pub order_by_id_descending: Vec<Creature>,

    pub order_by_name_ascending: Vec<Creature>,
    pub order_by_name_descending: Vec<Creature>,

    pub order_by_hp_ascending: Vec<Creature>,
    pub order_by_hp_descending: Vec<Creature>,

    pub order_by_level_ascending: Vec<Creature>,
    pub order_by_level_descending: Vec<Creature>,

    pub order_by_family_ascending: Vec<Creature>,
    pub order_by_family_descending: Vec<Creature>,

    pub order_by_alignment_ascending: Vec<Creature>,
    pub order_by_alignment_descending: Vec<Creature>,

    pub order_by_size_ascending: Vec<Creature>,
    pub order_by_size_descending: Vec<Creature>,

    pub order_by_rarity_ascending: Vec<Creature>,
    pub order_by_rarity_descending: Vec<Creature>,
}

impl Default for SortedVectorCache {
    fn default() -> Self {
        SortedVectorCache {
            unordered_creatures: vec![],
            order_by_id_ascending: vec![],
            order_by_id_descending: vec![],
            order_by_name_ascending: vec![],
            order_by_name_descending: vec![],
            order_by_hp_ascending: vec![],
            order_by_hp_descending: vec![],
            order_by_level_ascending: vec![],
            order_by_level_descending: vec![],
            order_by_family_ascending: vec![],
            order_by_family_descending: vec![],
            order_by_alignment_ascending: vec![],
            order_by_alignment_descending: vec![],
            order_by_size_ascending: vec![],
            order_by_size_descending: vec![],
            order_by_rarity_ascending: vec![],
            order_by_rarity_descending: vec![],
        }
    }
}
#[derive(Debug)]
struct FiltersCache {
    pub filtered_by_id: HashMap<i32, Vec<Creature>>,
    pub filtered_by_level: HashMap<i8, Vec<Creature>>,
    pub filtered_by_family: HashMap<Option<String>, Vec<Creature>>,
    pub filtered_by_alignment: HashMap<AlignmentEnum, Vec<Creature>>,
    pub filtered_by_size: HashMap<SizeEnum, Vec<Creature>>,
    pub filtered_by_rarity: HashMap<RarityEnum, Vec<Creature>>,
    pub filtered_by_melee: HashMap<bool, Vec<Creature>>,
    pub filtered_by_ranged: HashMap<bool, Vec<Creature>>,
    pub filtered_by_spell_caster: HashMap<bool, Vec<Creature>>,
}

impl Default for FiltersCache {
    fn default() -> Self {
        FiltersCache {
            filtered_by_id: Default::default(),
            filtered_by_level: Default::default(),
            filtered_by_family: Default::default(),
            filtered_by_alignment: Default::default(),
            filtered_by_size: Default::default(),
            filtered_by_rarity: Default::default(),
            filtered_by_melee: Default::default(),
            filtered_by_ranged: Default::default(),
            filtered_by_spell_caster: Default::default(),
        }
    }
}

fn from_db_data_to_filter_cache(data: &Result<Vec<Creature>, RedisError>) -> FiltersCache {
    let mut filters_cache = FiltersCache::default();
    // This is inefficient AF, I need to study a way to share
    // those objects between hashmaps, as they are immutable.
    // The problem is that having a reference requires a lifetime, and
    // right now the fields have different lifetime than the struct
    if let Ok(creatures) = data {
        for curr_creature in creatures {
            filters_cache
                .filtered_by_id
                .entry(curr_creature.id)
                .or_insert_with(Vec::new)
                .push(curr_creature.clone());

            filters_cache
                .filtered_by_level
                .entry(curr_creature.level)
                .or_insert_with(Vec::new)
                .push(curr_creature.clone());

            filters_cache
                .filtered_by_family
                .entry(curr_creature.family.clone())
                .or_insert_with(Vec::new)
                .push(curr_creature.clone());

            filters_cache
                .filtered_by_alignment
                .entry(curr_creature.alignment.clone())
                .or_insert_with(Vec::new)
                .push(curr_creature.clone());

            filters_cache
                .filtered_by_size
                .entry(curr_creature.size.clone())
                .or_insert_with(Vec::new)
                .push(curr_creature.clone());

            filters_cache
                .filtered_by_rarity
                .entry(curr_creature.rarity.clone())
                .or_insert_with(Vec::new)
                .push(curr_creature.clone());

            filters_cache
                .filtered_by_melee
                .entry(curr_creature.is_melee)
                .or_insert_with(Vec::new)
                .push(curr_creature.clone());

            filters_cache
                .filtered_by_ranged
                .entry(curr_creature.is_ranged)
                .or_insert_with(Vec::new)
                .push(curr_creature.clone());

            filters_cache
                .filtered_by_spell_caster
                .entry(curr_creature.is_spell_caster)
                .or_insert_with(Vec::new)
                .push(curr_creature.clone());
        }
    }
    filters_cache
}

fn from_db_data_to_sorted_vectors(data: &Result<Vec<Creature>, RedisError>) -> SortedVectorCache {
    let mut sorted_cache = SortedVectorCache::default();
    if let Ok(unordered_creatures) = data {
        // NEEDS TO BE OPTIMIZED, I'M DYING LOOKING AT THIS
        let mut sort_stage = unordered_creatures.clone();

        sorted_cache.unordered_creatures = unordered_creatures.clone();

        sort_stage.sort_by_key(|cr| cr.id.clone());
        sorted_cache.order_by_id_ascending = sort_stage.clone();
        sort_stage.reverse();
        sorted_cache.order_by_id_descending = sort_stage.clone();

        sort_stage.sort_by_key(|cr| cr.name.clone());
        sorted_cache.order_by_name_ascending = sort_stage.clone();
        sort_stage.reverse();
        sorted_cache.order_by_name_descending = sort_stage.clone();

        sort_stage.sort_by_key(|cr| cr.hp);
        sorted_cache.order_by_hp_ascending = sort_stage.clone();
        sort_stage.reverse();
        sorted_cache.order_by_hp_descending = sort_stage.clone();

        sort_stage.sort_by_key(|cr| cr.level);
        sorted_cache.order_by_level_ascending = sort_stage.clone();
        sort_stage.reverse();
        sorted_cache.order_by_level_descending = sort_stage.clone();

        sort_stage.sort_by_key(|cr| cr.family.clone());
        sorted_cache.order_by_family_ascending = sort_stage.clone();
        sort_stage.reverse();
        sorted_cache.order_by_family_descending = sort_stage.clone();

        sort_stage.sort_by_key(|cr| cr.alignment.clone());
        sorted_cache.order_by_alignment_ascending = sort_stage.clone();
        sort_stage.reverse();
        sorted_cache.order_by_alignment_descending = sort_stage.clone();

        sort_stage.sort_by_key(|cr| cr.size.clone());
        sorted_cache.order_by_size_ascending = sort_stage.clone();
        sort_stage.reverse();
        sorted_cache.order_by_size_descending = sort_stage.clone();

        sort_stage.sort_by_key(|cr| cr.rarity.clone());
        sorted_cache.order_by_rarity_ascending = sort_stage.clone();
        sort_stage.reverse();
        sorted_cache.order_by_rarity_descending = sort_stage.clone();
    }
    sorted_cache
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

fn fetch_data_from_database() -> Result<Vec<Creature>, RedisError> {
    db_communicator::get_creatures_by_ids(db_communicator::fetch_and_parse_all_keys(
        &"creature:".to_string(),
    )?)
}

// Function used to fetch data

fn get_list(sort_field: Option<SortEnum>, order_by: Option<OrderEnum>) -> Option<Vec<Creature>> {
    log::info!("Before getting list");
    let x = CACHE.lock();
    log::info!("getting them cache lock");
    if x.is_ok() {
        log::info!("Got cache lock, pattern matching rn");
        let cache = x.unwrap();
        match (sort_field.unwrap_or_default(), order_by.unwrap_or_default()) {
            (SortEnum::ID, OrderEnum::ASCENDING) => {
                Some(cache.lists.order_by_id_ascending.to_owned())
            }
            (SortEnum::ID, OrderEnum::DESCENDING) => {
                Some(cache.lists.order_by_id_descending.to_owned())
            }

            (SortEnum::HP, OrderEnum::ASCENDING) => {
                Some(cache.lists.order_by_hp_ascending.to_owned())
            }
            (SortEnum::HP, OrderEnum::DESCENDING) => {
                Some(cache.lists.order_by_hp_descending.to_owned())
            }

            (SortEnum::FAMILY, OrderEnum::ASCENDING) => {
                Some(cache.lists.order_by_family_ascending.to_owned())
            }
            (SortEnum::FAMILY, OrderEnum::DESCENDING) => {
                Some(cache.lists.order_by_family_descending.to_owned())
            }

            (SortEnum::ALIGNMENT, OrderEnum::ASCENDING) => {
                Some(cache.lists.order_by_alignment_ascending.to_owned())
            }
            (SortEnum::ALIGNMENT, OrderEnum::DESCENDING) => {
                Some(cache.lists.order_by_alignment_descending.to_owned())
            }

            (SortEnum::LEVEL, OrderEnum::ASCENDING) => {
                Some(cache.lists.order_by_level_ascending.to_owned())
            }
            (SortEnum::LEVEL, OrderEnum::DESCENDING) => {
                Some(cache.lists.order_by_level_descending.to_owned())
            }

            (SortEnum::NAME, OrderEnum::ASCENDING) => {
                Some(cache.lists.order_by_name_ascending.to_owned())
            }
            (SortEnum::NAME, OrderEnum::DESCENDING) => {
                Some(cache.lists.order_by_name_descending.to_owned())
            }

            (SortEnum::RARITY, OrderEnum::ASCENDING) => {
                Some(cache.lists.order_by_rarity_ascending.to_owned())
            }
            (SortEnum::RARITY, OrderEnum::DESCENDING) => {
                Some(cache.lists.order_by_rarity_descending.to_owned())
            }

            (SortEnum::SIZE, OrderEnum::ASCENDING) => {
                Some(cache.lists.order_by_size_ascending.to_owned())
            }
            (SortEnum::SIZE, OrderEnum::DESCENDING) => {
                Some(cache.lists.order_by_rarity_descending.to_owned())
            }

            _ => Some(cache.lists.unordered_creatures.to_owned()),
        }
    } else {
        None
    }
}

pub fn get_paginated_creatures<'a>(
    sort: &SortData,
    filters: &FieldFilters,
    pagination: &PaginatedRequest,
) -> Option<(u32, Vec<Creature>)> {
    log::info!("We getting them paginated creatures");
    match get_list(sort.sort_key, sort.order_by) {
        Some(list) => {
            println!("{:?}", list);
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

fn check_creature_pass_filters(creature: &Creature, filters: &FieldFilters) -> bool {
    check_creature_pass_equality_filters(creature, filters)
        && check_creature_pass_greater_filters(creature, filters)
        && check_creature_pass_lesser_filters(creature, filters)
        && check_creature_pass_string_filters(creature, filters)
}

fn check_creature_pass_greater_filters(creature: &Creature, filters: &FieldFilters) -> bool {
    let hp_pass = filters
        .max_hp_filter
        .map_or(true, |max_hp| creature.hp <= max_hp);

    let level_pass = filters
        .max_level_filter
        .map_or(true, |max_lvl| creature.level <= max_lvl);

    hp_pass && level_pass
}

fn check_creature_pass_lesser_filters(creature: &Creature, filters: &FieldFilters) -> bool {
    let hp_pass = filters
        .min_hp_filter
        .map_or(true, |min_hp| creature.hp >= min_hp);

    let level_pass = filters
        .min_level_filter
        .map_or(true, |min_lvl| creature.level >= min_lvl);

    hp_pass && level_pass
}

fn check_creature_pass_equality_filters(creature: &Creature, filters: &FieldFilters) -> bool {
    let rarity_pass = filters
        .rarity_filter
        .as_ref()
        .map_or(true, |rarity| creature.rarity == *rarity);
    let size_pass = filters
        .size_filter
        .as_ref()
        .map_or(true, |size| creature.size == *size);
    let alignment_pass = filters
        .alignment_filter
        .as_ref()
        .map_or(true, |alignment| creature.alignment == *alignment);
    let is_melee_pass = filters
        .is_melee_filter
        .map_or(true, |is_melee| creature.is_melee == is_melee);
    let is_ranged_pass = filters
        .is_ranged_filter
        .map_or(true, |is_ranged| creature.is_ranged == is_ranged);
    let is_spell_caster_pass = filters
        .is_spell_caster_filter
        .map_or(true, |is_spell_caster| {
            creature.is_spell_caster == is_spell_caster
        });

    rarity_pass
        && size_pass
        && alignment_pass
        && is_melee_pass
        && is_ranged_pass
        && is_spell_caster_pass
}

fn check_creature_pass_string_filters(creature: &Creature, filters: &FieldFilters) -> bool {
    let name_pass: bool = filters
        .name_filter
        .as_ref()
        .map_or(true, |name| creature.name.to_lowercase().contains(name));

    let family_pass: bool = filters.family_filter.as_ref().map_or(true, |name| {
        creature
            .family
            .as_ref()
            .unwrap_or(&"".to_string())
            .to_lowercase()
            .contains(name)
    });
    name_pass && family_pass
}
