use crate::db::data_providers::creature_fetcher::{
    fetch_creature_combat_data, fetch_creature_extra_data, fetch_creature_scales,
    fetch_creature_spellcaster_data, fetch_creature_traits,
};
use crate::models::creature::creature_component::creature_core::EssentialData;
use crate::models::creature::creature_metadata::creature_role::CreatureRoleEnum;
use crate::models::creature::creature_metadata::type_enum::CreatureTypeEnum;
use crate::models::shared::alignment_enum::AlignmentEnum;
use crate::models::shared::game_system_enum::GameSystem;
use crate::models::shared::rarity_enum::RarityEnum;
use crate::models::shared::size_enum::SizeEnum;
use crate::models::shared::status_enum::Status;
use anyhow::{Result, bail};
use log::warn;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Sqlite, Transaction};

pub async fn update_creature_core_table(conn: &Pool<Sqlite>, gs: &GameSystem) -> Result<()> {
    warn!("Handler for startup, Should only be used once for each gamesystem");
    let scales = fetch_creature_scales(conn).await?;
    let mut tx: Transaction<Sqlite> = conn.begin().await?;
    for cr in get_creatures_raw_essential_data(&mut tx, gs, 0, -1).await? {
        let traits = fetch_creature_traits(conn, gs, cr.id).await?;
        let alignment = AlignmentEnum::from((&traits, cr.remaster));
        update_alignment_column_value(&mut tx, gs, alignment.to_string(), cr.id).await?;
        let essential_data = EssentialData {
            id: cr.id,
            aon_id: cr.aon_id,
            name: cr.name,
            hp: cr.hp,
            base_level: cr.level,
            size: cr.size,
            family: cr.family.unwrap_or_else(|| String::from("-")),
            rarity: cr.rarity,
            license: cr.license,
            remaster: cr.remaster,
            source: cr.source,
            cr_type: CreatureTypeEnum::from(cr.cr_type),
            alignment,
            focus_points: cr.n_of_focus_points,
            status: cr.status,
        };
        let extra_data = fetch_creature_extra_data(conn, gs, essential_data.id).await?;
        let combat_data = fetch_creature_combat_data(conn, gs, essential_data.id).await?;
        let spellcaster_data = fetch_creature_spellcaster_data(conn, gs, essential_data.id).await?;
        let roles = CreatureRoleEnum::from_creature_with_given_scales(
            &essential_data,
            &extra_data,
            &combat_data,
            &spellcaster_data,
            &scales,
        );

        for (curr_role, curr_percentage) in roles {
            update_role_column_value(&mut tx, gs, curr_role, curr_percentage, essential_data.id)
                .await?;
        }
    }
    tx.commit().await?;
    Ok(())
}

async fn update_role_column_value_sf2e(
    conn: &mut Transaction<'_, Sqlite>,
    role: CreatureRoleEnum,
    value: i64,
    creature_id: i64,
) -> Result<()> {
    let x = match role {
        CreatureRoleEnum::Brute => {
            sqlx::query!(
                "UPDATE sf_creature_core SET brute_percentage = ? WHERE id = ?",
                value,
                creature_id
            )
        }
        CreatureRoleEnum::MagicalStriker => {
            sqlx::query!(
                "UPDATE sf_creature_core SET magical_striker_percentage = ? WHERE id = ?",
                value,
                creature_id
            )
        }
        CreatureRoleEnum::SkillParagon => {
            sqlx::query!(
                "UPDATE sf_creature_core SET skill_paragon_percentage = ? WHERE id = ?",
                value,
                creature_id
            )
        }
        CreatureRoleEnum::Skirmisher => {
            sqlx::query!(
                "UPDATE sf_creature_core SET skirmisher_percentage = ? WHERE id = ?",
                value,
                creature_id
            )
        }
        CreatureRoleEnum::Sniper => {
            sqlx::query!(
                "UPDATE sf_creature_core SET sniper_percentage = ? WHERE id = ?",
                value,
                creature_id
            )
        }
        CreatureRoleEnum::Soldier => {
            sqlx::query!(
                "UPDATE sf_creature_core SET soldier_percentage = ? WHERE id = ?",
                value,
                creature_id
            )
        }
        CreatureRoleEnum::Spellcaster => {
            sqlx::query!(
                "UPDATE sf_creature_core SET spellcaster_percentage = ? WHERE id = ?",
                value,
                creature_id
            )
        }
    }
    .execute(&mut **conn)
    .await?;
    if x.rows_affected() < 1 {
        bail!("Error encountered with creature id: {creature_id}. Could not update role: {role}")
    }
    Ok(())
}

