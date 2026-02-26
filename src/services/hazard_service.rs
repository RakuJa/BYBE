use crate::AppState;
use crate::db::hazard_proxy;

use crate::db::hazard_proxy::get_hazards_passing_all_filters;
use crate::models::hazard::hazard_field_filter::HazardFieldFilters;
use crate::models::hazard::hazard_listing_struct::{
    HazardFilterQuery, HazardListingPaginatedRequest,
};
use crate::models::hazard::hazard_struct::Hazard;
use crate::models::response_data::{
    HazardListingResponse, ResponseHazard, convert_result_to_response,
};
use crate::models::shared::game_system_enum::GameSystem;
use anyhow::Result;
use std::collections::HashMap;

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
    convert_result_to_response(
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

pub async fn get_filtered_hazards(
    app_state: &AppState,
    filters: &HazardFilterQuery,
    gs: &GameSystem,
) -> Result<Vec<Hazard>> {
    get_hazards_passing_all_filters(app_state, gs, filters).await
}
