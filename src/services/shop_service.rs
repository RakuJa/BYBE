use crate::db::shop_proxy;
use crate::models::item::item_fields_enum::ItemField;
use crate::models::response_data::ResponseItem;
use crate::models::routers_validator_structs::ItemFieldFilters;
use crate::models::shop_structs::{
    RandomShopData, ShopFilterQuery, ShopPaginatedRequest, ShopTypeEnum,
};
use crate::services::url_calculator::shop_next_url_calculator;
use crate::AppState;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ShopListingResponse {
    results: Option<Vec<ResponseItem>>,
    count: usize,
    total: usize,
    next: Option<String>,
}

pub async fn get_item(app_state: &AppState, id: i64) -> HashMap<String, Option<ResponseItem>> {
    hashmap! {
        String::from("results") =>
        shop_proxy::get_item_by_id(app_state, id).await
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
        shop_proxy::get_paginated_items(app_state, field_filter, pagination).await,
    )
}

pub async fn generate_random_shop_listing(
    app_state: &AppState,
    shop_data: &RandomShopData,
) -> ShopListingResponse {
    let min_level = shop_data.min_level.unwrap_or(0);
    let max_level = shop_data.max_level.unwrap_or(30);

    let shop_type = shop_data.shop_type.clone().unwrap_or_default();
    let n_of_consumables: i64 = shop_data.consumable_dices.iter().map(|x| x.roll()).sum();
    let n_of_equipables: i64 = shop_data.equipment_dices.iter().map(|x| x.roll()).sum();
    let (n_of_equipment, n_of_armors, n_of_weapons) = match shop_type {
        ShopTypeEnum::Blacksmith => {
            let n_of_forged_items = thread_rng().gen_range((n_of_equipables / 2)..=n_of_equipables);
            let n_of_weapons = thread_rng().gen_range(0..=n_of_forged_items);
            let n_of_armors = n_of_forged_items - n_of_weapons;
            (
                n_of_equipables - n_of_forged_items,
                n_of_weapons,
                n_of_armors,
            )
        }
        ShopTypeEnum::Alchemist => (n_of_equipables, 0, 0),
        ShopTypeEnum::General => {
            let n_of_forged_items = thread_rng().gen_range(0..=n_of_equipables / 2);
            let n_of_weapons = thread_rng().gen_range(0..=n_of_forged_items);
            let n_of_armors = n_of_forged_items - n_of_weapons;
            (
                n_of_equipables - n_of_forged_items,
                n_of_weapons,
                n_of_armors,
            )
        }
    };

    let pathfinder_version = shop_data.pathfinder_version.clone().unwrap_or_default();

    match shop_proxy::get_filtered_items(
        app_state,
        &ShopFilterQuery {
            min_level,
            max_level,
            n_of_equipment,
            n_of_consumables,
            n_of_weapons,
            n_of_armors,
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
                total: n_of_items,
            }
        }
        Err(_) => ShopListingResponse {
            results: None,
            count: 0,
            next: None,
            total: 0,
        },
    }
}

pub async fn get_traits_list(app_state: &AppState) -> Vec<String> {
    shop_proxy::get_all_possible_values_of_filter(app_state, ItemField::Traits).await
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
                next: if n_of_items >= pagination.paginated_request.page_size as usize {
                    Some(shop_next_url_calculator(
                        field_filters,
                        pagination,
                        n_of_items as u32,
                    ))
                } else {
                    None
                },
                total: res.0 as usize,
            }
        }
        Err(_) => ShopListingResponse {
            results: None,
            count: 0,
            next: None,
            total: 0,
        },
    }
}
