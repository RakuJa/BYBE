use crate::db::proxy;
use crate::models::creature::Creature;
use crate::models::creature_fields_enum::CreatureField;
use crate::models::creature_metadata::creature_role::CreatureRoleEnum;
use crate::models::creature_metadata::variant_enum::CreatureVariant;
use crate::models::response_data::{OptionalData, ResponseCreature};
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
    id: i64,
    optional_data: &OptionalData,
) -> HashMap<String, Option<ResponseCreature>> {
    hashmap! {
        String::from("results") =>
        proxy::get_creature_by_id(app_state, id, &CreatureVariant::Base, optional_data).await.map(ResponseCreature::from)
    }
}

pub async fn get_elite_creature(
    app_state: &AppState,
    id: i64,
    optional_data: &OptionalData,
) -> HashMap<String, Option<ResponseCreature>> {
    hashmap! {
        String::from("results") =>
        proxy::get_elite_creature_by_id(app_state, id, optional_data).await.map(ResponseCreature::from)
    }
}

pub async fn get_weak_creature(
    app_state: &AppState,
    id: i64,
    optional_data: &OptionalData,
) -> HashMap<String, Option<ResponseCreature>> {
    hashmap! {
        String::from("results") =>
        proxy::get_weak_creature_by_id(app_state, id, optional_data).await.map(ResponseCreature::from)
    }
}

pub async fn get_bestiary(
    app_state: &AppState,
    field_filter: &FieldFilters,
    pagination: &PaginatedRequest,
) -> BestiaryResponse {
    convert_result_to_bestiary_response(
        field_filter,
        pagination,
        proxy::get_paginated_creatures(app_state, field_filter, pagination).await,
    )
}

pub async fn get_families_list(app_state: &AppState) -> Vec<String> {
    proxy::get_keys(app_state, CreatureField::Family).await
}

pub async fn get_traits_list(app_state: &AppState) -> Vec<String> {
    proxy::get_keys(app_state, CreatureField::Traits).await
}

pub async fn get_sources_list(app_state: &AppState) -> Vec<String> {
    proxy::get_keys(app_state, CreatureField::Sources).await
}

pub async fn get_rarities_list(app_state: &AppState) -> Vec<String> {
    proxy::get_keys(app_state, CreatureField::Rarity).await
}

pub async fn get_sizes_list(app_state: &AppState) -> Vec<String> {
    proxy::get_keys(app_state, CreatureField::Size).await
}

pub async fn get_alignments_list(app_state: &AppState) -> Vec<String> {
    proxy::get_keys(app_state, CreatureField::Alignment).await
}

pub async fn get_creature_types_list(app_state: &AppState) -> Vec<String> {
    proxy::get_keys(app_state, CreatureField::CreatureTypes).await
}
//

pub async fn get_creature_roles_list() -> Vec<String> {
    CreatureRoleEnum::list()
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
                results: Some(cr.into_iter().map(ResponseCreature::from).collect()),
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
