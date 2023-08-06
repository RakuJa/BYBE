use crate::db::db_communicator::get_creature_by_id;
use crate::db::db_proxy;
use crate::models::creature::Creature;
use crate::models::creature_fields_enum::CreatureField;
use crate::models::routers_validator_structs::{FieldFilters, PaginatedRequest, SortData};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BestiaryResponse {
    results: Option<Vec<Creature>>,
    count: usize,
    next: Option<String>,
}

pub async fn get_creature(id: &String) -> Option<Creature> {
    match get_creature_by_id(id) {
        Ok(cr) => hashmap! {String::from("results") => Some(cr)},
        _ => hashmap! {String::from("results") => None},
    };
    match get_creature_by_id(id) {
        Ok(cr) => Some(cr),
        _ => None,
    }
}

pub fn get_bestiary(
    sort_field: &SortData,
    field_filter: &FieldFilters,
    pagination: &PaginatedRequest,
) -> BestiaryResponse {
    convert_result_to_bestiary_response(
        sort_field,
        field_filter,
        pagination,
        db_proxy::get_paginated_creatures(sort_field, field_filter, pagination),
    )
}

pub fn get_families_list() -> Vec<String> {
    db_proxy::get_keys(CreatureField::Family)
}

pub fn get_rarities_list() -> Vec<String> {
    db_proxy::get_keys(CreatureField::Rarity)
}

pub fn get_size_list() -> Vec<String> {
    db_proxy::get_keys(CreatureField::Size)
}

pub fn get_alignment_list() -> Vec<String> {
    db_proxy::get_keys(CreatureField::Alignment)
}

fn next_url_calculator(
    sort_field: &SortData,
    field_filters: &FieldFilters,
    pagination: &PaginatedRequest,
    next_cursor: u32,
) -> String {
    let base_url = "https://bybe.fly.dev/bestiary/list/"; //"0.0.0.0:25566/list/"
    let sort_query = format!(
        "?sort_key={}&order_by={}",
        sort_field.sort_key.unwrap_or_default(),
        sort_field.order_by.unwrap_or_default()
    );
    let filter_query = filter_query_calculator(field_filters);

    let pagination_query = format!("&cursor={}&page_size={}", next_cursor, pagination.page_size);
    format!(
        "{}{}{}{}",
        base_url, sort_query, filter_query, pagination_query
    )
}

fn filter_query_calculator(field_filters: &FieldFilters) -> String {
    let queries: Vec<String> = [
        field_filters
            .family_filter
            .clone()
            .map(|fam| format!("family_filter={}", fam)),
        field_filters
            .name_filter
            .clone()
            .map(|name| format!("name_filter={}", name)),
        field_filters
            .rarity_filter
            .clone()
            .map(|rar| format!("rarity_filter={}", rar)),
        field_filters
            .size_filter
            .clone()
            .map(|size| format!("size_filter={}", size)),
        field_filters
            .alignment_filter
            .clone()
            .map(|align| format!("alignment_filter={}", align)),
        field_filters
            .min_hp_filter
            .map(|hp| format!("min_hp_filter={}", hp)),
        field_filters
            .max_hp_filter
            .map(|hp| format!("max_hp_filter={}", hp)),
        field_filters
            .min_hp_filter
            .map(|lvl| format!("min_level_filter={}", lvl)),
        field_filters
            .max_hp_filter
            .map(|lvl| format!("max_level_filter={}", lvl)),
        field_filters
            .is_melee_filter
            .map(|is| format!("is_melee_filter={}", is)),
        field_filters
            .is_ranged_filter
            .map(|is| format!("is_ranged_filter={}", is)),
        field_filters
            .is_spell_caster_filter
            .map(|is| format!("is_spell_caster_filter={}", is)),
    ]
    .iter()
    .filter_map(|opt| opt.clone())
    .collect();
    match queries.len() {
        0 => String::new(),
        _ => format!("{}{}", "&", queries.join("&")),
    }
}

fn convert_result_to_bestiary_response(
    sort_field: &SortData,
    field_filters: &FieldFilters,
    pagination: &PaginatedRequest,
    result: Option<(u32, Vec<Creature>)>,
) -> BestiaryResponse {
    match result {
        Some(res) => {
            let cr: Vec<Creature> = res.1;
            let cr_length = cr.len();
            BestiaryResponse {
                results: Some(cr),
                count: cr_length,
                next: if cr_length >= pagination.page_size as usize {
                    Some(next_url_calculator(
                        sort_field,
                        field_filters,
                        pagination,
                        res.0,
                    ))
                } else {
                    None
                },
            }
        }
        None => BestiaryResponse {
            results: None,
            count: 0,
            next: None,
        },
    }
}
