use anyhow::Result;
use sqlx::{FromRow, Pool, Sqlite};

#[derive(FromRow)]
pub struct MyString {
    pub my_str: String,
}

pub async fn fetch_unique_values_of_field(
    conn: &Pool<Sqlite>,
    table: &str,
    field: &str,
) -> Result<Vec<String>> {
    let query = format!(
        "SELECT CAST(t1.{field} AS TEXT) AS my_str FROM ((SELECT DISTINCT ({field}) FROM {table})) t1"
    );
    let x: Vec<MyString> = sqlx::query_as(query.as_str()).fetch_all(conn).await?;
    Ok(x.iter().map(|x| x.my_str.clone()).collect())
}

pub async fn fetch_item_traits(conn: &Pool<Sqlite>, item_id: i64) -> Result<Vec<String>> {
    Ok(sqlx::query_as!(
        MyString,
        "SELECT name AS my_str
         FROM TRAIT_TABLE INTERSECT
         SELECT trait_id FROM TRAIT_ITEM_ASSOCIATION_TABLE WHERE item_id == ($1)",
        item_id
    )
    .fetch_all(conn)
    .await?
    .into_iter()
    .map(|x| x.my_str)
    .collect())
}

pub async fn fetch_weapon_runes(conn: &Pool<Sqlite>, wp_id: i64) -> Result<Vec<String>> {
    Ok(sqlx::query_as!(
        MyString,
        "SELECT name AS my_str
         FROM RUNE_TABLE INTERSECT
         SELECT rune_id FROM RUNE_WEAPON_ASSOCIATION_TABLE WHERE weapon_id == ($1)",
        wp_id
    )
    .fetch_all(conn)
    .await?
    .into_iter()
    .map(|x| x.my_str)
    .collect())
}

pub async fn fetch_armor_runes(conn: &Pool<Sqlite>, wp_id: i64) -> Result<Vec<String>> {
    Ok(sqlx::query_as!(
        MyString,
        "SELECT name AS my_str
         FROM RUNE_TABLE INTERSECT
         SELECT rune_id FROM RUNE_ARMOR_ASSOCIATION_TABLE WHERE armor_id == ($1)",
        wp_id
    )
    .fetch_all(conn)
    .await?
    .into_iter()
    .map(|x| x.my_str)
    .collect())
}
