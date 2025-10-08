use crate::AppState;
use crate::db::shop_proxy;
use crate::models::response_data::ResponseItem;
use crate::models::response_data::ShopListingResponse;
use crate::models::routers_validator_structs::ItemFieldFilters;
use crate::models::shared::game_system_enum::GameSystem;
use crate::models::shop_structs::{PfShopTemplateEnum, ShopPaginatedRequest, ShopTemplateData};
use crate::services::shared::url_calculator::shop_next_url;
use std::collections::HashMap;
use strum::IntoEnumIterator;

pub async fn get_item(app_state: &AppState, id: i64) -> HashMap<String, Option<ResponseItem>> {
    hashmap! {
        String::from("results") =>
        shop_proxy::get_item_by_id(app_state, &GameSystem::Pathfinder,  id).await
    }
}

pub async fn get_shop_listing(
    app_state: &AppState,
    field_filter: &ItemFieldFilters,
    pagination: &ShopPaginatedRequest,
) -> ShopListingResponse {
    convert_result_to_shop_response(
        field_filter,
        pagination,
        shop_proxy::get_paginated_items(
            app_state,
            &GameSystem::Pathfinder,
            field_filter,
            pagination,
        )
        .await,
    )
}

pub async fn get_sources_list(app_state: &AppState) -> Vec<String> {
    shop_proxy::get_all_sources(app_state, &GameSystem::Pathfinder).await
}

pub async fn get_traits_list(app_state: &AppState) -> Vec<String> {
    shop_proxy::get_all_traits(app_state, &GameSystem::Pathfinder).await
}

pub fn get_shop_templates_data() -> Vec<ShopTemplateData> {
    PfShopTemplateEnum::iter()
        .map(std::convert::Into::into)
        .collect()
}

fn convert_result_to_shop_response(
    field_filters: &ItemFieldFilters,
    pagination: &ShopPaginatedRequest,
    result: anyhow::Result<(u32, Vec<ResponseItem>)>,
) -> ShopListingResponse {
    match result {
        Ok(res) => {
            let item: Vec<ResponseItem> = res.1;
            let n_of_items = item.len();
            ShopListingResponse {
                results: Some(item),
                count: n_of_items,
                next: if n_of_items
                    >= pagination.paginated_request.page_size.unsigned_abs() as usize
                {
                    Some(shop_next_url(field_filters, pagination, n_of_items as u32))
                } else {
                    None
                },
                total: res.0 as usize,
                game: GameSystem::Pathfinder,
            }
        }
        Err(_) => ShopListingResponse::default_with_system(GameSystem::Pathfinder),
    }
}
