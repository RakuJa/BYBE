use crate::AppState;
use crate::db::data_providers::shop_fetcher;
use crate::models::item::item_field_filter::ItemFieldFilters;
use crate::models::item::item_struct::Item;
use crate::models::item::shop_structs::{ShopFilterQuery, ShopPaginatedRequest, ShopRanges};
use crate::models::response_data::ResponseItem;
use crate::models::shared::game_system_enum::GameSystem;
use anyhow::Result;
#[cfg(feature = "cache")]
use cached::cached;

pub async fn get_item_by_id(app_state: &AppState, gs: GameSystem, id: i64) -> Option<ResponseItem> {
    shop_fetcher::fetch_item_by_id(&app_state.pool, gs, id)
        .await
        .ok()
}

pub async fn get_filtered_items(
    app_state: &AppState,
    gs: GameSystem,
    filters: &ShopFilterQuery,
) -> Result<Vec<Item>> {
    shop_fetcher::fetch_items_with_filters(&app_state.pool, gs, filters).await
}

pub async fn get_paginated_items(
    app_state: &AppState,
    gs: GameSystem,
    filters: &ItemFieldFilters,
    pagination: &ShopPaginatedRequest,
) -> Result<(u32, Vec<ResponseItem>)> {
    let (items, count) = shop_fetcher::fetch_paginated_items(
        &app_state.pool,
        gs,
        filters,
        pagination.shop_sort_data.sort_by.unwrap_or_default(),
        pagination.shop_sort_data.order_by.unwrap_or_default(),
        pagination.paginated_request.cursor,
        pagination.paginated_request.page_size,
    )
    .await?;
    Ok((count as u32, items))
}

#[cfg_attr(feature = "cache", cached(key = "i64", convert = r##"{ gs.into() }"##))]
pub async fn get_shop_ranges(app_state: &AppState, gs: GameSystem) -> Option<ShopRanges> {
    shop_fetcher::fetch_shop_ranges(&app_state.pool, gs)
        .await
        .ok()
}

#[cfg_attr(feature = "cache", cached(key = "i64", convert = r##"{ gs.into() }"##))]
pub async fn get_all_sources(app_state: &AppState, gs: GameSystem) -> Vec<String> {
    shop_fetcher::fetch_shop_all_sources(&app_state.pool, gs)
        .await
        .unwrap_or_default()
}

#[cfg_attr(feature = "cache", cached(key = "i64", convert = r##"{ gs.into() }"##))]
pub async fn get_all_traits(app_state: &AppState, gs: GameSystem) -> Vec<String> {
    shop_fetcher::fetch_shop_all_traits(&app_state.pool, gs)
        .await
        .unwrap_or_default()
}
