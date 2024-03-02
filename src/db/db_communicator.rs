use crate::models::creature::Creature;
use crate::models::db::raw_creature::RawCreature;
use crate::models::db::raw_immunity::RawImmunity;
use crate::models::db::raw_language::RawLanguage;
use crate::models::db::raw_resistance::RawResistance;
use crate::models::db::raw_sense::RawSense;
use crate::models::db::raw_speed::RawSpeed;
use crate::models::db::raw_trait::RawTrait;
use crate::models::db::raw_weakness::RawWeakness;
use crate::models::items::spell::Spell;
use crate::models::items::weapon::Weapon;
use anyhow::Result;
use sqlx::{Error, Pool, Sqlite};

async fn from_raw_vec_to_creature(conn: &Pool<Sqlite>, raw_vec: Vec<RawCreature>) -> Vec<Creature> {
    let mut creature_list = Vec::new();
    for el in raw_vec {
        let immunities = get_creature_immunities(conn, el.id)
            .await
            .unwrap_or_default();
        let languages = get_creature_languages(conn, el.id)
            .await
            .unwrap_or_default();
        let resistances = get_creature_resistances(conn, el.id)
            .await
            .unwrap_or_default();
        let senses = get_creature_senses(conn, el.id).await.unwrap_or_default();
        let speeds = get_creature_speeds(conn, el.id).await.unwrap_or_default();
        let traits = get_creature_traits(conn, el.id).await.unwrap_or_default();
        let weaknesses = get_creature_weaknesses(conn, el.id)
            .await
            .unwrap_or_default();
        let spells = get_creature_spells(conn, el.id).await.unwrap_or_default();
        let weapons = get_creature_weapons(conn, el.id).await.unwrap_or_default();
        creature_list.push(Creature::from((
            el,
            traits,
            weapons,
            spells,
            immunities,
            languages,
            resistances,
            senses,
            speeds,
            weaknesses,
        )));
    }
    creature_list
}

async fn get_creature_immunities(
    conn: &Pool<Sqlite>,
    creature_id: i64,
) -> Result<Vec<RawImmunity>> {
    Ok(sqlx::query_as!(
        RawImmunity,
        "SELECT * FROM IMMUNITY_TABLE INTERSECT SELECT immunity_id FROM IMMUNITY_CREATURE_ASSOCIATION_TABLE WHERE creature_id == ($1)",
        creature_id
    ).fetch_all(conn).await?)
}

async fn get_creature_languages(conn: &Pool<Sqlite>, creature_id: i64) -> Result<Vec<RawLanguage>> {
    Ok(sqlx::query_as!(
        RawLanguage,
        "SELECT * FROM LANGUAGE_TABLE INTERSECT SELECT language_id FROM LANGUAGE_CREATURE_ASSOCIATION_TABLE WHERE creature_id == ($1)",
        creature_id
    ).fetch_all(conn).await?)
}

async fn get_creature_resistances(
    conn: &Pool<Sqlite>,
    creature_id: i64,
) -> Result<Vec<RawResistance>> {
    Ok(sqlx::query_as!(
        RawResistance,
        "SELECT name, value FROM RESISTANCE_TABLE WHERE creature_id == ($1)",
        creature_id
    )
    .fetch_all(conn)
    .await?)
}

async fn get_creature_senses(conn: &Pool<Sqlite>, creature_id: i64) -> Result<Vec<RawSense>> {
    Ok(sqlx::query_as!(
        RawSense,
        "SELECT * FROM SENSE_TABLE INTERSECT SELECT sense_id FROM SENSE_CREATURE_ASSOCIATION_TABLE WHERE creature_id == ($1)",
        creature_id
    ).fetch_all(conn).await?)
}

async fn get_creature_speeds(conn: &Pool<Sqlite>, creature_id: i64) -> Result<Vec<RawSpeed>> {
    Ok(sqlx::query_as!(
        RawSpeed,
        "SELECT name, value FROM SPEED_TABLE WHERE creature_id == ($1)",
        creature_id
    )
    .fetch_all(conn)
    .await?)
}

async fn get_creature_weaknesses(
    conn: &Pool<Sqlite>,
    creature_id: i64,
) -> Result<Vec<RawWeakness>> {
    Ok(sqlx::query_as!(
        RawWeakness,
        "SELECT name, value FROM WEAKNESS_TABLE WHERE creature_id == ($1)",
        creature_id
    )
    .fetch_all(conn)
    .await?)
}

async fn get_creature_traits(conn: &Pool<Sqlite>, creature_id: i64) -> Result<Vec<RawTrait>> {
    Ok(sqlx::query_as!(
        RawTrait,
        "SELECT * FROM TRAIT_TABLE INTERSECT SELECT trait_id FROM TRAIT_CREATURE_ASSOCIATION_TABLE WHERE creature_id == ($1)",
        creature_id
    ).fetch_all(conn).await?)
}

async fn get_creature_weapons(conn: &Pool<Sqlite>, creature_id: i64) -> Result<Vec<Weapon>> {
    Ok(sqlx::query_as!(
        Weapon,
        "SELECT * FROM WEAPON_TABLE WHERE creature_id == ($1)",
        creature_id
    )
    .fetch_all(conn)
    .await?)
}

async fn get_creature_spells(conn: &Pool<Sqlite>, creature_id: i64) -> Result<Vec<Spell>> {
    Ok(sqlx::query_as!(
        Spell,
        "SELECT * FROM SPELL_TABLE WHERE creature_id == ($1)",
        creature_id
    )
    .fetch_all(conn)
    .await?)
}

pub async fn fetch_creatures(conn: &Pool<Sqlite>) -> Result<Vec<Creature>, Error> {
    Ok(from_raw_vec_to_creature(
        conn,
        sqlx::query_as!(RawCreature, "SELECT * FROM CREATURE_TABLE ORDER BY name")
            .fetch_all(conn)
            .await?,
    )
    .await)
}
