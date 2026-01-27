use crate::db::data_providers::generic_fetcher::{
    fetch_armor_runes, fetch_item_traits, fetch_weapon_damage_data, fetch_weapon_runes,
};
use crate::db::data_providers::raw_query_builder::prepare_filtered_get_items;
use crate::models::item::armor_struct::{Armor, ArmorData};
use crate::models::item::item_metadata::type_enum::ItemTypeEnum;
use crate::models::item::item_struct::Item;
use crate::models::item::shield_struct::{Shield, ShieldData};
use crate::models::item::weapon_struct::{Weapon, WeaponData};
use crate::models::response_data::ResponseItem;
use crate::models::shared::game_system_enum::GameSystem;
use crate::models::shop_structs::ShopFilterQuery;
use anyhow::Result;
use log::debug;
use nanorand::{Rng, WyRand};
use sqlx::{Pool, Sqlite, query_as};

pub async fn fetch_item_by_id(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    item_id: i64,
) -> Result<ResponseItem> {
    let mut item: Item = sqlx::query_as(sqlx::AssertSqlSafe(format!(
        "SELECT * FROM {gs}_item_table WHERE id = ? ORDER BY name LIMIT 1"
    )))
    .bind(item_id)
    .fetch_one(conn)
    .await?;
    item.traits = fetch_item_traits(conn, gs, item_id).await?;
    Ok(match item.item_type {
        ItemTypeEnum::Consumable | ItemTypeEnum::Equipment => ResponseItem::from((item, *gs)),
        ItemTypeEnum::Weapon => ResponseItem {
            core_item: item,
            weapon_data: fetch_weapon_data_by_item_id(conn, gs, item_id).await.ok(),
            armor_data: None,
            shield_data: None,
            game: *gs,
        },
        ItemTypeEnum::Armor => ResponseItem {
            core_item: item,
            weapon_data: None,
            armor_data: fetch_armor_data_by_item_id(conn, gs, item_id).await.ok(),
            shield_data: None,
            game: *gs,
        },
        ItemTypeEnum::Shield => ResponseItem {
            core_item: item,
            weapon_data: None,
            armor_data: None,
            shield_data: fetch_shield_data_by_item_id(conn, gs, item_id).await.ok(),
            game: *gs,
        },
    })
}

async fn fetch_weapon_by_item_id(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    item_id: i64,
) -> Result<Weapon> {
    let mut weapon: Weapon = sqlx::query_as(sqlx::AssertSqlSafe(format!(
        "
        SELECT wt.id AS weapon_id, wt.to_hit_bonus,
        wt.splash_dmg, wt.n_of_potency_runes,
        wt.n_of_striking_runes, wt.range, wt.reload, wt.weapon_type, wt.base_item_id,
        it.*
        FROM {gs}_weapon_table wt
        LEFT JOIN {gs}_item_table it ON wt.base_item_id = it.id
        WHERE wt.base_item_id = ($1)
        "
    )))
    .bind(item_id)
    .fetch_one(conn)
    .await?;
    weapon.item_core.traits = fetch_item_traits(conn, gs, item_id).await?;
    weapon.weapon_data.property_runes = fetch_weapon_runes(conn, gs, weapon.weapon_data.id).await?;
    weapon.weapon_data.damage_data =
        fetch_weapon_damage_data(conn, gs, weapon.weapon_data.id).await?;
    Ok(weapon)
}

async fn fetch_armor_by_item_id(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    item_id: i64,
) -> Result<Armor> {
    let mut armor: Armor = sqlx::query_as(sqlx::AssertSqlSafe(format!(
        "
        SELECT at.id AS armor_id, at.bonus_ac, at.check_penalty, at.dex_cap, at.n_of_potency_runes,
        at.n_of_resilient_runes, at.speed_penalty, at.strength_required, at.base_item_id,
        it.*
        FROM {gs}_armor_table at
        LEFT JOIN {gs}_item_table it ON at.base_item_id = it.id
        WHERE at.base_item_id = ($1)
        "
    )))
    .bind(item_id)
    .fetch_one(conn)
    .await?;
    armor.item_core.traits = fetch_item_traits(conn, gs, item_id).await?;
    armor.armor_data.property_runes = fetch_armor_runes(conn, gs, armor.armor_data.id).await?;
    Ok(armor)
}

async fn fetch_shield_by_item_id(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    item_id: i64,
) -> Result<Shield> {
    let mut shield: Shield = sqlx::query_as(sqlx::AssertSqlSafe(format!(
        "
        SELECT st.id AS shield_id, st.bonus_ac, st.n_of_reinforcing_runes, st.speed_penalty,
        it.*
        FROM {gs}_shield_table st
        LEFT JOIN {gs}_item_table it ON st.base_item_id = it.id
        WHERE st.base_item_id = ($1)
        "
    )))
    .bind(item_id)
    .fetch_one(conn)
    .await?;
    shield.item_core.traits = fetch_item_traits(conn, gs, item_id).await?;
    Ok(shield)
}

