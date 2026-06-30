use crate::db::data_providers::generic_fetcher::{
    enrich_with_traits, fetch_action_traits, fetch_col_range, fetch_entity_traits,
};
use crate::db::data_providers::raw_query_builder::{
    format_pagination_clause, prepare_count_hazards_listing, prepare_filtered_get_hazards,
    prepare_paginated_get_hazards_listing,
};
use crate::models::hazard::hazard_field_filter::HazardFieldFilters;
use crate::models::hazard::hazard_listing_struct::{HazardFilterQuery, HazardSortEnum};
use crate::models::hazard::hazard_struct::{Hazard, HazardRanges};
use crate::models::response_data::ResponseHazard;
use crate::models::routers_validator_structs::OrderEnum;
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
    hazards: Vec<Hazard>,
) -> Vec<Hazard> {
    enrich_with_traits(pool, gs, hazards, true).await
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
    fetch_entity_traits(pool, gs, "hazard", hazard_id).await
}

pub async fn fetch_hazard_by_id(pool: &PgPool, gs: GameSystem, id: i64) -> Result<ResponseHazard> {
    let mut core_hazard: Hazard = sqlx::query_as(sqlx::AssertSqlSafe(format!(
        "SELECT * FROM {gs}_hazard_table WHERE id = $1 ORDER BY name LIMIT 1"
    )))
    .bind(id)
    .fetch_one(pool)
    .await?;
    core_hazard.traits = fetch_entity_traits(pool, gs, "hazard", id).await?;
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

pub async fn fetch_paginated_hazards(
    pool: &PgPool,
    gs: GameSystem,
    filters: &HazardFieldFilters,
    sort_by: HazardSortEnum,
    order_by: OrderEnum,
    cursor: u32,
    page_size: i16,
) -> Result<Vec<Hazard>> {
    let hazards: Vec<Hazard> = sqlx::query_as(sqlx::AssertSqlSafe(
        prepare_paginated_get_hazards_listing(gs, filters, sort_by, order_by, cursor, page_size),
    ))
    .fetch_all(pool)
    .await?;
    Ok(update_hazards_core_with_traits(pool, gs, hazards).await)
}

pub async fn fetch_hazards_listing_count(
    pool: &PgPool,
    gs: GameSystem,
    filters: &HazardFieldFilters,
) -> Result<i64> {
    Ok(
        sqlx::query_scalar(sqlx::AssertSqlSafe(prepare_count_hazards_listing(
            gs, filters,
        )))
        .fetch_one(pool)
        .await?,
    )
}

pub async fn fetch_hazard_ranges(pool: &PgPool, gs: GameSystem) -> Result<HazardRanges> {
    let from = format!("FROM {gs}_hazard_table");
    let (min_ac, max_ac) = fetch_col_range(pool, "ac", &from).await?;
    let (min_hardness, max_hardness) = fetch_col_range(pool, "hardness", &from).await?;
    let (min_hp, max_hp) = fetch_col_range(pool, "hp", &from).await?;
    let (min_stealth, max_stealth) = fetch_col_range(pool, "stealth", &from).await?;
    let (min_level, max_level) = fetch_col_range(pool, "level", &from).await?;
    let (min_will, max_will) = fetch_col_range(pool, "will", &from).await?;
    let (min_reflex, max_reflex) = fetch_col_range(pool, "reflex", &from).await?;
    let (min_fortitude, max_fortitude) = fetch_col_range(pool, "fortitude", &from).await?;
    Ok(HazardRanges {
        min_ac,
        max_ac,
        min_hardness,
        max_hardness,
        min_hp,
        max_hp,
        min_stealth,
        max_stealth,
        min_level,
        max_level,
        min_will,
        max_will,
        min_reflex,
        max_reflex,
        min_fortitude,
        max_fortitude,
    })
}
