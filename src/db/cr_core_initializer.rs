use crate::db::data_providers::creature_fetcher::{
    fetch_creature_combat_data, fetch_creature_extra_data, fetch_creature_scales,
    fetch_creature_spell_caster_data, fetch_creature_traits,
};
use crate::models::creature::creature_component::creature_core::EssentialData;
use crate::models::creature::creature_metadata::alignment_enum::AlignmentEnum;
use crate::models::creature::creature_metadata::creature_role::CreatureRoleEnum;
use crate::models::creature::creature_metadata::type_enum::CreatureTypeEnum;
use crate::models::shared::rarity_enum::RarityEnum;
use crate::models::shared::size_enum::SizeEnum;
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Sqlite};

/// Handler for startup, first creature_core initialization. Then it shouldn't be used

pub async fn update_creature_core_table(conn: &Pool<Sqlite>) -> Result<()> {
    let scales = fetch_creature_scales(conn).await?;
    for cr in get_creatures_raw_essential_data(conn, 0, -1).await? {
        let traits = fetch_creature_traits(conn, cr.id).await?;
        let alignment = AlignmentEnum::from((&traits, cr.remaster));
        update_alignment_column_value(conn, alignment.to_string(), cr.id).await?;
        let essential_data = EssentialData {
            id: cr.id,
            aon_id: cr.aon_id,
            name: cr.name,
            hp: cr.hp,
            base_level: cr.level,
            size: cr.size,
            family: cr.family.unwrap_or(String::from("-")),
            rarity: cr.rarity,
            license: cr.license,
            remaster: cr.remaster,
            source: cr.source,
            cr_type: CreatureTypeEnum::from(cr.cr_type),
            alignment,
        };
        let extra_data = fetch_creature_extra_data(conn, essential_data.id).await?;
        let combat_data = fetch_creature_combat_data(conn, essential_data.id).await?;
        let spell_caster_data = fetch_creature_spell_caster_data(conn, essential_data.id).await?;
        let roles = CreatureRoleEnum::from_creature_with_given_scales(
            &essential_data,
            &extra_data,
            &combat_data,
            &spell_caster_data,
            &scales,
        );

        for (curr_role, curr_percentage) in roles {
            update_role_column_value(conn, curr_role, curr_percentage, essential_data.id).await?;
        }
    }
    Ok(())
}

async fn update_role_column_value(
    conn: &Pool<Sqlite>,
    role: CreatureRoleEnum,
    value: i64,
    creature_id: i64,
) -> Result<()> {
    let x = match role {
        CreatureRoleEnum::Brute => {
            sqlx::query!(
                "UPDATE CREATURE_CORE SET brute_percentage = ? WHERE id = ?",
                value,
                creature_id
            )
        }
        CreatureRoleEnum::MagicalStriker => {
            sqlx::query!(
                "UPDATE CREATURE_CORE SET magical_striker_percentage = ? WHERE id = ?",
                value,
                creature_id
            )
        }
        CreatureRoleEnum::SkillParagon => {
            sqlx::query!(
                "UPDATE CREATURE_CORE SET skill_paragon_percentage = ? WHERE id = ?",
                value,
                creature_id
            )
        }
        CreatureRoleEnum::Skirmisher => {
            sqlx::query!(
                "UPDATE CREATURE_CORE SET skirmisher_percentage = ? WHERE id = ?",
                value,
                creature_id
            )
        }
        CreatureRoleEnum::Sniper => {
            sqlx::query!(
                "UPDATE CREATURE_CORE SET sniper_percentage = ? WHERE id = ?",
                value,
                creature_id
            )
        }
        CreatureRoleEnum::Soldier => {
            sqlx::query!(
                "UPDATE CREATURE_CORE SET soldier_percentage = ? WHERE id = ?",
                value,
                creature_id
            )
        }
        CreatureRoleEnum::SpellCaster => {
            sqlx::query!(
                "UPDATE CREATURE_CORE SET spell_caster_percentage = ? WHERE id = ?",
                value,
                creature_id
            )
        }
    }
    .execute(conn)
    .await?;
    if x.rows_affected() < 1 {
        bail!("Error encountered with creature id: {creature_id}. Could not update role: {role}")
    }
    Ok(())
}

async fn update_alignment_column_value(
    conn: &Pool<Sqlite>,
    alignment: String,
    creature_id: i64,
) -> Result<()> {
    let x = sqlx::query!(
        "UPDATE CREATURE_CORE SET alignment = ? WHERE id = ?",
        alignment,
        creature_id
    )
    .execute(conn)
    .await?;
    if x.rows_affected() < 1 {
        bail!("Error encountered with creature id: {creature_id}. Could not update alignment: {alignment}")
    }
    Ok(())
}

async fn get_creatures_raw_essential_data(
    conn: &Pool<Sqlite>,
    cursor: u32,
    page_size: i16,
) -> Result<Vec<RawEssentialData>> {
    Ok(sqlx::query_as!(
        RawEssentialData,
        "SELECT
            id, aon_id, name, hp, level, size, family, rarity,
            license, remaster, source, cr_type
        FROM CREATURE_TABLE ORDER BY name LIMIT ?,?",
        cursor,
        page_size
    )
    .fetch_all(conn)
    .await?)
}

#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct RawEssentialData {
    pub id: i64,
    pub aon_id: Option<i64>,
    pub name: String,
    pub hp: i64,
    pub level: i64,
    pub size: SizeEnum,
    pub family: Option<String>,
    pub rarity: RarityEnum,
    pub license: String,
    pub remaster: bool,
    pub source: String,
    pub cr_type: Option<String>,
}
