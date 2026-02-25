use crate::AppState;
use crate::db::hazard_proxy;

use crate::db::hazard_proxy::get_hazards_passing_all_filters;
use crate::models::hazard::hazard_field_filter::HazardFieldFilters;
use crate::models::hazard::hazard_listing_struct::{
    HazardFilterQuery, HazardListingPaginatedRequest,
};
use crate::models::hazard::hazard_struct::Hazard;
use crate::models::response_data::ResponseHazard;
use crate::models::shared::game_system_enum::GameSystem;
use crate::services::url_calculator::hazard_listing_next_url;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct HazardListingResponse {
    results: Option<Vec<ResponseHazard>>,
    count: usize,
    total: usize,
    next: Option<String>,
}

pub async fn get_hazard(
    app_state: &AppState,
    id: i64,
    gs: &GameSystem,
) -> HashMap<String, Option<ResponseHazard>> {
    hashmap! {
        String::from("results") =>
        hazard_proxy::get_hazard_by_id(app_state, gs, id).await
    }
}

pub async fn get_hazard_listing(
    app_state: &AppState,
    field_filter: &HazardFieldFilters,
    pagination: &HazardListingPaginatedRequest,
    gs: &GameSystem,
) -> HazardListingResponse {
    convert_result_to_hazard_response(
        field_filter,
        pagination,
        hazard_proxy::get_paginated_hazards(app_state, gs, field_filter, pagination).await,
    )
}

pub async fn get_traits_list(app_state: &AppState, gs: &GameSystem) -> Vec<String> {
    hazard_proxy::get_all_traits(app_state, gs).await
}

pub async fn get_sources_list(app_state: &AppState, gs: &GameSystem) -> Vec<String> {
    hazard_proxy::get_all_sources(app_state, gs).await
}

pub async fn get_rarities_list(app_state: &AppState, gs: &GameSystem) -> Vec<String> {
    hazard_proxy::get_all_rarities(app_state, gs).await
}

pub async fn get_sizes_list(app_state: &AppState, gs: &GameSystem) -> Vec<String> {
    hazard_proxy::get_all_sizes(app_state, gs).await
}
fn convert_result_to_hazard_response(
    field_filters: &HazardFieldFilters,
    pagination: &HazardListingPaginatedRequest,
    result: Result<(u32, Vec<ResponseHazard>)>,
) -> HazardListingResponse {
    match result {
        Ok(res) => {
            let cr: Vec<ResponseHazard> = res.1;
            let cr_length = cr.len();
            HazardListingResponse {
                results: Some(cr),
                count: cr_length,
                next: if cr_length >= pagination.paginated_request.page_size.unsigned_abs() as usize
                {
                    Some(hazard_listing_next_url(
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
        Err(_) => HazardListingResponse {
            results: None,
            count: 0,
            total: 0,
            next: None,
        },
    }
}

pub async fn get_filtered_hazards(
    app_state: &AppState,
    filters: &HazardFilterQuery,
    gs: &GameSystem,
) -> Result<Vec<Hazard>> {
    get_hazards_passing_all_filters(app_state, gs, filters).await
}
