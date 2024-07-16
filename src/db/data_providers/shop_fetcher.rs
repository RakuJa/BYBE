use crate::db::data_providers::generic_fetcher::{
    fetch_armor_runes, fetch_item_traits, fetch_weapon_runes, MyString,
};
use crate::db::data_providers::raw_query_builder::prepare_filtered_get_items;
use crate::models::item::armor_struct::{Armor, ArmorData};
use crate::models::item::item_metadata::type_enum::ItemTypeEnum;
use crate::models::item::item_struct::Item;
use crate::models::item::shield_struct::{Shield, ShieldData};
use crate::models::item::weapon_struct::{Weapon, WeaponData};
use crate::models::response_data::ResponseItem;
use crate::models::shop_structs::ShopFilterQuery;
use anyhow::Result;
use log::debug;
use rand::Rng;
use sqlx::{query_as, Pool, Sqlite};

pub async fn fetch_item_by_id(conn: &Pool<Sqlite>, item_id: i64) -> Result<ResponseItem> {
    let mut item: Item =
        sqlx::query_as("SELECT * FROM ITEM_TABLE WHERE id = ? ORDER BY name LIMIT 1")
            .bind(item_id)
            .fetch_one(conn)
            .await?;
    item.traits = fetch_item_traits(conn, item_id).await?;
    Ok(match item.item_type {
        ItemTypeEnum::Consumable | ItemTypeEnum::Equipment => ResponseItem::from(item),
        ItemTypeEnum::Weapon => ResponseItem {
            core_item: item,
            weapon_data: fetch_weapon_data_by_item_id(conn, item_id).await.ok(),
            armor_data: None,
            shield_data: None,
        },
        ItemTypeEnum::Armor => ResponseItem {
            core_item: item,
            weapon_data: None,
            armor_data: fetch_armor_data_by_item_id(conn, item_id).await.ok(),
            shield_data: None,
        },
        ItemTypeEnum::Shield => ResponseItem {
            core_item: item,
            weapon_data: None,
            armor_data: None,
            shield_data: fetch_shield_data_by_item_id(conn, item_id).await.ok(),
        },
    })
}

async fn fetch_weapon_by_item_id(conn: &Pool<Sqlite>, item_id: i64) -> Result<Weapon> {
    let mut weapon: Weapon = sqlx::query_as(
        "
        SELECT wt.id AS weapon_id, wt.bonus_dmg, wt.to_hit_bonus, wt.dmg_type,
        wt.number_of_dice, wt.die_size, wt.splash_dmg, wt.n_of_potency_runes,
        wt.n_of_striking_runes, wt.range, wt.reload, wt.weapon_type, wt.base_item_id,
        it.*
        FROM WEAPON_TABLE wt
        LEFT JOIN ITEM_TABLE it ON wt.base_item_id = it.id
        WHERE wt.base_item_id = ($1)
        ",
    )
    .bind(item_id)
    .fetch_one(conn)
    .await?;
    weapon.item_core.traits = fetch_item_traits(conn, item_id).await?;
    weapon.weapon_data.property_runes = fetch_weapon_runes(conn, weapon.weapon_data.id).await?;
    Ok(weapon)
}

async fn fetch_armor_by_item_id(conn: &Pool<Sqlite>, item_id: i64) -> Result<Armor> {
    let mut armor: Armor = sqlx::query_as(
        "
        SELECT at.id AS armor_id, at.bonus_ac, at.check_penalty, at.dex_cap, at.n_of_potency_runes,
        at.n_of_resilient_runes, at.speed_penalty, at.strength_required, at.base_item_id,
        it.*
        FROM ARMOR_TABLE at
        LEFT JOIN ITEM_TABLE it ON at.base_item_id = it.id
        WHERE at.base_item_id = ($1)
        ",
    )
    .bind(item_id)
    .fetch_one(conn)
    .await?;
    armor.item_core.traits = fetch_item_traits(conn, item_id).await?;
    armor.armor_data.property_runes = fetch_armor_runes(conn, armor.armor_data.id).await?;
    Ok(armor)
}

