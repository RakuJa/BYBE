use crate::db::db_proxy;
use crate::db::db_proxy::get_creature_by_id;
use crate::models::creature::Creature;
use crate::models::creature_fields_enum::CreatureField;
use crate::models::routers_validator_structs::{FieldFilters, PaginatedRequest};
use crate::services::url_calculator::{add_boolean_query, next_url_calculator};
use crate::AppState;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct BestiaryResponse {
    results: Option<Vec<Creature>>,
    count: usize,
    next: Option<String>,
}

pub async fn get_creature(app_state: &AppState, id: i32) -> HashMap<String, Option<Creature>> {
    hashmap! {String::from("results") => get_creature_by_id(app_state, id).await}
}

pub async fn get_elite_creature(
    app_state: &AppState,
    id: i32,
) -> HashMap<String, Option<Creature>> {
    hashmap! {String::from("results") => update_creature(app_state, id, 1).await}
}

pub async fn get_weak_creature(app_state: &AppState, id: i32) -> HashMap<String, Option<Creature>> {
    hashmap! {String::from("results") => update_creature(app_state, id, -1).await}
}

pub async fn get_bestiary(
    app_state: &AppState,
    field_filter: &FieldFilters,
    pagination: &PaginatedRequest,
) -> BestiaryResponse {
    convert_result_to_bestiary_response(
        field_filter,
        pagination,
        db_proxy::get_paginated_creatures(app_state, field_filter, pagination).await,
    )
}

pub async fn get_families_list(app_state: &AppState) -> Vec<String> {
    db_proxy::get_keys(app_state, CreatureField::Family).await
}

pub async fn get_traits_list(app_state: &AppState) -> Vec<String> {
    db_proxy::get_keys(app_state, CreatureField::Traits).await
}

pub async fn get_sources_list(app_state: &AppState) -> Vec<String> {
    db_proxy::get_keys(app_state, CreatureField::Sources).await
}

pub async fn get_rarities_list(app_state: &AppState) -> Vec<String> {
    db_proxy::get_keys(app_state, CreatureField::Rarity).await
}

pub async fn get_sizes_list(app_state: &AppState) -> Vec<String> {
    db_proxy::get_keys(app_state, CreatureField::Size).await
}

pub async fn get_alignments_list(app_state: &AppState) -> Vec<String> {
    db_proxy::get_keys(app_state, CreatureField::Alignment).await
}

pub async fn get_creature_types_list(app_state: &AppState) -> Vec<String> {
    db_proxy::get_keys(app_state, CreatureField::CreatureTypes).await
}

fn hp_increase_by_level() -> HashMap<i8, u16> {
    hashmap! { 1 => 10, 2=> 15, 5=> 20, 20=> 30 }
}

async fn update_creature(app_state: &AppState, id: i32, level_delta: i8) -> Option<Creature> {
    match get_creature_by_id(app_state, id).await {
        Some(mut creature) => {
            let hp_increase = hp_increase_by_level();
            let desired_key = hp_increase
                .keys()
                .filter(|&&lvl| creature.level >= lvl)
                .max()
                .unwrap_or(hp_increase.keys().next().unwrap_or(&0));
            creature.hp += *hp_increase.get(desired_key).unwrap_or(&0) as i16 * level_delta as i16;
            creature.hp = creature.hp.max(1);

            creature.level += level_delta;

            creature.archive_link = add_boolean_query(
                &creature.archive_link,
                &String::from(if level_delta >= 1 { "Elite" } else { "Weak" }),
                true,
            );
            Some(creature)
        }
        None => None,
    }
}

fn convert_result_to_bestiary_response(
    field_filters: &FieldFilters,
    pagination: &PaginatedRequest,
    result: Result<(u32, Vec<Creature>)>,
) -> BestiaryResponse {
    match result {
        Ok(res) => {
            let cr: Vec<Creature> = res.1;
            let cr_length = cr.len();
            BestiaryResponse {
                results: Some(cr),
                count: cr_length,
                next: if cr_length >= pagination.page_size as usize {
                    Some(next_url_calculator(field_filters, pagination, res.0))
                } else {
                    None
                },
            }
        }
        Err(_) => BestiaryResponse {
            results: None,
            count: 0,
            next: None,
        },
    }
}
