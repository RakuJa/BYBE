use crate::db::data_providers::generic_fetcher::{
    enrich_with_traits, fetch_armor_runes, fetch_col_range, fetch_col_range_f64, fetch_item_traits,
    fetch_weapon_damage_data, fetch_weapon_runes,
};
use crate::db::data_providers::raw_query_builder::{
    format_pagination_clause, prepare_count_items_listing, prepare_filtered_get_items,
    prepare_paginated_get_items_listing,
};
use crate::models::item::armor_struct::{Armor, ArmorData};
use crate::models::item::item_field_filter::ItemFieldFilters;
use crate::models::item::item_metadata::type_enum::ItemTypeEnum;
use crate::models::item::item_struct::Item;
use crate::models::item::shield_struct::{Shield, ShieldData};
use crate::models::item::shop_structs::{ItemSortEnum, ShopFilterQuery, ShopRanges};
use crate::models::item::weapon_struct::{Weapon, WeaponData};
use crate::models::response_data::ResponseItem;
use crate::models::routers_validator_structs::OrderEnum;
use crate::models::shared::game_system_enum::GameSystem;
use anyhow::Result;
use futures::future::try_join_all;
use nanorand::{Rng, WyRand};
use sqlx::{PgPool, query_as};
use tracing::debug;

pub async fn fetch_item_by_id(pool: &PgPool, gs: GameSystem, item_id: i64) -> Result<ResponseItem> {
    let mut item: Item = sqlx::query_as(sqlx::AssertSqlSafe(format!(
        "SELECT * FROM {gs}_item_table WHERE status = 'valid' AND id = $1 ORDER BY name LIMIT 1"
    )))
    .bind(item_id)
    .fetch_one(pool)
    .await?;
    item.traits = fetch_item_traits(pool, gs, item_id).await?;
    Ok(match item.item_type {
        ItemTypeEnum::Consumable | ItemTypeEnum::Equipment => ResponseItem::from((item, gs)),
        ItemTypeEnum::Weapon => ResponseItem {
            core_item: item,
            weapon_data: fetch_weapon_data_by_item_id(pool, gs, item_id).await.ok(),
            armor_data: None,
            shield_data: None,
            game: gs,
        },
        ItemTypeEnum::Armor => ResponseItem {
            core_item: item,
            weapon_data: None,
            armor_data: fetch_armor_data_by_item_id(pool, gs, item_id).await.ok(),
            shield_data: None,
            game: gs,
        },
        ItemTypeEnum::Shield => ResponseItem {
            core_item: item,
            weapon_data: None,
            armor_data: None,
            shield_data: fetch_shield_data_by_item_id(pool, gs, item_id).await.ok(),
            game: gs,
        },
    })
}

async fn fetch_weapon_by_item_id(pool: &PgPool, gs: GameSystem, item_id: i64) -> Result<Weapon> {
    let mut weapon: Weapon = sqlx::query_as(sqlx::AssertSqlSafe(format!(
        "
        SELECT wt.id AS weapon_id, wt.to_hit_bonus,
        wt.splash_dmg, wt.n_of_potency_runes,
        wt.n_of_striking_runes, wt.range, wt.reload, wt.weapon_type, wt.base_item_id,
        it.*
        FROM {gs}_weapon_table wt
        LEFT JOIN {gs}_item_table it ON wt.base_item_id = it.id
        WHERE wt.base_item_id = ($1) AND it.status = 'valid'
        "
    )))
    .bind(item_id)
    .fetch_one(pool)
    .await?;
    weapon.item_core.traits = fetch_item_traits(pool, gs, item_id).await?;
    weapon.weapon_data.property_runes = fetch_weapon_runes(pool, gs, weapon.weapon_data.id).await?;
    weapon.weapon_data.damage_data =
        fetch_weapon_damage_data(pool, gs, weapon.weapon_data.id).await?;
    Ok(weapon)
}

async fn fetch_armor_by_item_id(pool: &PgPool, gs: GameSystem, item_id: i64) -> Result<Armor> {
    let mut armor: Armor = sqlx::query_as(sqlx::AssertSqlSafe(format!(
        "
        SELECT at.id AS armor_id, at.bonus_ac, at.check_penalty, at.dex_cap, at.n_of_potency_runes,
        at.n_of_resilient_runes, at.speed_penalty, at.strength_required, at.base_item_id,
        it.*
        FROM {gs}_armor_table at
        LEFT JOIN {gs}_item_table it ON at.base_item_id = it.id
        WHERE at.base_item_id = ($1) AND it.status = 'valid'
        "
    )))
    .bind(item_id)
    .fetch_one(pool)
    .await?;
    armor.item_core.traits = fetch_item_traits(pool, gs, item_id).await?;
    armor.armor_data.property_runes = fetch_armor_runes(pool, gs, armor.armor_data.id).await?;
    Ok(armor)
}

