use crate::models::item::weapon_struct::DamageData;
use crate::models::shared::game_system_enum::GameSystem;
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
    Ok(sqlx::query_scalar(sqlx::AssertSqlSafe(query))
        .fetch_all(conn)
        .await?)
}

pub async fn fetch_item_traits(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    item_id: i64,
) -> Result<Vec<String>> {
    let query = match gs {
        GameSystem::Pathfinder => {
            sqlx::query_scalar!(
                "SELECT name
                 FROM pf_trait_table INTERSECT
                 SELECT trait_id FROM pf_trait_item_association_table WHERE item_id == ($1)
                 ORDER BY name",
                item_id
            )
        }
        GameSystem::Starfinder => {
            sqlx::query_scalar!(
                "SELECT name
                 FROM sf_trait_table INTERSECT
                 SELECT trait_id FROM sf_trait_item_association_table WHERE item_id == ($1)
                 ORDER BY name",
                item_id
            )
        }
    };
    Ok(query.fetch_all(conn).await?)
}

pub async fn fetch_weapon_traits(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    weapon_id: i64,
) -> Result<Vec<String>> {
    let query = match gs {
        GameSystem::Pathfinder => {
            sqlx::query_scalar!(
                "SELECT name
                 FROM pf_trait_table INTERSECT
                 SELECT trait_id FROM pf_trait_weapon_association_table WHERE weapon_id == ($1)
                 ORDER BY name",
                weapon_id
            )
        }
        GameSystem::Starfinder => {
            sqlx::query_scalar!(
                "SELECT name
                 FROM sf_trait_table INTERSECT
                 SELECT trait_id FROM sf_trait_weapon_association_table WHERE weapon_id == ($1)
                 ORDER BY name",
                weapon_id
            )
        }
    };
    Ok(query.fetch_all(conn).await?)
}

pub async fn fetch_shield_traits(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    shield_id: i64,
) -> Result<Vec<String>> {
    let query = match gs {
        GameSystem::Pathfinder => {
            sqlx::query_scalar!(
                "SELECT name
                 FROM pf_trait_table INTERSECT
                 SELECT trait_id FROM pf_trait_shield_association_table WHERE shield_id == ($1)
                 ORDER BY name",
                shield_id
            )
        }
        GameSystem::Starfinder => {
            sqlx::query_scalar!(
                "SELECT name
                 FROM sf_trait_table INTERSECT
                 SELECT trait_id FROM sf_trait_shield_association_table WHERE shield_id == ($1)
                 ORDER BY name",
                shield_id
            )
        }
    };

    Ok(query.fetch_all(conn).await?)
}

pub async fn fetch_armor_traits(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    armor_id: i64,
) -> Result<Vec<String>> {
    let query = match gs {
        GameSystem::Pathfinder => {
            sqlx::query_scalar!(
                "SELECT name
                 FROM pf_trait_table INTERSECT
                 SELECT trait_id FROM pf_trait_armor_association_table WHERE armor_id == ($1)
                 ORDER BY name",
                armor_id
            )
        }
        GameSystem::Starfinder => {
            sqlx::query_scalar!(
                "SELECT name
                 FROM sf_trait_table INTERSECT
                 SELECT trait_id FROM sf_trait_armor_association_table WHERE armor_id == ($1)
                 ORDER BY name",
                armor_id
            )
        }
    };
    Ok(query.fetch_all(conn).await?)
}

pub async fn fetch_weapon_runes(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    wp_id: i64,
) -> Result<Vec<String>> {
    let query = match gs {
        GameSystem::Pathfinder => {
            sqlx::query_scalar!(
                "SELECT name
                 FROM pf_rune_table INTERSECT
                 SELECT rune_id FROM pf_rune_weapon_association_table WHERE weapon_id == ($1)
                 ORDER BY name",
                wp_id
            )
        }
        GameSystem::Starfinder => {
            sqlx::query_scalar!(
                "SELECT name
                 FROM sf_rune_table INTERSECT
                 SELECT rune_id FROM sf_rune_weapon_association_table WHERE weapon_id == ($1)
                 ORDER BY name",
                wp_id
            )
        }
    };

    Ok(query.fetch_all(conn).await?)
}

pub async fn fetch_weapon_damage_data(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    wp_id: i64,
) -> Result<Vec<DamageData>> {
    Ok(sqlx::query_as(sqlx::AssertSqlSafe(format!(
        "SELECT id, bonus_dmg, dmg_type, number_of_dice, die_size
             FROM {gs}_weapon_damage_table dm RIGHT JOIN (
             SELECT id AS wp_id FROM {gs}_weapon_table WHERE wp_id == ($1)
             ) ON wp_id == dm.weapon_id",
    )))
    .bind(wp_id)
    .fetch_all(conn)
    .await?)
}

pub async fn fetch_armor_runes(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    wp_id: i64,
) -> Result<Vec<String>> {
    let query = match gs {
        GameSystem::Pathfinder => {
            sqlx::query_scalar!(
                "SELECT name
                 FROM pf_rune_table INTERSECT
                 SELECT rune_id FROM pf_rune_armor_association_table WHERE armor_id == ($1)
                 ORDER BY name",
                wp_id
            )
        }
        GameSystem::Starfinder => {
            sqlx::query_scalar!(
                "SELECT name
                FROM sf_rune_table INTERSECT
                SELECT rune_id FROM sf_rune_armor_association_table WHERE armor_id == ($1)
                ORDER BY name",
                wp_id
            )
        }
    };
    Ok(query.fetch_all(conn).await?)
}
