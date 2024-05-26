use crate::db::data_providers::{generic_fetcher, shop_fetcher};
use crate::models::creature::creature_metadata::type_enum::CreatureTypeEnum;
use crate::models::item::item_fields_enum::{FieldsUniqueValuesStruct, ItemField};
use crate::models::item::item_struct::Item;
use crate::models::routers_validator_structs::{ItemFieldFilters, PaginatedRequest};
use crate::models::shop_structs::ShopFilterQuery;
use crate::AppState;
use anyhow::Result;
use cached::proc_macro::once;
use strum::IntoEnumIterator;

pub async fn get_item_by_id(app_state: &AppState, id: i64) -> Option<Item> {
    shop_fetcher::fetch_item_by_id(&app_state.conn, id)
        .await
        .ok()
}

pub async fn get_filtered_items(
    app_state: &AppState,
    filters: &ShopFilterQuery,
) -> Result<Vec<Item>> {
    shop_fetcher::fetch_items_with_filters(&app_state.conn, filters).await
}

pub async fn get_paginated_items(
    app_state: &AppState,
    filters: &ItemFieldFilters,
    pagination: &PaginatedRequest,
) -> Result<(u32, Vec<Item>)> {
    let list = get_list(app_state).await;

    let filtered_list: Vec<Item> = list
        .into_iter()
        .filter(|x| Item::is_passing_filters(x, filters))
        .collect();

    let curr_slice: Vec<Item> = filtered_list
        .iter()
        .skip(pagination.cursor as usize)
        .take(pagination.page_size as usize)
        .cloned()
        .collect();

    Ok((curr_slice.len() as u32, curr_slice))
}

/// Gets all the items from the DB.
/// It will cache the result.
#[once(sync_writes = true, result = true)]
async fn get_all_items_from_db(app_state: &AppState) -> Result<Vec<Item>> {
    shop_fetcher::fetch_items(
        &app_state.conn,
        &PaginatedRequest {
            cursor: 0,
            page_size: -1,
        },
    )
    .await
}

/// Infallible method, it will expose a vector representing the values fetched from db or empty vec
async fn get_list(app_state: &AppState) -> Vec<Item> {
    get_all_items_from_db(app_state).await.unwrap_or(vec![])
}

pub async fn get_all_possible_values_of_filter(
    app_state: &AppState,
    field: ItemField,
) -> Vec<String> {
    let runtime_fields_values = get_all_keys(app_state).await;
    let mut x = match field {
        ItemField::Category => runtime_fields_values.list_of_categories,

        ItemField::Size => runtime_fields_values.list_of_sizes,
        ItemField::Rarity => runtime_fields_values.list_of_rarities,
        ItemField::Traits => runtime_fields_values.list_of_traits,
        ItemField::Sources => runtime_fields_values.list_of_sources,
        ItemField::Level => runtime_fields_values.list_of_levels,
        ItemField::ItemType => CreatureTypeEnum::iter().map(|x| x.to_string()).collect(),
        _ => vec![],
    };
    x.sort();
    x
}

/// Gets all the runtime keys (each table column unique values). It will cache the result
#[once(sync_writes = true)]
async fn get_all_keys(app_state: &AppState) -> FieldsUniqueValuesStruct {
    FieldsUniqueValuesStruct {
        list_of_levels: generic_fetcher::fetch_unique_values_of_field(
            &app_state.conn,
            "CREATURE_CORE",
            "level",
        )
        .await
        .unwrap_or_default(),
        list_of_categories: generic_fetcher::fetch_unique_values_of_field(
            &app_state.conn,
            "CREATURE_CORE",
            "family",
        )
        .await
        .unwrap(),
        list_of_traits: shop_fetcher::fetch_traits_associated_with_items(&app_state.conn)
            .await
            .unwrap_or_default(),
        list_of_sources: generic_fetcher::fetch_unique_values_of_field(
            &app_state.conn,
            "CREATURE_CORE",
            "source",
        )
        .await
        .unwrap_or_default(),
        list_of_sizes: generic_fetcher::fetch_unique_values_of_field(
            &app_state.conn,
            "CREATURE_CORE",
            "size",
        )
        .await
        .unwrap_or_default(),
        list_of_rarities: generic_fetcher::fetch_unique_values_of_field(
            &app_state.conn,
            "CREATURE_CORE",
            "rarity",
        )
        .await
        .unwrap_or_default(),
    }
}
