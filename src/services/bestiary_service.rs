use crate::db::bestiary_proxy;
use crate::models::bestiary_structs::BestiaryPaginatedRequest;
use crate::models::creature::creature_filter_enum::CreatureFilter;
use crate::models::creature::creature_metadata::creature_role::CreatureRoleEnum;
use crate::models::creature::creature_metadata::variant_enum::CreatureVariant;
use crate::models::creature::creature_struct::Creature;
use crate::models::response_data::{ResponseCreature, ResponseDataModifiers};
use crate::models::routers_validator_structs::CreatureFieldFilters;
use crate::services::url_calculator::bestiary_next_url;
use crate::AppState;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct BestiaryResponse {
    results: Option<Vec<ResponseCreature>>,
    count: usize,
    total: usize,
    next: Option<String>,
}

pub async fn get_creature(
    app_state: &AppState,
    id: i64,
    response_data_mods: &ResponseDataModifiers,
) -> HashMap<String, Option<ResponseCreature>> {
    hashmap! {
        String::from("results") =>
        bestiary_proxy::get_creature_by_id(app_state, id, CreatureVariant::Base, response_data_mods).await.map(ResponseCreature::from)
    }
}

pub async fn get_elite_creature(
    app_state: &AppState,
    id: i64,
    response_data_mods: &ResponseDataModifiers,
) -> HashMap<String, Option<ResponseCreature>> {
    hashmap! {
        String::from("results") =>
        bestiary_proxy::get_elite_creature_by_id(app_state, id, response_data_mods).await.map(ResponseCreature::from)
    }
}

pub async fn get_weak_creature(
    app_state: &AppState,
    id: i64,
    response_data_mods: &ResponseDataModifiers,
) -> HashMap<String, Option<ResponseCreature>> {
    hashmap! {
        String::from("results") =>
        bestiary_proxy::get_weak_creature_by_id(app_state, id, response_data_mods).await.map(ResponseCreature::from)
    }
}

pub async fn get_bestiary_listing(
    app_state: &AppState,
    field_filter: &CreatureFieldFilters,
    pagination: &BestiaryPaginatedRequest,
) -> BestiaryResponse {
    convert_result_to_bestiary_response(
        field_filter,
        pagination,
        bestiary_proxy::get_paginated_creatures(app_state, field_filter, pagination).await,
    )
}

pub async fn get_families_list(app_state: &AppState) -> Vec<String> {
    bestiary_proxy::get_all_possible_values_of_filter(app_state, CreatureFilter::Family).await
}

pub async fn get_traits_list(app_state: &AppState) -> Vec<String> {
    bestiary_proxy::get_all_possible_values_of_filter(app_state, CreatureFilter::Traits).await
}

pub async fn get_sources_list(app_state: &AppState) -> Vec<String> {
    bestiary_proxy::get_all_possible_values_of_filter(app_state, CreatureFilter::Sources).await
}

pub async fn get_rarities_list(app_state: &AppState) -> Vec<String> {
    bestiary_proxy::get_all_possible_values_of_filter(app_state, CreatureFilter::Rarity).await
}

pub async fn get_sizes_list(app_state: &AppState) -> Vec<String> {
    bestiary_proxy::get_all_possible_values_of_filter(app_state, CreatureFilter::Size).await
}

pub async fn get_alignments_list(app_state: &AppState) -> Vec<String> {
    bestiary_proxy::get_all_possible_values_of_filter(app_state, CreatureFilter::Alignment).await
}

pub async fn get_creature_types_list(app_state: &AppState) -> Vec<String> {
    bestiary_proxy::get_all_possible_values_of_filter(app_state, CreatureFilter::CreatureTypes)
        .await
}

pub fn get_creature_roles_list() -> Vec<String> {
    CreatureRoleEnum::list()
}
fn convert_result_to_bestiary_response(
    field_filters: &CreatureFieldFilters,
    pagination: &BestiaryPaginatedRequest,
    result: Result<(u32, Vec<Creature>)>,
) -> BestiaryResponse {
    match result {
        Ok(res) => {
            let cr: Vec<Creature> = res.1;
            let cr_length = cr.len();
            BestiaryResponse {
                results: Some(cr.into_iter().map(ResponseCreature::from).collect()),
                count: cr_length,
                next: if cr_length >= pagination.paginated_request.page_size.unsigned_abs() as usize
                {
                    Some(bestiary_next_url(
                        field_filters,
                        pagination,
                        cr_length as u32,
                    ))
                } else {
                    None
                },
                total: res.0 as usize,
            }
        }
        Err(_) => BestiaryResponse {
            results: None,
            count: 0,
            total: 0,
            next: None,
        },
    }
}
