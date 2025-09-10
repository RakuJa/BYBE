use anyhow::Result;
use sqlx::{Pool, Sqlite};

use crate::GameSystem;

pub async fn create_creature_core_table(conn: &Pool<Sqlite>, gs: &GameSystem) -> Result<()> {
    delete_core_table(conn, gs).await?;
    create_temporary_table(conn, gs).await?;
    sqlx::query(
        format!(
            "
    CREATE TABLE IF NOT EXISTS {gs}_creature_core(
        id INTEGER PRIMARY KEY NOT NULL,
        aon_id INTEGER,
        name TEXT NOT NULL  DEFAULT '',
        hp INTEGER NOT NULL  DEFAULT -1,
        level INTEGER NOT NULL  DEFAULT -99,
        size TEXT NOT NULL  DEFAULT 'MEDIUM',
        rarity TEXT NOT NULL DEFAULT 'COMMON',
        is_melee BOOL NOT NULL DEFAULT 0,
        is_ranged BOOL NOT NULL DEFAULT 0,
        is_spellcaster BOOL NOT NULL DEFAULT 0,
        focus_points INTEGER NOT NULL DEFAULT -99,
        archive_link TEXT,
        cr_type TEXT NOT NULL DEFAULT 'MONSTER',
        family TEXT NOT NULL DEFAULT '-',
        license TEXT NOT NULL DEFAULT '',
        source TEXT NOT NULL DEFAULT '',
        remaster BOOL NOT NULL DEFAULT 0,
        alignment TEXT NOT NULL DEFAULT NO
    )"
        )
        .as_str(),
    )
    .execute(conn)
    .await?;
    insert_role_columns_in_core_table(conn, gs).await?;
    Ok(())
}

async fn create_temporary_table(conn: &Pool<Sqlite>, gs: &GameSystem) -> Result<()> {
    sqlx::query(format!("
    CREATE TABLE IF NOT EXISTS {gs}_tmp_creature_core AS
        SELECT
        ct.id,
        ct.aon_id,
        ct.name,
        ct.hp,
        ct.LEVEL,
        ct.SIZE,
        ct.rarity,
        ct.license,
        ct.source,
        ct.n_of_focus_points as focus_points,
        ct.remaster,
      	CASE WHEN ct.id IN (
      		SELECT wcat.creature_id
                FROM {gs}_weapon_creature_association_table wcat LEFT JOIN (
	                SELECT * FROM {gs}_weapon_table w1 WHERE UPPER(w1.weapon_type) = 'MELEE'
                ) wt ON base_item_id = wcat.weapon_id
  		) THEN TRUE ELSE FALSE END AS is_melee,
        CASE WHEN ct.id IN (
            SELECT wcat.creature_id
            FROM {gs}_weapon_creature_association_table wcat LEFT JOIN (
                SELECT * FROM {gs}_weapon_table w1 WHERE UPPER(w1.weapon_type) = 'MELEE'
            ) wt ON base_item_id = wcat.weapon_id
        )
  		THEN TRUE ELSE FALSE END AS is_ranged,
        CASE WHEN st.creature_id IS NOT NULL THEN TRUE ELSE FALSE END AS is_spellcaster,
        CASE WHEN ct.aon_id IS NOT NULL THEN CONCAT('https://2e.aonprd.com/', CAST(UPPER(COALESCE(UPPER(ct.cr_type) , 'MONSTER')) AS TEXT), 's' , '.aspx?ID=', CAST(ct.aon_id AS TEXT)) ELSE NULL END AS archive_link,
        COALESCE(ct.cr_type , 'Monster') AS cr_type,
        COALESCE(ct.family , '-') AS family
        FROM {gs}_creature_table ct
        LEFT JOIN {gs}_spell_table st ON ct.id = st.creature_id
        GROUP BY ct.id;
    ").as_str()
        // Be careful, cr_type must be either Monster or NPC or we have runtime error
    ).execute(conn).await?;
    Ok(())
}

pub async fn initialize_data(conn: &Pool<Sqlite>, gs: &GameSystem) -> Result<()> {
    sqlx::query(
        format!(
            "
        INSERT INTO {gs}_creature_core (
            id, aon_id, name, hp, level, size, rarity,
            license, source, remaster, is_melee, is_ranged,
            is_spellcaster, archive_link, cr_type, family, focus_points
        ) SELECT
            id, aon_id, name, hp, level, size, rarity,
            license, source, remaster, is_melee, is_ranged,
            is_spellcaster, archive_link, cr_type, family, focus_points
        FROM {gs}_tmp_creature_core;
        "
        )
        .as_str(),
    )
    .execute(conn)
    .await?;
    Ok(())
}

async fn insert_role_columns_in_core_table(conn: &Pool<Sqlite>, gs: &GameSystem) -> Result<()> {
    sqlx::query(
        format!(
            "
        ALTER TABLE {gs}_creature_core ADD brute_percentage INTEGER NOT NULL DEFAULT 0;
        ALTER TABLE {gs}_creature_core ADD magical_striker_percentage INTEGER NOT NULL DEFAULT 0;
        ALTER TABLE {gs}_creature_core ADD skill_paragon_percentage INTEGER NOT NULL DEFAULT 0;
        ALTER TABLE {gs}_creature_core ADD skirmisher_percentage INTEGER NOT NULL DEFAULT 0;
        ALTER TABLE {gs}_creature_core ADD sniper_percentage INTEGER NOT NULL DEFAULT 0;
        ALTER TABLE {gs}_creature_core ADD soldier_percentage INTEGER NOT NULL DEFAULT 0;
        ALTER TABLE {gs}_creature_core ADD spellcaster_percentage INTEGER NOT NULL DEFAULT 0;
    "
        )
        .as_str(),
    )
    .execute(conn)
    .await?;
    Ok(())
}

/// Removes temporary tables created during execution of init
pub async fn cleanup_db(conn: &Pool<Sqlite>, gs: &GameSystem) -> Result<()> {
    sqlx::query(format!("DROP TABLE {gs}_tmp_creature_core").as_str())
        .execute(conn)
        .await?;
    Ok(())
}

async fn delete_core_table(conn: &Pool<Sqlite>, gs: &GameSystem) -> Result<()> {
    sqlx::query(format!("DROP TABLE IF EXISTS {gs}_creature_core").as_str())
        .execute(conn)
        .await?;
    Ok(())
}