async fn fetch_shield_by_item_id(conn: &Pool<Sqlite>, item_id: i64) -> Result<Shield> {
    let mut shield: Shield = sqlx::query_as(
        "
        SELECT st.id AS shield_id, st.bonus_ac, st.n_of_reinforcing_runes, st.speed_penalty,
        it.*
        FROM SHIELD_TABLE st
        LEFT JOIN ITEM_TABLE it ON st.base_item_id = it.id
        WHERE st.base_item_id = ($1)
        ",
    )
    .bind(item_id)
    .fetch_one(conn)
    .await?;
    shield.item_core.traits = fetch_item_traits(conn, item_id).await?;
    Ok(shield)
}

async fn fetch_weapon_data_by_item_id(conn: &Pool<Sqlite>, item_id: i64) -> Result<WeaponData> {
    Ok(fetch_weapon_by_item_id(conn, item_id).await?.weapon_data)
}

async fn fetch_armor_data_by_item_id(conn: &Pool<Sqlite>, item_id: i64) -> Result<ArmorData> {
    Ok(fetch_armor_by_item_id(conn, item_id).await?.armor_data)
}

async fn fetch_shield_data_by_item_id(conn: &Pool<Sqlite>, item_id: i64) -> Result<ShieldData> {
    Ok(fetch_shield_by_item_id(conn, item_id).await?.shield_data)
}

pub async fn fetch_items(conn: &Pool<Sqlite>, cursor: u32, page_size: i16) -> Result<Vec<Item>> {
    let items: Vec<Item> = sqlx::query_as(
        "
        SELECT * FROM ITEM_TABLE it
        LEFT OUTER JOIN ITEM_CREATURE_ASSOCIATION_TABLE icat
        ON it.id = icat.item_id WHERE icat.item_id IS NULL
        AND UPPER(item_type) == 'EQUIPMENT' OR UPPER(item_type) == 'CONSUMABLE'
        GROUP BY it.id
        ORDER BY name LIMIT ?,?",
    )
    .bind(cursor)
    .bind(page_size)
    .fetch_all(conn)
    .await?;
    Ok(update_items_with_traits(conn, items).await)
}

pub async fn fetch_weapons(
    conn: &Pool<Sqlite>,
    cursor: u32,
    page_size: i16,
) -> Result<Vec<Weapon>> {
    let x: Vec<Weapon> = sqlx::query_as(
        "
        SELECT wt.id AS weapon_id, wt.bonus_dmg, wt.to_hit_bonus, wt.dmg_type, wt.number_of_dice, wt.die_size, wt.splash_dmg,
        wt.n_of_potency_runes, wt.n_of_striking_runes, wt.range, wt.reload, wt.weapon_type, wt.base_item_id,
        it.*
        FROM WEAPON_TABLE wt
        LEFT OUTER JOIN ITEM_CREATURE_ASSOCIATION_TABLE icat
        ON wt.base_item_id = icat.item_id
        LEFT JOIN ITEM_TABLE it ON wt.base_item_id = it.id
        WHERE icat.item_id IS NULL
        GROUP BY it.id
        ORDER BY name LIMIT ?,?
    ",
    )
    .bind(cursor)
    .bind(page_size)
    .fetch_all(conn)
    .await?;
    let mut result_vec = Vec::new();
    for mut el in x {
        el.item_core.traits = fetch_item_traits(conn, el.item_core.id).await?;
        el.weapon_data.property_runes = fetch_weapon_runes(conn, el.weapon_data.id).await?;
        result_vec.push(Weapon {
            item_core: el.item_core,
            weapon_data: el.weapon_data,
        })
    }
    Ok(result_vec)
}

