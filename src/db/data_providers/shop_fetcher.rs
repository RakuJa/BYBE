use crate::db::data_providers::generic_fetcher::MyString;
use crate::models::db::raw_trait::RawTrait;
use crate::models::item::item_struct::Item;
use crate::models::routers_validator_structs::PaginatedRequest;
use anyhow::Result;
use sqlx::{Pool, Sqlite};

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

pub async fn fetch_items(
    conn: &Pool<Sqlite>,
    paginated_request: &PaginatedRequest,
) -> Result<Vec<Item>> {
    let items: Vec<Item> = sqlx::query_as("SELECT * FROM ITEM_TABLE ORDER BY name LIMIT ?,?")
        .bind(paginated_request.cursor)
        .bind(paginated_request.page_size)
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
