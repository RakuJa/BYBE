use crate::db::data_providers::generic_fetcher::MyString;
use crate::db::data_providers::raw_query_builder::prepare_filtered_get_items;
use crate::models::db::raw_trait::RawTrait;
use crate::models::item::item_metadata::type_enum::ItemTypeEnum;
use crate::models::item::item_struct::Item;
use crate::models::shop_structs::ShopFilterQuery;
use anyhow::Result;
use log::debug;
use rand::Rng;
use sqlx::{query_as, Pool, Sqlite};

pub async fn fetch_item_by_id(conn: &Pool<Sqlite>, item_id: i64) -> Result<Item> {
    let mut item: Item =
        sqlx::query_as("SELECT * FROM ITEM_TABLE WHERE id = ? ORDER BY name LIMIT 1")
            .bind(item_id)
            .fetch_one(conn)
            .await?;
    item.traits = fetch_item_traits(conn, item_id)
        .await
        .unwrap_or_default()
        .iter()
        .map(|x| x.name.clone())
        .collect();
    Ok(item)
}

pub async fn fetch_items(conn: &Pool<Sqlite>, cursor: u32, page_size: i16) -> Result<Vec<Item>> {
    let items: Vec<Item> = sqlx::query_as("SELECT * FROM ITEM_TABLE ORDER BY name LIMIT ?,?")
        .bind(cursor)
        .bind(page_size)
        .fetch_all(conn)
        .await?;
    Ok(update_items_with_traits(conn, items).await)
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

async fn fetch_item_traits(conn: &Pool<Sqlite>, item_id: i64) -> Result<Vec<RawTrait>> {
    Ok(sqlx::query_as!(
        RawTrait,
        "SELECT * FROM TRAIT_TABLE INTERSECT SELECT trait_id FROM TRAIT_ITEM_ASSOCIATION_TABLE WHERE item_id == ($1)",
        item_id
    ).fetch_all(conn).await?)
}

async fn update_items_with_traits(conn: &Pool<Sqlite>, mut items: Vec<Item>) -> Vec<Item> {
    for item in &mut items {
        item.traits = fetch_item_traits(conn, item.id)
            .await
            .unwrap_or_default()
            .iter()
            .map(|x| x.name.clone())
            .collect();
    }
    items
}

pub async fn fetch_items_with_filters(
    conn: &Pool<Sqlite>,
    filters: &ShopFilterQuery,
) -> Result<Vec<Item>> {
    let result: Vec<Item> = query_as(prepare_filtered_get_items(filters).as_str())
        .fetch_all(conn)
        .await?;
    let equipment: Vec<Item> = result
        .iter()
        .filter(|x| x.item_type == ItemTypeEnum::Equipment)
        .cloned()
        .collect();
    let consumables: Vec<Item> = result
        .iter()
        .filter(|x| x.item_type == ItemTypeEnum::Consumable)
        .cloned()
        .collect();

    if result.len() as i64 >= filters.n_of_consumables + filters.n_of_equipment {
        debug!("Result vector is the correct size, no more operations needed");
        return Ok(result);
    }
    debug!("Result vector is not the correct size, duplicating random elements..");
    // We clone, otherwise we increment the probability of the same item being copied n times
    let mut item_vec = result.clone();
    for _ in 0..(equipment.len() as i64 - filters.n_of_equipment) {
        if let Some(x) = equipment.get(rand::thread_rng().gen_range(0..equipment.len())) {
            item_vec.push(x.clone());
        }
    }
    for _ in 0..(consumables.len() as i64 - filters.n_of_consumables) {
        if let Some(x) = consumables.get(rand::thread_rng().gen_range(0..consumables.len())) {
            item_vec.push(x.clone());
        }
    }
    Ok(result)
}
