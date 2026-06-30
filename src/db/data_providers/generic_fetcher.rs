use crate::models::item::weapon_struct::DamageData;
use crate::models::shared::game_system_enum::GameSystem;
use anyhow::Result;
use sqlx::PgPool;

pub async fn fetch_unique_values_of_field(
    pool: &PgPool,
    table: &str,
    field: &str,
) -> Result<Vec<String>> {
    let query = format!(
        "SELECT CAST(t1.{field} AS TEXT) FROM ((SELECT DISTINCT ({field}) FROM {table})) t1"
    );
    Ok(sqlx::query_scalar(sqlx::AssertSqlSafe(query))
        .fetch_all(pool)
        .await?)
}

pub async fn fetch_item_traits(pool: &PgPool, gs: GameSystem, item_id: i64) -> Result<Vec<String>> {
    Ok(sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
        "SELECT name FROM {gs}_trait_table INTERSECT
         SELECT trait_id FROM {gs}_trait_item_association_table WHERE item_id = $1
         ORDER BY name"
    )))
    .bind(item_id)
    .fetch_all(pool)
    .await?)
}

pub async fn fetch_weapon_traits(
    pool: &PgPool,
    gs: GameSystem,
    weapon_id: i64,
) -> Result<Vec<String>> {
    Ok(sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
        "SELECT name FROM {gs}_trait_table INTERSECT
         SELECT trait_id FROM {gs}_trait_weapon_association_table WHERE weapon_id = $1
         ORDER BY name"
    )))
    .bind(weapon_id)
    .fetch_all(pool)
    .await?)
}

pub async fn fetch_shield_traits(
    pool: &PgPool,
    gs: GameSystem,
    shield_id: i64,
) -> Result<Vec<String>> {
    Ok(sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
        "SELECT name FROM {gs}_trait_table INTERSECT
         SELECT trait_id FROM {gs}_trait_shield_association_table WHERE shield_id = $1
         ORDER BY name"
    )))
    .bind(shield_id)
    .fetch_all(pool)
    .await?)
}

pub async fn fetch_armor_traits(
    pool: &PgPool,
    gs: GameSystem,
    armor_id: i64,
) -> Result<Vec<String>> {
    Ok(sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
        "SELECT name FROM {gs}_trait_table INTERSECT
         SELECT trait_id FROM {gs}_trait_armor_association_table WHERE armor_id = $1
         ORDER BY name"
    )))
    .bind(armor_id)
    .fetch_all(pool)
    .await?)
}

pub async fn fetch_weapon_runes(pool: &PgPool, gs: GameSystem, wp_id: i64) -> Result<Vec<String>> {
    Ok(sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
        "SELECT name FROM {gs}_rune_table INTERSECT
         SELECT rune_id FROM {gs}_rune_weapon_association_table WHERE weapon_id = $1
         ORDER BY name"
    )))
    .bind(wp_id)
    .fetch_all(pool)
    .await?)
}

pub async fn fetch_weapon_damage_data(
    pool: &PgPool,
    gs: GameSystem,
    wp_id: i64,
) -> Result<Vec<DamageData>> {
    Ok(sqlx::query_as(sqlx::AssertSqlSafe(format!(
        "SELECT id, bonus_dmg, dmg_type, number_of_dice, die_size
             FROM {gs}_weapon_damage_table dm RIGHT JOIN (
             SELECT id AS wp_id FROM {gs}_weapon_table WHERE id = $1
             ) wt ON wt.wp_id = dm.weapon_id",
    )))
    .bind(wp_id)
    .fetch_all(pool)
    .await?)
}

pub async fn fetch_armor_runes(pool: &PgPool, gs: GameSystem, wp_id: i64) -> Result<Vec<String>> {
    Ok(sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
        "SELECT name FROM {gs}_rune_table INTERSECT
         SELECT rune_id FROM {gs}_rune_armor_association_table WHERE armor_id = $1
         ORDER BY name"
    )))
    .bind(wp_id)
    .fetch_all(pool)
    .await?)
}

pub async fn fetch_action_traits(
    pool: &PgPool,
    gs: GameSystem,
    action_id: i64,
) -> Result<Vec<String>> {
    Ok(sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
        "SELECT tt.name
        FROM {gs}_trait_action_association_table tcat
            LEFT JOIN {gs}_trait_table tt ON tcat.trait_id = tt.name WHERE action_id = $1 GROUP BY tt.name",
    ))).bind(action_id)
        .fetch_all(pool)
        .await?)
}