async fn fetch_shield_by_item_id(pool: &PgPool, gs: GameSystem, item_id: i64) -> Result<Shield> {
    let mut shield: Shield = sqlx::query_as(sqlx::AssertSqlSafe(format!(
        "
        SELECT st.id AS shield_id, st.bonus_ac, st.n_of_reinforcing_runes, st.speed_penalty,
        it.*
        FROM {gs}_shield_table st
        LEFT JOIN {gs}_item_table it ON st.base_item_id = it.id
        WHERE st.base_item_id = ($1) AND it.status = 'valid'
        "
    )))
    .bind(item_id)
    .fetch_one(pool)
    .await?;
    shield.item_core.traits = fetch_item_traits(pool, gs, item_id).await?;
    Ok(shield)
}

async fn fetch_weapon_data_by_item_id(
    pool: &PgPool,
    gs: GameSystem,
    item_id: i64,
) -> Result<WeaponData> {
    Ok(fetch_weapon_by_item_id(pool, gs, item_id)
        .await?
        .weapon_data)
}

async fn fetch_armor_data_by_item_id(
    pool: &PgPool,
    gs: GameSystem,
    item_id: i64,
) -> Result<ArmorData> {
    Ok(fetch_armor_by_item_id(pool, gs, item_id).await?.armor_data)
}

async fn fetch_shield_data_by_item_id(
    pool: &PgPool,
    gs: GameSystem,
    item_id: i64,
) -> Result<ShieldData> {
    Ok(fetch_shield_by_item_id(pool, gs, item_id)
        .await?
        .shield_data)
}

pub async fn fetch_items(
    pool: &PgPool,
    gs: GameSystem,
    cursor: i64,
    page_size: i16,
) -> Result<Vec<Item>> {
    let pagination = format_pagination_clause(cursor, page_size);
    let items: Vec<Item> = sqlx::query_as(sqlx::AssertSqlSafe(format!(
        "
        SELECT * FROM {gs}_item_table
        WHERE is_derived = False AND status = 'valid'
            AND UPPER(item_type) = 'EQUIPMENT' OR UPPER(item_type) = 'CONSUMABLE'
        GROUP BY id
        ORDER BY name {pagination}"
    )))
    .fetch_all(pool)
    .await?;
    Ok(update_items_with_traits(pool, gs, items).await)
}

pub async fn fetch_weapons(
    pool: &PgPool,
    gs: GameSystem,
    cursor: i64,
    page_size: i16,
) -> Result<Vec<Weapon>> {
    let pagination = format_pagination_clause(cursor, page_size);
    let weapons: Vec<Weapon> = sqlx::query_as(sqlx::AssertSqlSafe(format!(
        "
        SELECT wt.id AS weapon_id, wt.to_hit_bonus, wt.splash_dmg, wt.n_of_potency_runes,
            wt.n_of_striking_runes, wt.range, wt.reload, wt.weapon_type, wt.base_item_id,
            it.*
        FROM {gs}_weapon_table wt
        LEFT JOIN {gs}_item_table it ON wt.base_item_id = it.id
        WHERE it.is_derived = False AND it.status = 'valid'
        GROUP BY it.id
        ORDER BY name {pagination}
    "
    )))
    .fetch_all(pool)
    .await?;
    try_join_all(weapons.into_iter().map(|mut el| {
        let pool = pool.clone();
        async move {
            el.item_core.traits = fetch_item_traits(&pool, gs, el.item_core.id).await?;
            el.weapon_data.property_runes =
                fetch_weapon_runes(&pool, gs, el.weapon_data.id).await?;
            el.weapon_data.damage_data =
                fetch_weapon_damage_data(&pool, gs, el.weapon_data.id).await?;
            Ok(el)
        }
    }))
    .await
}

