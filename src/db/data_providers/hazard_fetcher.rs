use crate::db::data_providers::generic_fetcher::fetch_action_traits;
use crate::db::data_providers::raw_query_builder::{
    format_pagination_clause, prepare_filtered_get_hazards,
};
use crate::models::hazard::hazard_listing_struct::HazardFilterQuery;
use crate::models::hazard::hazard_struct::Hazard;
use crate::models::response_data::ResponseHazard;
use crate::models::shared::action::{Action, CoreAction};
use crate::models::shared::alignment_enum::ALIGNMENT_TRAITS;
use crate::models::shared::game_system_enum::GameSystem;
use anyhow::Result;
use sqlx::PgPool;

async fn fetch_hazard_actions(
    pool: &PgPool,
    gs: GameSystem,
    hazard_id: i64,
) -> Result<Vec<Action>> {
    let core_actions: Vec<CoreAction> = sqlx::query_as(sqlx::AssertSqlSafe(format!(
        "SELECT a.* FROM {gs}_action_table AS a
        JOIN {gs}_action_hazard_association_table AS ca ON ca.action_id = a.id
        WHERE ca.hazard_id = $1"
    )))
    .bind(hazard_id)
    .fetch_all(pool)
    .await?;
    let mut res: Vec<Action> = Vec::with_capacity(core_actions.len());
    for action in core_actions {
        let action_id = action.id;
        res.push(Action {
            core_action: action,
            traits: fetch_action_traits(pool, gs, action_id).await?,
        });
    }

    Ok(res)
}

async fn update_hazards_core_with_traits(
    pool: &PgPool,
    gs: GameSystem,
    mut hazard: Vec<Hazard>,
) -> Vec<Hazard> {
    for core in &mut hazard {
        core.traits = fetch_hazard_traits(pool, gs, core.essential.id)
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
    pool: &PgPool,
    gs: GameSystem,
) -> Result<Vec<String>> {
    Ok(sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
        "
        SELECT
            tt.name
        FROM {gs}_trait_hazard_association_table tcat
            LEFT JOIN {gs}_trait_table tt ON tcat.trait_id = tt.name GROUP BY tt.name",
    )))
    .fetch_all(pool)
    .await?
    .iter()
    .filter(|x: &&String| !ALIGNMENT_TRAITS.contains(&&*x.to_uppercase()))
    .cloned()
    .collect())
}

pub async fn fetch_hazard_traits(
    pool: &PgPool,
    gs: GameSystem,
    hazard_id: i64,
) -> Result<Vec<String>> {
    Ok(sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
        "SELECT name FROM {gs}_trait_table INTERSECT SELECT trait_id
             FROM {gs}_trait_hazard_association_table WHERE hazard_id = $1"
    )))
    .bind(hazard_id)
    .fetch_all(pool)
    .await?)
}

pub async fn fetch_hazard_by_id(pool: &PgPool, gs: GameSystem, id: i64) -> Result<ResponseHazard> {
    let mut core_hazard: Hazard = sqlx::query_as(sqlx::AssertSqlSafe(format!(
        "SELECT * FROM {gs}_hazard_table WHERE id = $1 ORDER BY name LIMIT 1"
    )))
    .bind(id)
    .fetch_one(pool)
    .await?;
    core_hazard.traits = fetch_hazard_traits(pool, gs, id).await?;
    core_hazard.actions = fetch_hazard_actions(pool, gs, id).await?;

    Ok(ResponseHazard {
        core_hazard,
        game: gs,
    })
}

pub async fn fetch_hazard_core_data_with_filters(
    pool: &PgPool,
    gs: GameSystem,
    hazard_filter_query: &HazardFilterQuery,
) -> Result<Vec<Hazard>> {
    let query = prepare_filtered_get_hazards(gs, hazard_filter_query);
    let core_data: Vec<Hazard> = sqlx::query_as(sqlx::AssertSqlSafe(query))
        .fetch_all(pool)
        .await?;
    Ok(update_hazards_core_with_traits(pool, gs, core_data).await)
}

/// Gets all the hazard it can find with the given pagination as boundaries for the search.
pub async fn fetch_hazards_data(
    pool: &PgPool,
    gs: GameSystem,
    cursor: i64,
    page_size: i16,
) -> Result<Vec<Hazard>> {
    let pagination = format_pagination_clause(cursor, page_size);
    let cr_core: Vec<Hazard> = sqlx::query_as(sqlx::AssertSqlSafe(format!(
        "SELECT * FROM {gs}_hazard_table ORDER BY name {pagination}"
    )))
    .fetch_all(pool)
    .await?;
    Ok(update_hazards_core_with_traits(pool, gs, cr_core).await)
}