async fn update_role_column_value_pf2e(
    conn: &mut Transaction<'_, Sqlite>,
    role: CreatureRoleEnum,
    value: i64,
    creature_id: i64,
) -> Result<()> {
    let x = match role {
        CreatureRoleEnum::Brute => {
            sqlx::query!(
                "UPDATE pf_creature_core SET brute_percentage = ? WHERE id = ?",
                value,
                creature_id
            )
        }
        CreatureRoleEnum::MagicalStriker => {
            sqlx::query!(
                "UPDATE pf_creature_core SET magical_striker_percentage = ? WHERE id = ?",
                value,
                creature_id
            )
        }
        CreatureRoleEnum::SkillParagon => {
            sqlx::query!(
                "UPDATE pf_creature_core SET skill_paragon_percentage = ? WHERE id = ?",
                value,
                creature_id
            )
        }
        CreatureRoleEnum::Skirmisher => {
            sqlx::query!(
                "UPDATE pf_creature_core SET skirmisher_percentage = ? WHERE id = ?",
                value,
                creature_id
            )
        }
        CreatureRoleEnum::Sniper => {
            sqlx::query!(
                "UPDATE pf_creature_core SET sniper_percentage = ? WHERE id = ?",
                value,
                creature_id
            )
        }
        CreatureRoleEnum::Soldier => {
            sqlx::query!(
                "UPDATE pf_creature_core SET soldier_percentage = ? WHERE id = ?",
                value,
                creature_id
            )
        }
        CreatureRoleEnum::Spellcaster => {
            sqlx::query!(
                "UPDATE pf_creature_core SET spellcaster_percentage = ? WHERE id = ?",
                value,
                creature_id
            )
        }
    }
    .execute(&mut **conn)
    .await?;
    if x.rows_affected() < 1 {
        bail!("Error encountered with creature id: {creature_id}. Could not update role: {role}")
    }
    Ok(())
}

async fn update_role_column_value(
    conn: &mut Transaction<'_, Sqlite>,
    gs: &GameSystem,
    role: CreatureRoleEnum,
    value: i64,
    creature_id: i64,
) -> Result<()> {
    match gs {
        GameSystem::Pathfinder => {
            update_role_column_value_pf2e(conn, role, value, creature_id).await?;
        }
        GameSystem::Starfinder => {
            update_role_column_value_sf2e(conn, role, value, creature_id).await?;
        }
    }
    Ok(())
}

async fn update_alignment_column_value(
    conn: &mut Transaction<'_, Sqlite>,
    gs: &GameSystem,
    alignment: String,
    creature_id: i64,
) -> Result<()> {
    let x = match gs {
        GameSystem::Pathfinder => {
            sqlx::query!(
                "UPDATE pf_creature_core SET alignment = ? WHERE id = ?",
                alignment,
                creature_id
            )
            .execute(&mut **conn)
            .await?
        }
        GameSystem::Starfinder => {
            sqlx::query!(
                "UPDATE sf_creature_core SET alignment = ? WHERE id = ?",
                alignment,
                creature_id
            )
            .execute(&mut **conn)
            .await?
        }
    };
    if x.rows_affected() < 1 {
        bail!(
            "Error encountered with creature id: {creature_id}. Could not update alignment: {alignment}"
        )
    }
    Ok(())
}

async fn get_creatures_raw_essential_data(
    conn: &mut Transaction<'_, Sqlite>,
    gs: &GameSystem,
    cursor: u32,
    page_size: i16,
) -> Result<Vec<RawEssentialData>> {
    Ok(match gs {
        GameSystem::Pathfinder => {
            sqlx::query_as!(
                RawEssentialData,
                "SELECT
                id, aon_id, name, hp, level, size, family, rarity,
                license, remaster, source, cr_type, n_of_focus_points, status
                FROM pf_creature_table ORDER BY name LIMIT ?,?",
                cursor,
                page_size
            )
            .fetch_all(&mut **conn)
            .await?
        }
        GameSystem::Starfinder => {
            sqlx::query_as!(
                RawEssentialData,
                "SELECT
                id, aon_id, name, hp, level, size, family, rarity,
                license, remaster, source, cr_type, n_of_focus_points, status
                FROM sf_creature_table ORDER BY name LIMIT ?,?",
                cursor,
                page_size
            )
            .fetch_all(&mut **conn)
            .await?
        }
    })
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
    pub n_of_focus_points: i64,
    pub status: Status,
}
