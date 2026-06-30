use crate::AppState;
use crate::db::data_providers::hazard_fetcher::fetch_traits_associated_with_hazards;
use crate::db::data_providers::{fetch_unique_values_from_db, hazard_fetcher};
use crate::models::hazard::hazard_field_filter::HazardFieldFilters;
use crate::models::hazard::hazard_listing_struct::{
    HazardFilterQuery, HazardListingPaginatedRequest,
};
use crate::models::hazard::hazard_struct::{Hazard, HazardRanges};
use crate::models::response_data::ResponseHazard;
use crate::models::shared::game_system_enum::GameSystem;
use anyhow::Result;
#[cfg(feature = "cache")]
use cached::cached;

pub async fn get_hazard_by_id(
    app_state: &AppState,
    gs: GameSystem,
    id: i64,
) -> Option<ResponseHazard> {
    hazard_fetcher::fetch_hazard_by_id(&app_state.pool, gs, id)
        .await
        .ok()
}

pub async fn get_hazards_passing_all_filters(
    app_state: &AppState,
    gs: GameSystem,
    filters: &HazardFilterQuery,
) -> Result<Vec<Hazard>> {
    hazard_fetcher::fetch_hazard_core_data_with_filters(&app_state.pool, gs, filters).await
}

pub async fn get_paginated_hazards(
    app_state: &AppState,
    gs: GameSystem,
    filters: &HazardFieldFilters,
    pagination: &HazardListingPaginatedRequest,
) -> Result<(u32, Vec<ResponseHazard>)> {
    let (hazards, count) = hazard_fetcher::fetch_paginated_hazards(
        &app_state.pool,
        gs,
        filters,
        pagination
            .hazard_sort_data
            .sort_by
            .clone()
            .unwrap_or_default(),
        pagination.hazard_sort_data.order_by.unwrap_or_default(),
        pagination.paginated_request.cursor,
        pagination.paginated_request.page_size,
    )
    .await?;
    let count = count as u32;
    let response_hazards = hazards
        .into_iter()
        .map(|h| ResponseHazard {
            core_hazard: h,
            game: gs,
        })
        .collect();
    Ok((count, response_hazards))
}

#[cfg_attr(feature = "cache", cached(key = "i64", convert = r##"{ gs.into() }"##))]
pub async fn get_all_sources(app_state: &AppState, gs: GameSystem) -> Vec<String> {
    fetch_unique_values_from_db(app_state, format!("{gs}_hazard_table"), "source".into()).await
}

#[cfg_attr(feature = "cache", cached(key = "i64", convert = r##"{ gs.into() }"##))]
pub async fn get_all_rarities(app_state: &AppState, gs: GameSystem) -> Vec<String> {
    fetch_unique_values_from_db(app_state, format!("{gs}_hazard_table"), "rarity".into()).await
}
#[cfg_attr(feature = "cache", cached(key = "i64", convert = r##"{ gs.into() }"##))]
pub async fn get_all_sizes(app_state: &AppState, gs: GameSystem) -> Vec<String> {
    fetch_unique_values_from_db(app_state, format!("{gs}_hazard_table"), "size".into()).await
}
#[cfg_attr(feature = "cache", cached(key = "i64", convert = r##"{ gs.into() }"##))]
pub async fn get_all_traits(app_state: &AppState, gs: GameSystem) -> Vec<String> {
    fetch_traits_associated_with_hazards(&app_state.pool, gs)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|x| x.name)
        .collect()
}
#[cfg_attr(feature = "cache", cached(key = "i64", convert = r##"{ gs.into() }"##))]
pub async fn get_hazard_ranges(app_state: &AppState, gs: GameSystem) -> Option<HazardRanges> {
    hazard_fetcher::fetch_hazard_ranges(&app_state.pool, gs)
        .await
        .ok()
}
