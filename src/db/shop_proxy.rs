use crate::AppState;
use crate::db::data_providers::shop_fetcher;
use crate::models::item::armor_struct::Armor;
use crate::models::item::item_struct::Item;
use crate::models::item::shield_struct::Shield;
use crate::models::item::weapon_struct::Weapon;
use crate::models::response_data::ResponseItem;
use crate::models::routers_validator_structs::{ItemFieldFilters, OrderEnum};
use crate::models::shared::game_system_enum::GameSystem;
use crate::models::shop_structs::{ItemSortEnum, ShopFilterQuery, ShopPaginatedRequest};
use anyhow::Result;
use cached::proc_macro::cached;
use itertools::Itertools;

pub async fn get_item_by_id(
    app_state: &AppState,
    gs: &GameSystem,
    id: i64,
) -> Option<ResponseItem> {
    shop_fetcher::fetch_item_by_id(&app_state.conn, gs, id)
        .await
        .ok()
}

pub async fn get_filtered_items(
    app_state: &AppState,
    gs: &GameSystem,
    filters: &ShopFilterQuery,
) -> Result<Vec<Item>> {
    shop_fetcher::fetch_items_with_filters(&app_state.conn, gs, filters).await
}

pub async fn get_paginated_items(
    app_state: &AppState,
    gs: &GameSystem,
    filters: &ItemFieldFilters,
    pagination: &ShopPaginatedRequest,
) -> Result<(u32, Vec<ResponseItem>)> {
    let list = get_list(app_state, gs).await;

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
            ItemSortEnum::Trait => a.core_item.traits.cmp(&b.core_item.traits),
            ItemSortEnum::Type => a.core_item.item_type.cmp(&b.core_item.item_type),
            ItemSortEnum::Rarity => a.core_item.rarity.cmp(&b.core_item.rarity),
            ItemSortEnum::Source => a.core_item.source.cmp(&b.core_item.source),
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
        .take(if pagination.paginated_request.page_size >= 0 {
            pagination.paginated_request.page_size.unsigned_abs() as usize
        } else {
            usize::MAX
        })
        .cloned()
        .collect();

    Ok((total_item_count as u32, curr_slice))
}

/// Gets all the items from the DB.
async fn get_all_items_from_db(app_state: &AppState, gs: &GameSystem) -> Result<Vec<Item>> {
    shop_fetcher::fetch_items(&app_state.conn, gs, 0, -1).await
}

/// Gets all the weapons from the DB.
async fn get_all_weapons_from_db(app_state: &AppState, gs: &GameSystem) -> Result<Vec<Weapon>> {
    shop_fetcher::fetch_weapons(&app_state.conn, gs, 0, -1).await
}

/// Gets all the armors from the DB.
async fn get_all_armors_from_db(app_state: &AppState, gs: &GameSystem) -> Result<Vec<Armor>> {
    shop_fetcher::fetch_armors(&app_state.conn, gs, 0, -1).await
}

/// Gets all the shields from the DB.
async fn get_all_shields_from_db(app_state: &AppState, gs: &GameSystem) -> Result<Vec<Shield>> {
    shop_fetcher::fetch_shields(&app_state.conn, gs, 0, -1).await
}

/// Infallible method, it will expose a vector representing the values fetched from db or empty vec
#[cached(key = "i64", convert = r##"{ gs.into() }"##)]
async fn get_list(app_state: &AppState, gs: &GameSystem) -> Vec<ResponseItem> {
    let mut response_vec = Vec::new();
    for el in get_all_items_from_db(app_state, gs).await.unwrap_or(vec![]) {
        response_vec.push(ResponseItem {
            core_item: el,
            weapon_data: None,
            armor_data: None,
            shield_data: None,
            game_system: *gs,
        });
    }
    for el in get_all_weapons_from_db(app_state, gs)
        .await
        .unwrap_or(vec![])
    {
        response_vec.push(ResponseItem {
            core_item: el.item_core,
            weapon_data: Some(el.weapon_data),
            armor_data: None,
            shield_data: None,
            game_system: *gs,
        });
    }
    for el in get_all_armors_from_db(app_state, gs)
        .await
        .unwrap_or(vec![])
    {
        response_vec.push(ResponseItem {
            core_item: el.item_core,
            weapon_data: None,
            armor_data: Some(el.armor_data),
            shield_data: None,
            game_system: *gs,
        });
    }
    for el in get_all_shields_from_db(app_state, gs).await.unwrap() {
        response_vec.push(ResponseItem {
            core_item: el.item_core,
            weapon_data: None,
            armor_data: None,
            shield_data: Some(el.shield_data),
            game_system: *gs,
        });
    }
    response_vec
}

/// Gets all the runtime sources. It will cache the result
#[cached(key = "i64", convert = r##"{ gs.into() }"##)]
pub async fn get_all_sources(app_state: &AppState, gs: &GameSystem) -> Vec<String> {
    get_all_items_from_db(app_state, gs).await.map_or_else(
        |_| vec![],
        |v| {
            v.into_iter()
                .map(|x| x.source)
                .unique()
                .filter(|x| !x.is_empty())
                .sorted()
                .collect()
        },
    )
}

/// Gets all the runtime traits. It will cache the result
#[cached(key = "i64", convert = r##"{ gs.into() }"##)]
pub async fn get_all_traits(app_state: &AppState, gs: &GameSystem) -> Vec<String> {
    match (
        get_all_items_from_db(app_state, gs).await,
        get_all_weapons_from_db(app_state, gs).await,
        get_all_armors_from_db(app_state, gs).await,
        get_all_shields_from_db(app_state, gs).await,
    ) {
        (Ok(items), Ok(wps), Ok(armors), Ok(shields)) => items
            .into_iter()
            .chain(wps.into_iter().map(|x| x.item_core))
            .chain(armors.into_iter().map(|x| x.item_core))
            .chain(shields.into_iter().map(|x| x.item_core))
            .flat_map(|x| x.traits)
            .unique()
            .filter(|x| !x.is_empty())
            .sorted()
            .collect(),
        _ => {
            vec![]
        }
    }
}
