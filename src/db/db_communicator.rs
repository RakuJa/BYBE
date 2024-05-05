use crate::models::creature::Creature;
use crate::models::db::raw_creature::RawCreature;
use crate::models::db::raw_immunity::RawImmunity;
use crate::models::db::raw_language::RawLanguage;
use crate::models::db::raw_resistance::RawResistance;
use crate::models::db::raw_sense::RawSense;
use crate::models::db::raw_speed::RawSpeed;
use crate::models::db::raw_trait::RawTrait;
use crate::models::db::raw_weakness::RawWeakness;
use crate::models::items::action::Action;
use crate::models::items::skill::Skill;
use crate::models::items::spell::Spell;
use crate::models::items::weapon::Weapon;
use crate::models::scales_struct::ability_scales::AbilityScales;
use crate::models::scales_struct::ac_scales::AcScales;
use crate::models::scales_struct::area_dmg_scales::AreaDmgScales;
use crate::models::scales_struct::creature_scales::CreatureScales;
use crate::models::scales_struct::hp_scales::HpScales;
use crate::models::scales_struct::item_scales::ItemScales;
use crate::models::scales_struct::perception_scales::PerceptionScales;
use crate::models::scales_struct::res_weak_scales::ResWeakScales;
use crate::models::scales_struct::saving_throw_scales::SavingThrowScales;
use crate::models::scales_struct::skill_scales::SkillScales;
use crate::models::scales_struct::spell_dc_and_atk_scales::SpellDcAndAtkScales;
use crate::models::scales_struct::strike_bonus_scales::StrikeBonusScales;
use crate::models::scales_struct::strike_dmg_scales::StrikeDmgScales;
use anyhow::Result;
use regex::Regex;
use sqlx::{Error, Pool, Sqlite};

async fn from_raw_vec_to_creature(
    conn: &Pool<Sqlite>,
    scales: &CreatureScales,
    raw_vec: Vec<RawCreature>,
) -> Vec<Creature> {
    let mut creature_list = Vec::new();
    let scales_dmg_regex = Regex::new(r"\((\d+)\)").ok().unwrap();
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
        let actions = get_creature_actions(conn, el.id).await.unwrap_or_default();
        let skills = get_creature_skills(conn, el.id).await.unwrap_or_default();
        creature_list.push(Creature::from((
            el,
            traits,
            weapons,
            spells,
            actions,
            skills,
            immunities,
            languages,
            resistances,
            senses,
            speeds,
            weaknesses,
            scales,
            &scales_dmg_regex,
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

async fn get_creature_actions(conn: &Pool<Sqlite>, creature_id: i64) -> Result<Vec<Action>> {
    Ok(sqlx::query_as!(
        Action,
        "SELECT * FROM ACTION_TABLE WHERE creature_id == ($1)",
        creature_id
    )
    .fetch_all(conn)
    .await?)
}

async fn get_creature_skills(conn: &Pool<Sqlite>, creature_id: i64) -> Result<Vec<Skill>> {
    Ok(sqlx::query_as!(
        Skill,
        "SELECT name, description, modifier, proficiency FROM SKILL_TABLE WHERE creature_id == ($1)",
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

pub async fn fetch_creatures(
    conn: &Pool<Sqlite>,
    creature_scales: &CreatureScales,
) -> Result<Vec<Creature>, Error> {
    Ok(from_raw_vec_to_creature(
        conn,
        creature_scales,
        sqlx::query_as!(RawCreature, "SELECT * FROM CREATURE_TABLE ORDER BY name")
            .fetch_all(conn)
            .await?,
    )
    .await)
}

pub async fn fetch_creature_scales(conn: &Pool<Sqlite>) -> Result<CreatureScales> {
    Ok(CreatureScales {
        ability_scales: sqlx::query_as!(AbilityScales, "SELECT * FROM ABILITY_SCALES_TABLE",)
            .fetch_all(conn)
            .await?
            .into_iter()
            .map(|n| (n.level as i8, n))
            .collect(),
        ac_scales: sqlx::query_as!(AcScales, "SELECT * FROM AC_SCALES_TABLE",)
            .fetch_all(conn)
            .await?
            .into_iter()
            .map(|n| (n.level as i8, n))
            .collect(),
        area_dmg_scales: sqlx::query_as!(AreaDmgScales, "SELECT * FROM AREA_DAMAGE_SCALES_TABLE",)
            .fetch_all(conn)
            .await?
            .into_iter()
            .map(|n| (n.level as i8, n))
            .collect(),
        hp_scales: sqlx::query_as!(HpScales, "SELECT * FROM HP_SCALES_TABLE",)
            .fetch_all(conn)
            .await?
            .into_iter()
            .map(|n| (n.level as i8, n))
            .collect(),
        item_scales: sqlx::query_as!(ItemScales, "SELECT * FROM ITEM_SCALES_TABLE",)
            .fetch_all(conn)
            .await?
            .into_iter()
            .map(|n| (n.cr_level.clone(), n))
            .collect(),
        perception_scales: sqlx::query_as!(
            PerceptionScales,
            "SELECT * FROM PERCEPTION_SCALES_TABLE",
        )
        .fetch_all(conn)
        .await?
        .into_iter()
        .map(|n| (n.level as i8, n))
        .collect(),
        res_weak_scales: sqlx::query_as!(ResWeakScales, "SELECT * FROM RES_WEAK_SCALES_TABLE",)
            .fetch_all(conn)
            .await?
            .into_iter()
            .map(|n| (n.level as i8, n))
            .collect(),
        saving_throw_scales: sqlx::query_as!(
            SavingThrowScales,
            "SELECT * FROM SAVING_THROW_SCALES_TABLE",
        )
        .fetch_all(conn)
        .await?
        .into_iter()
        .map(|n| (n.level as i8, n))
        .collect(),
        skill_scales: sqlx::query_as!(SkillScales, "SELECT * FROM SKILL_SCALES_TABLE",)
            .fetch_all(conn)
            .await?
            .into_iter()
            .map(|n| (n.level as i8, n))
            .collect(),
        spell_dc_and_atk_scales: sqlx::query_as!(
            SpellDcAndAtkScales,
            "SELECT * FROM SPELL_DC_AND_ATTACK_SCALES_TABLE",
        )
        .fetch_all(conn)
        .await?
        .into_iter()
        .map(|n| (n.level as i8, n))
        .collect(),
        strike_bonus_scales: sqlx::query_as!(
            StrikeBonusScales,
            "SELECT * FROM STRIKE_BONUS_SCALES_TABLE",
        )
        .fetch_all(conn)
        .await?
        .into_iter()
        .map(|n| (n.level as i8, n))
        .collect(),
        strike_dmg_scales: sqlx::query_as!(
            StrikeDmgScales,
            "SELECT * FROM STRIKE_DAMAGE_SCALES_TABLE",
        )
        .fetch_all(conn)
        .await?
        .into_iter()
        .map(|n| (n.level as i8, n))
        .collect(),
    })
}

// TODO: Restructure creature fetch to use transaction
