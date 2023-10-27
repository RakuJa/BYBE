use crate::db::db_communicator;
use crate::models::creature::{check_creature_pass_filters, Creature};
use std::collections::{HashMap, HashSet};

use crate::db::db_cache::{from_db_data_to_filter_cache, from_db_data_to_sorted_vectors};
use crate::models::creature_fields_enum::CreatureField;
use crate::models::creature_filter_enum::CreatureFilter;
use crate::models::creature_sort_enums::{OrderEnum, SortEnum};
use crate::models::routers_validator_structs::{FieldFilters, PaginatedRequest, SortData};
use anyhow::Result;
use cached::proc_macro::cached;

pub fn get_creature_by_id(id: i32) -> Option<Creature> {
    let list = get_list(None, None);
    list.iter().find(|creature| creature.id == id).cloned()
}

pub fn get_paginated_creatures(
    sort: &SortData,
    filters: &FieldFilters,
    pagination: &PaginatedRequest,
) -> Result<(u32, Vec<Creature>)> {
    let list = get_list(sort.sort_key, sort.order_by);

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
    let creature_list = get_list(None, None);
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

#[cached(time = 604800, sync_writes = true)]
pub fn get_keys(field: CreatureField) -> Vec<String> {
    if let Some(db_data) = fetch_data_from_database() {
        let runtime_fields_values = from_db_data_to_filter_cache(db_data);
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

#[cached(time = 604800, sync_writes = true)]
fn fetch_data_from_database() -> Option<Vec<Creature>> {
    if let Some(monster_vec) = fetch_creatures("monster:") {
        if let Some(mut npc_vec) = fetch_creatures("npc:") {
            let mut creature_vec = monster_vec;
            creature_vec.append(&mut npc_vec);
            return Some(creature_vec);
        }
    }
    None
}

fn fetch_creatures(pattern: &str) -> Option<Vec<Creature>> {
    if let Ok(keys) = db_communicator::fetch_and_parse_all_keys(pattern) {
        if let Ok(creatures) = db_communicator::get_creatures_by_ids(keys) {
            return Some(creatures);
        }
    }
    None
}

#[cached(time = 604800, sync_writes = true)]
fn get_list(sort_field: Option<SortEnum>, order_by: Option<OrderEnum>) -> Vec<Creature> {
    if let Some(db_data) = fetch_data_from_database() {
        let sorted_vec_cache = from_db_data_to_sorted_vectors(db_data);
        let x = match (sort_field.unwrap_or_default(), order_by.unwrap_or_default()) {
            (SortEnum::Id, OrderEnum::Ascending) => {
                sorted_vec_cache.order_by_id_ascending.to_owned()
            }
            (SortEnum::Id, OrderEnum::Descending) => {
                sorted_vec_cache.order_by_id_descending.to_owned()
            }

            (SortEnum::Hp, OrderEnum::Ascending) => {
                sorted_vec_cache.order_by_hp_ascending.to_owned()
            }
            (SortEnum::Hp, OrderEnum::Descending) => {
                sorted_vec_cache.order_by_hp_descending.to_owned()
            }

            (SortEnum::Family, OrderEnum::Ascending) => {
                sorted_vec_cache.order_by_family_ascending.to_owned()
            }
            (SortEnum::Family, OrderEnum::Descending) => {
                sorted_vec_cache.order_by_family_descending.to_owned()
            }

            (SortEnum::Alignment, OrderEnum::Ascending) => {
                sorted_vec_cache.order_by_alignment_ascending.to_owned()
            }
            (SortEnum::Alignment, OrderEnum::Descending) => {
                sorted_vec_cache.order_by_alignment_descending.to_owned()
            }

            (SortEnum::Level, OrderEnum::Ascending) => {
                sorted_vec_cache.order_by_level_ascending.to_owned()
            }
            (SortEnum::Level, OrderEnum::Descending) => {
                sorted_vec_cache.order_by_level_descending.to_owned()
            }

            (SortEnum::Name, OrderEnum::Ascending) => {
                sorted_vec_cache.order_by_name_ascending.to_owned()
            }
            (SortEnum::Name, OrderEnum::Descending) => {
                sorted_vec_cache.order_by_name_descending.to_owned()
            }

            (SortEnum::Rarity, OrderEnum::Ascending) => {
                sorted_vec_cache.order_by_rarity_ascending.to_owned()
            }
            (SortEnum::Rarity, OrderEnum::Descending) => {
                sorted_vec_cache.order_by_rarity_descending.to_owned()
            }

            (SortEnum::Size, OrderEnum::Ascending) => {
                sorted_vec_cache.order_by_size_ascending.to_owned()
            }
            (SortEnum::Size, OrderEnum::Descending) => {
                sorted_vec_cache.order_by_rarity_descending.to_owned()
            }
        };
        return x;
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