pub async fn fetch_armors(
    pool: &PgPool,
    gs: GameSystem,
    cursor: i64,
    page_size: i16,
) -> Result<Vec<Armor>> {
    let pagination = format_pagination_clause(cursor, page_size);
    let armors: Vec<Armor> = sqlx::query_as(sqlx::AssertSqlSafe(format!(
        "
        SELECT at.id AS armor_id, at.bonus_ac, at.check_penalty, at.dex_cap, at.n_of_potency_runes,
            at.n_of_resilient_runes, at.speed_penalty, at.strength_required, at.base_item_id, it.*
        FROM {gs}_armor_table at
        LEFT JOIN {gs}_item_table it ON at.base_item_id = it.id
        WHERE it.is_derived = False AND it.status = 'valid'
        GROUP BY it.id
        ORDER BY name {pagination}
    "
    )))
    .fetch_all(pool)
    .await?;
    try_join_all(armors.into_iter().map(|mut el| {
        let pool = pool.clone();
        async move {
            el.item_core.traits = fetch_item_traits(&pool, gs, el.item_core.id).await?;
            el.armor_data.property_runes = fetch_armor_runes(&pool, gs, el.armor_data.id).await?;
            Ok(el)
        }
    }))
    .await
}

pub async fn fetch_shields(
    pool: &PgPool,
    gs: GameSystem,
    cursor: i64,
    page_size: i16,
) -> Result<Vec<Shield>> {
    let pagination = format_pagination_clause(cursor, page_size);
    let shields: Vec<Shield> = sqlx::query_as(sqlx::AssertSqlSafe(format!(
        "
        SELECT st.id AS shield_id, st.bonus_ac, st.n_of_reinforcing_runes, st.speed_penalty, it.*
        FROM {gs}_shield_table st
        LEFT JOIN {gs}_item_table it ON st.base_item_id = it.id
        WHERE it.is_derived = False AND it.status = 'valid'
        GROUP BY it.id
        ORDER BY name {pagination}
    "
    )))
    .fetch_all(pool)
    .await?;
    try_join_all(shields.into_iter().map(|mut el| {
        let pool = pool.clone();
        async move {
            el.item_core.traits = fetch_item_traits(&pool, gs, el.item_core.id).await?;
            Ok(el)
        }
    }))
    .await
}

async fn update_items_with_traits(pool: &PgPool, gs: GameSystem, items: Vec<Item>) -> Vec<Item> {
    enrich_with_traits(pool, gs, items, false).await
}

pub async fn fetch_items_with_filters(
    pool: &PgPool,
    gs: GameSystem,
    filters: &ShopFilterQuery,
) -> Result<Vec<Item>> {
    let items: Vec<Item> = query_as(sqlx::AssertSqlSafe(prepare_filtered_get_items(gs, filters)))
        .fetch_all(pool)
        .await?;
    let equipment: Vec<&Item> = items
        .iter()
        .filter(|x| x.item_type == ItemTypeEnum::Equipment)
        .collect();
    let weapons: Vec<&Item> = items
        .iter()
        .filter(|x| x.item_type == ItemTypeEnum::Weapon)
        .collect();
    let armors: Vec<&Item> = items
        .iter()
        .filter(|x| x.item_type == ItemTypeEnum::Armor)
        .collect();
    let shields: Vec<&Item> = items
        .iter()
        .filter(|x| x.item_type == ItemTypeEnum::Shield)
        .collect();
    let consumables: Vec<&Item> = items
        .iter()
        .filter(|x| x.item_type == ItemTypeEnum::Consumable)
        .collect();

    let n_of_items_to_return = filters.n_of_equipment
        + filters.n_of_shields
        + filters.n_of_weapons
        + filters.n_of_armors
        + filters.n_of_consumables;
    Ok(
        if i64::try_from(items.len()).unwrap_or(i64::MAX) >= n_of_items_to_return {
            debug!("Result vector is the correct size, no more operations needed");
            items
        } else {
            debug!("Result vector is not the correct size, duplicating random elements..");

            let mut item_vec = fill_item_vec_to_len(&equipment, filters.n_of_equipment);
            item_vec.extend(fill_item_vec_to_len(&consumables, filters.n_of_consumables));
            item_vec.extend(fill_item_vec_to_len(&weapons, filters.n_of_weapons));
            item_vec.extend(fill_item_vec_to_len(&armors, filters.n_of_armors));
            item_vec.extend(fill_item_vec_to_len(&shields, filters.n_of_shields));

            item_vec
        },
    )
}

