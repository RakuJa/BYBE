use crate::db::shop_proxy;
use crate::models::response_data::ResponseItem;
use crate::models::routers_validator_structs::ItemFieldFilters;
use crate::models::shop_structs::{
    RandomShopData, ShopFilterQuery, ShopPaginatedRequest, ShopTemplateEnum,
};
use crate::services::url_calculator::shop_next_url_calculator;
use crate::AppState;
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

    let shop_type = shop_data.shop_template.clone().unwrap_or_default();
    let n_of_consumables: i64 = shop_data.consumable_dices.iter().map(|x| x.roll()).sum();
    let n_of_equipables: i64 = shop_data.equipment_dices.iter().map(|x| x.roll()).sum();
    let (n_of_equipment, n_of_weapons, n_of_armors, n_of_shields) = match shop_type {
        ShopTemplateEnum::Blacksmith => {
            // This will never panic if n_of_equipables >= 1 and dice sum should always be at least 1.
            // if 1<=n<2 => n/2 = 0..n
            // TLDR we know that it will never panic.

            let n_of_forged_items = fastrand::i64((n_of_equipables / 2)..=n_of_equipables);
            let forged_items_tuple = get_forged_items_tuple(n_of_forged_items);
            (
                n_of_equipables - n_of_forged_items,
                forged_items_tuple.0,
                forged_items_tuple.1,
                forged_items_tuple.2,
            )
        }
        ShopTemplateEnum::Alchemist => (n_of_equipables, 0, 0, 0),
        ShopTemplateEnum::General => {
            // This can panic if n_of_equipables is <=1,
            // n=1 => n/2 = 0, 0..0 panic!
            // we manually set it as 1 in that case

            let n_of_forged_items = fastrand::i64(
                0..=if n_of_equipables > 1 {
                    n_of_equipables / 2
                } else {
                    1
                },
            );
            let forged_items_tuple = get_forged_items_tuple(n_of_forged_items);
            (
                n_of_equipables - n_of_forged_items,
                forged_items_tuple.0,
                forged_items_tuple.1,
                forged_items_tuple.2,
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
            n_of_shields,
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

pub async fn get_sources_list(app_state: &AppState) -> Vec<String> {
    shop_proxy::get_all_sources(app_state).await
}

/// Gets the n of: weapons, armors, shields (in this order).
/// Changing order is considered a BREAKING CHANGE.
/// Calculating it randomly from the n of forged items.
fn get_forged_items_tuple(n_of_forged_items: i64) -> (i64, i64, i64) {
    // This can panic if n=0.
    // n<2 => 0..1, ok!
    // n<1 => 0..0, panic!
    // if that's the case we return 0 manually
    let n_of_weapons = if n_of_forged_items > 0 {
        fastrand::i64(n_of_forged_items / 2..=n_of_forged_items)
    } else {
        0
    };
    let n_of_armors = n_of_forged_items - n_of_weapons;
    // This can panic if we do not have enough armors (n<3).
    // n<3 => 1..1, panic!
    // n=3 => 1..2, ok!
    // if that's the case we return 0 manually
    // We take at least 1 shield if there are >3 armor
    // (shield will never be > armor,
    // with n>3 => (n/3)+1 is always < n
    let n_of_shields = if n_of_armors >= 3 {
        fastrand::i64(1..(n_of_armors / 3) + 1)
    } else {
        0
    };
    (n_of_weapons, n_of_armors - n_of_shields, n_of_shields)
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