async fn fetch_weapon_data_by_item_id(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    item_id: i64,
) -> Result<WeaponData> {
    Ok(fetch_weapon_by_item_id(conn, gs, item_id)
        .await?
        .weapon_data)
}

async fn fetch_armor_data_by_item_id(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    item_id: i64,
) -> Result<ArmorData> {
    Ok(fetch_armor_by_item_id(conn, gs, item_id).await?.armor_data)
}

async fn fetch_shield_data_by_item_id(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    item_id: i64,
) -> Result<ShieldData> {
    Ok(fetch_shield_by_item_id(conn, gs, item_id)
        .await?
        .shield_data)
}

pub async fn fetch_items(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    cursor: u32,
    page_size: i16,
) -> Result<Vec<Item>> {
    let items: Vec<Item> = sqlx::query_as(sqlx::AssertSqlSafe(format!(
        "
        SELECT * FROM {gs}_item_table
        WHERE is_derived = False
            AND UPPER(item_type) == 'EQUIPMENT' OR UPPER(item_type) == 'CONSUMABLE'
        GROUP BY id
        ORDER BY name LIMIT ?,?"
    )))
    .bind(cursor)
    .bind(page_size)
    .fetch_all(conn)
    .await?;
    Ok(update_items_with_traits(conn, gs, items).await)
}

pub async fn fetch_weapons(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    cursor: u32,
    page_size: i16,
) -> Result<Vec<Weapon>> {
    let x: Vec<Weapon> = sqlx::query_as(sqlx::AssertSqlSafe(format!(
        "
        SELECT wt.id AS weapon_id, wt.to_hit_bonus, wt.splash_dmg, wt.n_of_potency_runes,
            wt.n_of_striking_runes, wt.range, wt.reload, wt.weapon_type, wt.base_item_id,
            it.*
        FROM {gs}_weapon_table wt
        LEFT JOIN {gs}_item_table it ON wt.base_item_id = it.id
        WHERE it.is_derived = False
        GROUP BY it.id
        ORDER BY name LIMIT ?,?
    "
    )))
    .bind(cursor)
    .bind(page_size)
    .fetch_all(conn)
    .await?;
    let mut result_vec = Vec::new();
    for mut el in x {
        el.item_core.traits = fetch_item_traits(conn, gs, el.item_core.id).await?;
        el.weapon_data.property_runes = fetch_weapon_runes(conn, gs, el.weapon_data.id).await?;
        el.weapon_data.damage_data = fetch_weapon_damage_data(conn, gs, el.weapon_data.id).await?;
        result_vec.push(el);
    }
    Ok(result_vec)
}

pub async fn fetch_armors(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    cursor: u32,
    page_size: i16,
) -> Result<Vec<Armor>> {
    let x: Vec<Armor> = sqlx::query_as(sqlx::AssertSqlSafe(format!(
        "
        SELECT at.id AS armor_id, at.bonus_ac, at.check_penalty, at.dex_cap, at.n_of_potency_runes,
            at.n_of_resilient_runes, at.speed_penalty, at.strength_required, at.base_item_id, it.*
        FROM {gs}_armor_table at
        LEFT JOIN {gs}_item_table it ON at.base_item_id = it.id
        WHERE it.is_derived = False
        GROUP BY it.id
        ORDER BY name LIMIT ?,?
    "
    )))
    .bind(cursor)
    .bind(page_size)
    .fetch_all(conn)
    .await?;
    let mut result_vec = Vec::new();
    for mut el in x {
        el.item_core.traits = fetch_item_traits(conn, gs, el.item_core.id).await?;
        el.armor_data.property_runes = fetch_armor_runes(conn, gs, el.armor_data.id).await?;
        result_vec.push(el);
    }
    Ok(result_vec)
}

pub async fn fetch_shields(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    cursor: u32,
    page_size: i16,
) -> Result<Vec<Shield>> {
    let x: Vec<Shield> = sqlx::query_as(sqlx::AssertSqlSafe(format!(
        "
        SELECT st.id AS shield_id, st.bonus_ac, st.n_of_reinforcing_runes, st.speed_penalty, it.*
        FROM {gs}_shield_table st
        LEFT JOIN {gs}_item_table it ON st.base_item_id = it.id
        WHERE it.is_derived = False
        GROUP BY it.id
        ORDER BY name LIMIT ?,?
    "
    )))
    .bind(cursor)
    .bind(page_size)
    .fetch_all(conn)
    .await?;
    let mut result_vec = Vec::new();
    for mut el in x {
        el.item_core.traits = fetch_item_traits(conn, gs, el.item_core.id).await?;
        result_vec.push(el);
    }
    Ok(result_vec)
}

async fn update_items_with_traits(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    mut items: Vec<Item>,
) -> Vec<Item> {
    for item in &mut items {
        item.traits = fetch_item_traits(conn, gs, item.id).await.unwrap_or(vec![]);
    }
    items
}

pub async fn fetch_items_with_filters(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    filters: &ShopFilterQuery,
) -> Result<Vec<Item>> {
    let items: Vec<Item> = query_as(sqlx::AssertSqlSafe(prepare_filtered_get_items(gs, filters)))
        .fetch_all(conn)
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