fn fill_item_vec_to_len(item_vec: &[&Item], desired_len: i64) -> Vec<Item> {
    let mut og_vec: Vec<Item> = item_vec.iter().map(|x| (*x).clone()).collect();
    for _ in 0..(i64::try_from(item_vec.len()).unwrap_or(i64::MAX) - desired_len) {
        if let Some(x) = item_vec.get(WyRand::new().generate_range(0..item_vec.len())) {
            og_vec.push((*x).clone());
        }
    }
    og_vec
}

pub async fn fetch_paginated_items(
    pool: &PgPool,
    gs: GameSystem,
    filters: &ItemFieldFilters,
    sort_by: ItemSortEnum,
    order_by: OrderEnum,
    cursor: u32,
    page_size: i16,
) -> Result<Vec<ResponseItem>> {
    let query =
        prepare_paginated_get_items_listing(gs, filters, sort_by, order_by, cursor, page_size);
    let items: Vec<Item> = sqlx::query_as(sqlx::AssertSqlSafe(query))
        .fetch_all(pool)
        .await?;
    let mut result = Vec::with_capacity(items.len());
    for mut item in items {
        let item_id = item.id;
        item.traits = fetch_item_traits(pool, gs, item_id)
            .await
            .unwrap_or_default();
        let response_item = match item.item_type {
            ItemTypeEnum::Weapon => ResponseItem {
                core_item: item,
                weapon_data: fetch_weapon_data_by_item_id(pool, gs, item_id).await.ok(),
                armor_data: None,
                shield_data: None,
                game: gs,
            },
            ItemTypeEnum::Armor => ResponseItem {
                core_item: item,
                weapon_data: None,
                armor_data: fetch_armor_data_by_item_id(pool, gs, item_id).await.ok(),
                shield_data: None,
                game: gs,
            },
            ItemTypeEnum::Shield => ResponseItem {
                core_item: item,
                weapon_data: None,
                armor_data: None,
                shield_data: fetch_shield_data_by_item_id(pool, gs, item_id).await.ok(),
                game: gs,
            },
            _ => ResponseItem {
                core_item: item,
                weapon_data: None,
                armor_data: None,
                shield_data: None,
                game: gs,
            },
        };
        result.push(response_item);
    }
    Ok(result)
}

pub async fn fetch_items_listing_count(
    pool: &PgPool,
    gs: GameSystem,
    filters: &ItemFieldFilters,
) -> Result<i64> {
    let query = prepare_count_items_listing(gs, filters);
    Ok(sqlx::query_scalar(sqlx::AssertSqlSafe(query))
        .fetch_one(pool)
        .await?)
}

pub async fn fetch_shop_ranges(pool: &PgPool, gs: GameSystem) -> Result<ShopRanges> {
    let from = format!("FROM {gs}_item_table WHERE is_derived = false AND status = 'valid'");
    let (min_hp, max_hp) = fetch_col_range(pool, "hp", &from).await?;
    let (min_level, max_level) = fetch_col_range(pool, "level", &from).await?;
    let (min_price, max_price) = fetch_col_range(pool, "price", &from).await?;
    let (min_bulk, max_bulk) = fetch_col_range_f64(pool, "bulk", &from).await?;
    let (min_number_of_uses, max_number_of_uses) =
        fetch_col_range(pool, "number_of_uses", &from).await?;
    Ok(ShopRanges {
        min_bulk,
        max_bulk,
        min_quantity: 1,
        max_quantity: 1,
        min_hp,
        max_hp,
        min_level,
        max_level,
        min_price,
        max_price,
        min_number_of_uses,
        max_number_of_uses,
    })
}

pub async fn fetch_shop_all_sources(pool: &PgPool, gs: GameSystem) -> Result<Vec<String>> {
    Ok(sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
        "SELECT DISTINCT source FROM {gs}_item_table
         WHERE is_derived = false AND status = 'valid' AND source != ''
         ORDER BY source"
    )))
    .fetch_all(pool)
    .await?)
}

pub async fn fetch_shop_all_traits(pool: &PgPool, gs: GameSystem) -> Result<Vec<String>> {
    Ok(sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
        "SELECT tt.name
         FROM {gs}_trait_item_association_table tcat
         JOIN {gs}_trait_table tt ON tcat.trait_id = tt.name
         JOIN {gs}_item_table it ON tcat.item_id = it.id
         WHERE it.is_derived = false AND it.status = 'valid' AND tt.name != ''
         GROUP BY tt.name
         ORDER BY tt.name"
    )))
    .fetch_all(pool)
    .await?)
}
