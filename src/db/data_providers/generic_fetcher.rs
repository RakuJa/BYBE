use crate::models::item::weapon_struct::DamageData;
use anyhow::Result;
use sqlx::{Pool, Sqlite};

pub async fn fetch_unique_values_of_field(
    conn: &Pool<Sqlite>,
    table: &str,
    field: &str,
) -> Result<Vec<String>> {
    let query = format!(
        "SELECT CAST(t1.{field} AS TEXT) FROM ((SELECT DISTINCT ({field}) FROM {table})) t1"
    );
    Ok(sqlx::query_scalar(query.as_str()).fetch_all(conn).await?)
}

pub async fn fetch_item_traits(conn: &Pool<Sqlite>, item_id: i64) -> Result<Vec<String>> {
    Ok(sqlx::query_scalar!(
        "SELECT name
         FROM TRAIT_TABLE INTERSECT
         SELECT trait_id FROM TRAIT_ITEM_ASSOCIATION_TABLE WHERE item_id == ($1)
         ORDER BY name",
        item_id
    )
    .fetch_all(conn)
    .await?)
}

pub async fn fetch_weapon_traits(conn: &Pool<Sqlite>, weapon_id: i64) -> Result<Vec<String>> {
    Ok(sqlx::query_scalar!(
        "SELECT name
         FROM TRAIT_TABLE INTERSECT
         SELECT trait_id FROM TRAIT_WEAPON_ASSOCIATION_TABLE WHERE weapon_id == ($1)
         ORDER BY name",
        weapon_id
    )
    .fetch_all(conn)
    .await?)
}

pub async fn fetch_shield_traits(conn: &Pool<Sqlite>, shield_id: i64) -> Result<Vec<String>> {
    Ok(sqlx::query_scalar!(
        "SELECT name
         FROM TRAIT_TABLE INTERSECT
         SELECT trait_id FROM TRAIT_SHIELD_ASSOCIATION_TABLE WHERE shield_id == ($1)
         ORDER BY name",
        shield_id
    )
    .fetch_all(conn)
    .await?)
}

pub async fn fetch_armor_traits(conn: &Pool<Sqlite>, armor_id: i64) -> Result<Vec<String>> {
    Ok(sqlx::query_scalar!(
        "SELECT name
         FROM TRAIT_TABLE INTERSECT
         SELECT trait_id FROM TRAIT_ARMOR_ASSOCIATION_TABLE WHERE armor_id == ($1)
         ORDER BY name",
        armor_id
    )
    .fetch_all(conn)
    .await?)
}

pub async fn fetch_weapon_runes(conn: &Pool<Sqlite>, wp_id: i64) -> Result<Vec<String>> {
    Ok(sqlx::query_scalar!(
        "SELECT name
         FROM RUNE_TABLE INTERSECT
         SELECT rune_id FROM RUNE_WEAPON_ASSOCIATION_TABLE WHERE weapon_id == ($1)
         ORDER BY name",
        wp_id
    )
    .fetch_all(conn)
    .await?)
}

pub async fn fetch_weapon_damage_data(conn: &Pool<Sqlite>, wp_id: i64) -> Result<Vec<DamageData>> {
    Ok(sqlx::query_as(
        "SELECT id, bonus_dmg, dmg_type, number_of_dice, die_size
         FROM WEAPON_DAMAGE_TABLE dm RIGHT JOIN (
         SELECT id AS wp_id FROM WEAPON_TABLE WHERE wp_id == ($1)
         ) ON wp_id == dm.weapon_id",
    )
    .bind(wp_id)
    .fetch_all(conn)
    .await?)
}

pub async fn fetch_armor_runes(conn: &Pool<Sqlite>, wp_id: i64) -> Result<Vec<String>> {
    Ok(sqlx::query_scalar!(
        "SELECT name
         FROM RUNE_TABLE INTERSECT
         SELECT rune_id FROM RUNE_ARMOR_ASSOCIATION_TABLE WHERE armor_id == ($1)
         ORDER BY name",
        wp_id
    )
    .fetch_all(conn)
    .await?)
}
