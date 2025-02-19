use anyhow::Result;
use sqlx::{Pool, Sqlite};

pub async fn create_creature_core_table(conn: &Pool<Sqlite>) -> Result<()> {
    delete_core_table(conn).await?;
    create_temporary_table(conn).await?;
    sqlx::query!(
        "
    CREATE TABLE IF NOT EXISTS CREATURE_CORE(
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
    )",
    )
    .execute(conn)
    .await?;
    insert_role_columns_in_core_table(conn).await?;
    Ok(())
}

async fn create_temporary_table(conn: &Pool<Sqlite>) -> Result<()> {
    sqlx::query!("
    CREATE TABLE IF NOT EXISTS TMP_CREATURE_CORE AS
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
                FROM WEAPON_CREATURE_ASSOCIATION_TABLE wcat LEFT JOIN (
	                SELECT * FROM WEAPON_TABLE w1 WHERE UPPER(w1.weapon_type) = 'MELEE'
                ) wt ON base_item_id = wcat.weapon_id
  		) THEN TRUE ELSE FALSE END AS is_melee,
        CASE WHEN ct.id IN (
            SELECT wcat.creature_id
            FROM WEAPON_CREATURE_ASSOCIATION_TABLE wcat LEFT JOIN (
                SELECT * FROM WEAPON_TABLE w1 WHERE UPPER(w1.weapon_type) = 'MELEE'
            ) wt ON base_item_id = wcat.weapon_id
        )
  		THEN TRUE ELSE FALSE END AS is_ranged,
        CASE WHEN st.creature_id IS NOT NULL THEN TRUE ELSE FALSE END AS is_spellcaster,
        CASE WHEN ct.aon_id IS NOT NULL THEN CONCAT('https://2e.aonprd.com/', CAST(UPPER(COALESCE(UPPER(ct.cr_type) , 'MONSTER')) AS TEXT), 's' , '.aspx?ID=', CAST(ct.aon_id AS TEXT)) ELSE NULL END AS archive_link,
        COALESCE(ct.cr_type , 'Monster') AS cr_type,
        COALESCE(ct.family , '-') AS family
        FROM CREATURE_TABLE ct
        LEFT JOIN SPELL_TABLE st ON ct.id = st.creature_id
        GROUP BY ct.id;
    "
        // Be careful, cr_type must be either Monster or NPC or we have runtime error
    ).execute(conn).await?;
    Ok(())
}

pub async fn initialize_data(conn: &Pool<Sqlite>) -> Result<()> {
    sqlx::query(
        "
        INSERT INTO CREATURE_CORE (
            id, aon_id, name, hp, level, size, rarity,
            license, source, remaster, is_melee, is_ranged,
            is_spellcaster, archive_link, cr_type, family, focus_points
        ) SELECT
            id, aon_id, name, hp, level, size, rarity,
            license, source, remaster, is_melee, is_ranged,
            is_spellcaster, archive_link, cr_type, family, focus_points
        FROM TMP_CREATURE_CORE;
        ",
    )
    .execute(conn)
    .await?;
    Ok(())
}

async fn insert_role_columns_in_core_table(conn: &Pool<Sqlite>) -> Result<()> {
    sqlx::query(
        "
        ALTER TABLE CREATURE_CORE ADD brute_percentage INTEGER NOT NULL DEFAULT 0;
        ALTER TABLE CREATURE_CORE ADD magical_striker_percentage INTEGER NOT NULL DEFAULT 0;
        ALTER TABLE CREATURE_CORE ADD skill_paragon_percentage INTEGER NOT NULL DEFAULT 0;
        ALTER TABLE CREATURE_CORE ADD skirmisher_percentage INTEGER NOT NULL DEFAULT 0;
        ALTER TABLE CREATURE_CORE ADD sniper_percentage INTEGER NOT NULL DEFAULT 0;
        ALTER TABLE CREATURE_CORE ADD soldier_percentage INTEGER NOT NULL DEFAULT 0;
        ALTER TABLE CREATURE_CORE ADD spellcaster_percentage INTEGER NOT NULL DEFAULT 0;
    ",
    )
    .execute(conn)
    .await?;
    Ok(())
}

/// Removes temporary tables created during execution of init
pub async fn cleanup_db(conn: &Pool<Sqlite>) -> Result<()> {
    sqlx::query("DROP TABLE TMP_CREATURE_CORE")
        .execute(conn)
        .await?;
    Ok(())
}

async fn delete_core_table(conn: &Pool<Sqlite>) -> Result<()> {
    sqlx::query!("DROP TABLE IF EXISTS CREATURE_CORE")
        .execute(conn)
        .await?;
    Ok(())
}
