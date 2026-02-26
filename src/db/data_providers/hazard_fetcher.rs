use crate::db::data_providers::raw_query_builder::prepare_filtered_get_hazards;
use crate::models::hazard::hazard_listing_struct::HazardFilterQuery;
use crate::models::hazard::hazard_struct::Hazard;
use crate::models::response_data::ResponseHazard;
use crate::models::shared::action::Action;
use crate::models::shared::alignment_enum::ALIGNMENT_TRAITS;
use crate::models::shared::game_system_enum::GameSystem;
use anyhow::Result;
use sqlx::{Pool, Sqlite};

async fn fetch_hazard_actions(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    hazard_id: i64,
) -> Result<Vec<Action>> {
    Ok(match gs {
        GameSystem::Pathfinder => {
            sqlx::query_as!(
                Action,
                "SELECT a.* FROM pf_action_table AS a
                JOIN pf_action_hazard_association_table AS ca ON ca.action_id = a.id
                WHERE ca.hazard_id == ($1)",
                hazard_id
            )
            .fetch_all(conn)
            .await?
        }
        GameSystem::Starfinder => {
            sqlx::query_as!(
                Action,
                "SELECT a.* FROM sf_action_table AS a
                JOIN sf_action_hazard_association_table AS ca ON ca.action_id = a.id
                WHERE ca.hazard_id == ($1)",
                hazard_id
            )
            .fetch_all(conn)
            .await?
        }
    })
}

async fn update_hazards_core_with_traits(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    mut hazard: Vec<Hazard>,
) -> Vec<Hazard> {
    for core in &mut hazard {
        core.traits = fetch_hazard_traits(conn, gs, core.essential.id)
            .await
            .unwrap_or_default()
            .iter()
            .filter(|x| !ALIGNMENT_TRAITS.contains(&&*x.as_str().to_uppercase()))
            .cloned()
            .collect();
    }
    hazard
}

pub async fn fetch_traits_associated_with_hazards(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
) -> Result<Vec<String>> {
    Ok(sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
        "
        SELECT
            tt.name
        FROM {gs}_trait_hazard_association_table tcat
            LEFT JOIN {gs}_trait_table tt ON tcat.trait_id = tt.name GROUP BY tt.name",
    )))
    .fetch_all(conn)
    .await?
    .iter()
    .filter(|x: &&String| !ALIGNMENT_TRAITS.contains(&&*x.to_uppercase()))
    .cloned()
    .collect())
}

pub async fn fetch_hazard_traits(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    hazard_id: i64,
) -> Result<Vec<String>> {
    Ok(sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
        "SELECT name FROM {gs}_trait_table INTERSECT SELECT trait_id
             FROM {gs}_trait_hazard_association_table WHERE hazard_id == ($1)"
    )))
    .bind(hazard_id)
    .fetch_all(conn)
    .await?)
}

pub async fn fetch_hazard_by_id(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    id: i64,
) -> Result<ResponseHazard> {
    let mut core_hazard: Hazard = sqlx::query_as(sqlx::AssertSqlSafe(format!(
        "SELECT * FROM {gs}_hazard_table WHERE id = ? ORDER BY name LIMIT 1"
    )))
    .bind(id)
    .fetch_one(conn)
    .await?;
    core_hazard.traits = fetch_hazard_traits(conn, gs, id).await?;
    core_hazard.actions = fetch_hazard_actions(conn, gs, id).await?;

    Ok(ResponseHazard {
        core_hazard,
        game: *gs,
    })
}

pub async fn fetch_hazard_core_data_with_filters(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    hazard_filter_query: &HazardFilterQuery,
) -> Result<Vec<Hazard>> {
    let query = prepare_filtered_get_hazards(gs, hazard_filter_query);
    let core_data: Vec<Hazard> = sqlx::query_as(sqlx::AssertSqlSafe(query))
        .fetch_all(conn)
        .await?;
    Ok(update_hazards_core_with_traits(conn, gs, core_data).await)
}

/// Gets all the hazard it can find with the given pagination as boundaries for the search.
pub async fn fetch_hazards_data(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    cursor: u32,
    page_size: i16,
) -> Result<Vec<Hazard>> {
    let cr_core: Vec<Hazard> = sqlx::query_as(sqlx::AssertSqlSafe(format!(
        "SELECT * FROM {gs}_hazard_table ORDER BY name LIMIT ?,?"
    )))
    .bind(cursor)
    .bind(page_size)
    .fetch_all(conn)
    .await?;
    Ok(update_hazards_core_with_traits(conn, gs, cr_core).await)
}
