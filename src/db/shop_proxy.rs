use crate::db::data_providers::{generic_fetcher, shop_fetcher};
use crate::models::creature::creature_metadata::type_enum::CreatureTypeEnum;
use crate::models::item::armor_struct::Armor;
use crate::models::item::item_fields_enum::{FieldsUniqueValuesStruct, ItemField};
use crate::models::item::item_struct::Item;
use crate::models::item::shield_struct::Shield;
use crate::models::item::weapon_struct::Weapon;
use crate::models::response_data::ResponseItem;
use crate::models::routers_validator_structs::{ItemFieldFilters, OrderEnum};
use crate::models::shop_structs::{ItemSortEnum, ShopFilterQuery, ShopPaginatedRequest};
use crate::AppState;
use anyhow::Result;
use cached::proc_macro::once;
use strum::IntoEnumIterator;

pub async fn get_item_by_id(app_state: &AppState, id: i64) -> Option<ResponseItem> {
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
    pagination: &ShopPaginatedRequest,
) -> Result<(u32, Vec<ResponseItem>)> {
    let list = get_list(app_state).await;

    let mut filtered_list: Vec<ResponseItem> = list
        .into_iter()
        .filter(|x| Item::is_passing_filters(&x.core_item, filters))
        .collect();

    let total_item_count = filtered_list.len();

    filtered_list.sort_by(|a, b| {
        let cmp = match pagination
            .shop_sort_data
            .sort_by
            .clone()
            .unwrap_or_default()
        {
            ItemSortEnum::Id => a.core_item.id.cmp(&b.core_item.id),
            ItemSortEnum::Name => a.core_item.name.cmp(&b.core_item.name),
            ItemSortEnum::Level => a.core_item.level.cmp(&b.core_item.level),
            ItemSortEnum::Type => a.core_item.item_type.cmp(&b.core_item.item_type),
        };
        match pagination
            .shop_sort_data
            .order_by
            .clone()
            .unwrap_or_default()
        {
            OrderEnum::Ascending => cmp,
            OrderEnum::Descending => cmp.reverse(),
        }
    });

    let curr_slice: Vec<ResponseItem> = filtered_list
        .iter()
        .skip(pagination.paginated_request.cursor as usize)
        .take(pagination.paginated_request.page_size as usize)
        .cloned()
        .collect();

    Ok((total_item_count as u32, curr_slice))
}

/// Gets all the items from the DB.
async fn get_all_items_from_db(app_state: &AppState) -> Result<Vec<Item>> {
    shop_fetcher::fetch_items(&app_state.conn, 0, -1).await
}

/// Gets all the weapons from the DB.
async fn get_all_weapons_from_db(app_state: &AppState) -> Result<Vec<Weapon>> {
    shop_fetcher::fetch_weapons(&app_state.conn, 0, -1).await
}

/// Gets all the armors from the DB.
async fn get_all_armors_from_db(app_state: &AppState) -> Result<Vec<Armor>> {
    shop_fetcher::fetch_armors(&app_state.conn, 0, -1).await
}

/// Gets all the shields from the DB.
async fn get_all_shields_from_db(app_state: &AppState) -> Result<Vec<Shield>> {
    shop_fetcher::fetch_shields(&app_state.conn, 0, -1).await
}

/// Infallible method, it will expose a vector representing the values fetched from db or empty vec
#[once(sync_writes = true)]
async fn get_list(app_state: &AppState) -> Vec<ResponseItem> {
    let mut response_vec = Vec::new();
    for el in get_all_items_from_db(app_state).await.unwrap_or(vec![]) {
        response_vec.push(ResponseItem {
            core_item: el,
            weapon_data: None,
            armor_data: None,
            shield_data: None,
        })
    }
    for el in get_all_weapons_from_db(app_state).await.unwrap_or(vec![]) {
        response_vec.push(ResponseItem {
            core_item: el.item_core,
            weapon_data: Some(el.weapon_data),
            armor_data: None,
            shield_data: None,
        })
    }
    for el in get_all_armors_from_db(app_state).await.unwrap_or(vec![]) {
        response_vec.push(ResponseItem {
            core_item: el.item_core,
            weapon_data: None,
            armor_data: Some(el.armor_data),
            shield_data: None,
        })
    }
    for el in get_all_shields_from_db(app_state).await.unwrap() {
        response_vec.push(ResponseItem {
            core_item: el.item_core,
            weapon_data: None,
            armor_data: None,
            shield_data: Some(el.shield_data),
        })
    }
    response_vec
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
