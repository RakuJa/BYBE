use crate::db::data_providers::creature_fetcher::{
    fetch_all_creature_traits, fetch_creature_combat_data, fetch_creature_extra_data,
    fetch_creature_scales, fetch_creature_spellcaster_data,
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
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Sqlite, Transaction};
use std::collections::BTreeMap;
use tracing::warn;

pub async fn update_creature_core_table(conn: &Pool<Sqlite>, gs: &GameSystem) -> Result<()> {
    warn!("Handler for startup, Should only be used once for each gamesystem");
    let scales = fetch_creature_scales(conn).await?;
    let all_traits = fetch_all_creature_traits(conn, gs).await?;
    let mut tx: Transaction<Sqlite> = conn.begin().await?;
    for cr in get_creatures_raw_essential_data(&mut tx, gs, 0, -1).await? {
        let traits = all_traits.get(&cr.id).cloned().unwrap_or_default();
        let alignment = AlignmentEnum::from((&traits, cr.remaster));
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
        let (extra_data, combat_data, spellcaster_data) = tokio::try_join!(
            fetch_creature_extra_data(conn, gs, essential_data.id),
            fetch_creature_combat_data(conn, gs, essential_data.id),
            fetch_creature_spellcaster_data(conn, gs, essential_data.id),
        )?;
        let roles = CreatureRoleEnum::from_creature_with_given_scales(
            &essential_data,
            &extra_data,
            &combat_data,
            &spellcaster_data,
            &scales,
        );

        update_core_columns(
            &mut tx,
            gs,
            &roles,
            essential_data.alignment.to_string(),
            essential_data.id,
        )
        .await?;
    }
    tx.commit().await?;
    Ok(())
}

async fn update_core_columns(
    conn: &mut Transaction<'_, Sqlite>,
    gs: &GameSystem,
    roles: &BTreeMap<CreatureRoleEnum, i64>,
    alignment: String,
    creature_id: i64,
) -> Result<()> {
    let brute = find_role(roles, CreatureRoleEnum::Brute);
    let magical_striker = find_role(roles, CreatureRoleEnum::MagicalStriker);
    let skill_paragon = find_role(roles, CreatureRoleEnum::SkillParagon);
    let skirmisher = find_role(roles, CreatureRoleEnum::Skirmisher);
    let sniper = find_role(roles, CreatureRoleEnum::Sniper);
    let soldier = find_role(roles, CreatureRoleEnum::Soldier);
    let spellcaster = find_role(roles, CreatureRoleEnum::Spellcaster);

    let x = match gs {
        GameSystem::Pathfinder => {
            sqlx::query!(
                "UPDATE pf_creature_core
             SET alignment                  = ?,
                 brute_percentage           = ?,
                 magical_striker_percentage = ?,
                 skill_paragon_percentage   = ?,
                 skirmisher_percentage      = ?,
                 sniper_percentage          = ?,
                 soldier_percentage         = ?,
                 spellcaster_percentage     = ?
             WHERE id = ?",
                alignment,
                brute,
                magical_striker,
                skill_paragon,
                skirmisher,
                sniper,
                soldier,
                spellcaster,
                creature_id
            )
            .execute(&mut **conn)
            .await?
        }
        GameSystem::Starfinder => {
            sqlx::query!(
                "UPDATE sf_creature_core
             SET alignment                  = ?,
                 brute_percentage           = ?,
                 magical_striker_percentage = ?,
                 skill_paragon_percentage   = ?,
                 skirmisher_percentage      = ?,
                 sniper_percentage          = ?,
                 soldier_percentage         = ?,
                 spellcaster_percentage     = ?
             WHERE id = ?",
                alignment,
                brute,
                magical_striker,
                skill_paragon,
                skirmisher,
                sniper,
                soldier,
                spellcaster,
                creature_id
            )
            .execute(&mut **conn)
            .await?
        }
    };

    if x.rows_affected() < 1 {
        bail!("Could not update core columns for creature id: {creature_id}");
    }
    Ok(())
}

fn find_role(roles: &BTreeMap<CreatureRoleEnum, i64>, target: CreatureRoleEnum) -> i64 {
    roles.get(&target).copied().unwrap_or(0)
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
