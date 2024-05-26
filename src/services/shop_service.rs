use crate::db::shop_proxy;
use crate::models::item::item_fields_enum::ItemField;
use crate::models::item::item_struct::Item;
use crate::models::response_data::ResponseItem;
use crate::models::routers_validator_structs::{ItemFieldFilters, PaginatedRequest};
use crate::models::shop_structs::{RandomShopData, ShopFilterQuery};
use crate::services::url_calculator::shop_next_url_calculator;
use crate::AppState;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ShopListingResponse {
    results: Option<Vec<ResponseItem>>,
    count: usize,
    next: Option<String>,
}

pub async fn get_item(app_state: &AppState, id: i64) -> HashMap<String, Option<ResponseItem>> {
    hashmap! {
        String::from("results") =>
        shop_proxy::get_item_by_id(app_state, id).await.map(ResponseItem::from)
    }
}

pub async fn get_shop_listing(
    app_state: &AppState,
    field_filter: &ItemFieldFilters,
    pagination: &PaginatedRequest,
) -> ShopListingResponse {
    convert_result_to_shop_response(
        field_filter,
        pagination,
        shop_proxy::get_paginated_items(app_state, field_filter, pagination).await,
    )
}

pub async fn generate_random_shop_listing(
    app_state: &AppState,
    shop_data: &RandomShopData,
) -> ShopListingResponse {
    let min_level = shop_data.min_level.unwrap_or(0);
    let max_level = shop_data.max_level.unwrap_or(30);

    let _shop_type = shop_data.shop_type.clone().unwrap_or_default();
    let n_of_consumables: i64 = shop_data.consumable_dices.iter().map(|x| x.roll()).sum();
    let n_of_equipment: i64 = shop_data.equipment_dices.iter().map(|x| x.roll()).sum();

    let pathfinder_version = shop_data.pathfinder_version.clone().unwrap_or_default();

    match shop_proxy::get_filtered_items(
        app_state,
        &ShopFilterQuery {
            //shop_type,
            min_level,
            max_level,
            n_of_equipment,
            n_of_consumables,
            pathfinder_version,
        },
    )
    .await
    {
        Ok(result) => {
            let n_of_items = result.len();
            ShopListingResponse {
                results: Some(result.into_iter().map(ResponseItem::from).collect()),
                count: n_of_items,
                next: None,
            }
        }
        Err(_) => ShopListingResponse {
            results: None,
            count: 0,
            next: None,
        },
    }
}

pub async fn get_traits_list(app_state: &AppState) -> Vec<String> {
    shop_proxy::get_all_possible_values_of_filter(app_state, ItemField::Traits).await
}

fn convert_result_to_shop_response(
    field_filters: &ItemFieldFilters,
    pagination: &PaginatedRequest,
    result: anyhow::Result<(u32, Vec<Item>)>,
) -> ShopListingResponse {
    match result {
        Ok(res) => {
            let item: Vec<Item> = res.1;
            let n_of_items = item.len();
            ShopListingResponse {
                results: Some(item.into_iter().map(ResponseItem::from).collect()),
                count: n_of_items,
                next: if n_of_items >= pagination.page_size as usize {
                    Some(shop_next_url_calculator(field_filters, pagination, res.0))
                } else {
                    None
                },
            }
        }
        Err(_) => ShopListingResponse {
            results: None,
            count: 0,
            next: None,
        },
    }
}