pub async fn fetch_armors(conn: &Pool<Sqlite>, cursor: u32, page_size: i16) -> Result<Vec<Armor>> {
    let x: Vec<Armor> = sqlx::query_as(
        "
        SELECT at.id AS armor_id, at.bonus_ac, at.check_penalty, at.dex_cap, at.n_of_potency_runes,
        at.n_of_resilient_runes, at.speed_penalty, at.strength_required, at.base_item_id,
        it.*
        FROM ARMOR_TABLE at
        LEFT OUTER JOIN ITEM_CREATURE_ASSOCIATION_TABLE icat
        ON at.base_item_id = icat.item_id
        LEFT JOIN ITEM_TABLE it ON at.base_item_id = it.id
        WHERE icat.item_id IS NULL
        GROUP BY it.id
        ORDER BY name LIMIT ?,?
    ",
    )
    .bind(cursor)
    .bind(page_size)
    .fetch_all(conn)
    .await?;
    let mut result_vec = Vec::new();
    for mut el in x {
        el.item_core.traits = fetch_item_traits(conn, el.item_core.id).await?;
        el.armor_data.property_runes = fetch_armor_runes(conn, el.armor_data.id).await?;
        result_vec.push(Armor {
            item_core: el.item_core,
            armor_data: el.armor_data,
        })
    }
    Ok(result_vec)
}

pub async fn fetch_shields(
    conn: &Pool<Sqlite>,
    cursor: u32,
    page_size: i16,
) -> Result<Vec<Shield>> {
    let x: Vec<Shield> = sqlx::query_as(
        "
        SELECT st.id AS shield_id, st.bonus_ac, st.n_of_reinforcing_runes, st.speed_penalty,
        it.*
        FROM SHIELD_TABLE st
        LEFT OUTER JOIN ITEM_CREATURE_ASSOCIATION_TABLE icat
        ON st.base_item_id = icat.item_id
        LEFT JOIN ITEM_TABLE it ON st.base_item_id = it.id
        WHERE icat.item_id IS NULL
        GROUP BY it.id
        ORDER BY name LIMIT ?,?
    ",
    )
    .bind(cursor)
    .bind(page_size)
    .fetch_all(conn)
    .await?;
    let mut result_vec = Vec::new();
    for mut el in x {
        el.item_core.traits = fetch_item_traits(conn, el.item_core.id).await?;
        result_vec.push(Shield {
            item_core: el.item_core,
            shield_data: el.shield_data,
        })
    }
    Ok(result_vec)
}

pub async fn fetch_traits_associated_with_items(conn: &Pool<Sqlite>) -> Result<Vec<String>> {
    let x: Vec<MyString> = sqlx::query_as(
        "
        SELECT
            tt.name AS my_str
        FROM TRAIT_ITEM_ASSOCIATION_TABLE tiat
            LEFT JOIN TRAIT_TABLE tt ON tiat.trait_id = tt.name GROUP BY tt.name",
    )
    .fetch_all(conn)
    .await?;
    Ok(x.iter().map(|x| x.my_str.clone()).collect())
}

async fn update_items_with_traits(conn: &Pool<Sqlite>, mut items: Vec<Item>) -> Vec<Item> {
    for item in &mut items {
        item.traits = fetch_item_traits(conn, item.id).await.unwrap_or(vec![]);
    }
    items
}

pub async fn fetch_items_with_filters(
    conn: &Pool<Sqlite>,
    filters: &ShopFilterQuery,
) -> Result<Vec<Item>> {
    let items: Vec<Item> = query_as(prepare_filtered_get_items(filters).as_str())
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

    if items.len() as i64 >= filters.n_of_consumables + filters.n_of_equipment {
        debug!("Result vector is the correct size, no more operations needed");
        return Ok(items);
    }
    debug!("Result vector is not the correct size, duplicating random elements..");

    let mut item_vec = fill_item_vec_to_len(&equipment, filters.n_of_equipment);
    item_vec.extend(fill_item_vec_to_len(&consumables, filters.n_of_consumables));
    item_vec.extend(fill_item_vec_to_len(&weapons, filters.n_of_weapons));
    item_vec.extend(fill_item_vec_to_len(&armors, filters.n_of_armors));
    item_vec.extend(fill_item_vec_to_len(&shields, filters.n_of_shields));

    Ok(item_vec)
}

fn fill_item_vec_to_len(item_vec: &[&Item], desired_len: i64) -> Vec<Item> {
    let mut og_vec: Vec<Item> = item_vec.iter().map(|x| (*x).clone()).collect();
    for _ in 0..(item_vec.len() as i64 - desired_len) {
        if let Some(x) = item_vec.get(rand::thread_rng().gen_range(0..item_vec.len())) {
            og_vec.push((*x).clone());
        }
    }
    og_vec
}
