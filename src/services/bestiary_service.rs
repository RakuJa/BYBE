use crate::db::db_proxy;
use crate::models::creature::Creature;
use crate::models::creature_fields_enum::CreatureField;
use crate::models::creature_metadata::creature_role::CreatureRoleEnum;
use crate::models::creature_metadata::variant_enum::CreatureVariant;
use crate::models::response_data::{ResponseCreature, ResponseData};
use crate::models::routers_validator_structs::{FieldFilters, PaginatedRequest};
use crate::services::url_calculator::next_url_calculator;
use crate::AppState;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct BestiaryResponse {
    results: Option<Vec<ResponseCreature>>,
    count: usize,
    next: Option<String>,
}

pub async fn get_creature(
    app_state: &AppState,
    id: i32,
    response_data: &ResponseData,
) -> HashMap<String, Option<ResponseCreature>> {
    hashmap! {
        String::from("results") =>
        db_proxy::get_creature_by_id(app_state, id, CreatureVariant::Base).await.map(|x| ResponseCreature::from((x, response_data)))
    }
}

pub async fn get_elite_creature(
    app_state: &AppState,
    id: i32,
    response_data: &ResponseData,
) -> HashMap<String, Option<ResponseCreature>> {
    hashmap! {
        String::from("results") =>
        db_proxy::get_elite_creature_by_id(app_state, id).await.map(|x| ResponseCreature::from((x, response_data)))
    }
}

pub async fn get_weak_creature(
    app_state: &AppState,
    id: i32,
    response_data: &ResponseData,
) -> HashMap<String, Option<ResponseCreature>> {
    hashmap! {
        String::from("results") =>
        db_proxy::get_weak_creature_by_id(app_state, id).await.map(|x| ResponseCreature::from((x, response_data)))
    }
}

pub async fn get_bestiary(
    app_state: &AppState,
    field_filter: &FieldFilters,
    pagination: &PaginatedRequest,
    response_data: &ResponseData,
) -> BestiaryResponse {
    convert_result_to_bestiary_response(
        field_filter,
        pagination,
        db_proxy::get_paginated_creatures(app_state, field_filter, pagination).await,
        response_data,
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

pub async fn get_creature_roles_list() -> Vec<String> {
    CreatureRoleEnum::list()
}
fn convert_result_to_bestiary_response(
    field_filters: &FieldFilters,
    pagination: &PaginatedRequest,
    result: Result<(u32, Vec<Creature>)>,
    response_data: &ResponseData,
) -> BestiaryResponse {
    match result {
        Ok(res) => {
            let cr: Vec<Creature> = res.1;
            let cr_length = cr.len();
            BestiaryResponse {
                results: Some(
                    cr.into_iter()
                        .clone()
                        .map(|x| ResponseCreature::from((x, response_data)))
                        .collect(),
                ),
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
