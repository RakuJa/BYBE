use crate::db::data_providers::generic_fetcher::{
    enrich_with_traits, fetch_action_traits, fetch_all_with_binds, fetch_all_with_binds_and_count,
    fetch_col_range, fetch_entity_traits, fetch_weapon_actions, fetch_weapon_damage_data,
    fetch_weapon_runes, fetch_weapon_traits,
};
use crate::db::data_providers::raw_query_builder::{
    format_pagination_clause, prepare_filtered_get_hazards, prepare_paginated_get_hazards_listing,
};
use crate::models::db::resistance::{CoreResistanceData, Resistance};
use crate::models::db::weakness::Weakness;
use crate::models::hazard::hazard_field_filter::HazardFieldFilters;
use crate::models::hazard::hazard_listing_struct::{HazardFilterQuery, HazardSortEnum};
use crate::models::hazard::hazard_struct::{Hazard, HazardRanges};
use crate::models::item::weapon_struct::Weapon;
use crate::models::response_data::ResponseHazard;
use crate::models::routers_validator_structs::OrderEnum;
use crate::models::shared::action::{Action, CoreAction};
use crate::models::shared::alignment_enum::ALIGNMENT_TRAITS;
use crate::models::shared::game_system_enum::GameSystem;
use crate::models::shared::trait_data::TraitData;
use anyhow::Result;
use futures::future::join_all;
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
) -> Result<Vec<TraitData>> {
    Ok(sqlx::query_as(sqlx::AssertSqlSafe(format!(
        "
        SELECT
            tt.name, tt.description, tt.display_name
        FROM {gs}_trait_hazard_association_table tcat
            LEFT JOIN {gs}_trait_table tt ON tcat.trait_id = tt.name GROUP BY tt.name",
    )))
    .fetch_all(pool)
    .await?
    .iter()
    .filter(|x: &&TraitData| !ALIGNMENT_TRAITS.contains(&&*x.name.to_uppercase()))
    .cloned()
    .collect())
}

pub async fn fetch_hazard_traits(
    pool: &PgPool,
    gs: GameSystem,
    hazard_id: i64,
) -> Result<Vec<TraitData>> {
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
    core_hazard.resistances = fetch_hazard_resistances(pool, gs, id)
        .await
        .unwrap_or_default();
    core_hazard.immunities = fetch_hazard_immunities(pool, gs, id)
        .await
        .unwrap_or_default()
        .into_iter()
        .flatten()
        .collect();
    core_hazard.weaknesses = fetch_hazard_weaknesses(pool, gs, id).await?;
    core_hazard.weapons = fetch_hazard_weapons(pool, gs, id).await?;

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
    let (query, binds) = prepare_filtered_get_hazards(gs, hazard_filter_query);
    let core_data: Vec<Hazard> = fetch_all_with_binds(pool, query, binds).await?;
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
    let hz_core: Vec<Hazard> = sqlx::query_as(sqlx::AssertSqlSafe(format!(
        "SELECT * FROM {gs}_hazard_table ORDER BY name {pagination}"
    )))
    .fetch_all(pool)
    .await?;
    Ok(update_hazards_core_with_traits(pool, gs, hz_core).await)
}

/// Returns the requested page of hazards alongside the total count of hazards matching
/// `filters` (ignoring pagination), fetched in a single round trip via `COUNT(*) OVER()`.
pub async fn fetch_paginated_hazards(
    pool: &PgPool,
    gs: GameSystem,
    filters: &HazardFieldFilters,
    sort_by: HazardSortEnum,
    order_by: OrderEnum,
    cursor: u32,
    page_size: i16,
) -> Result<(Vec<Hazard>, i64)> {
    let (query, binds) =
        prepare_paginated_get_hazards_listing(gs, filters, sort_by, order_by, cursor, page_size);
    let (hazards, total_count): (Vec<Hazard>, i64) =
        fetch_all_with_binds_and_count(pool, query, binds).await?;
    Ok((
        update_hazards_core_with_traits(pool, gs, hazards).await,
        total_count,
    ))
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

async fn fetch_hazard_resistances(
    pool: &PgPool,
    gs: GameSystem,
    hazard_id: i64,
) -> Result<Vec<Resistance>> {
    Ok(join_all(
        fetch_hazard_resistances_core(pool, gs, hazard_id)
            .await?
            .iter()
            .map(async |x| {
                let (double_vs, exception_vs) = fetch_hazard_resistances_vs(pool, gs, x.id)
                    .await
                    .unwrap_or_default();
                Resistance {
                    core: x.clone(),
                    double_vs,
                    exception_vs,
                }
            }),
    )
    .await)
}

async fn fetch_hazard_resistances_core(
    pool: &PgPool,
    gs: GameSystem,
    hazard_id: i64,
) -> Result<Vec<CoreResistanceData>> {
    Ok(sqlx::query_as(sqlx::AssertSqlSafe(format!(
        "SELECT id, name, value FROM {gs}_hazard_resistance_association_table JOIN
        {gs}_resistance_table ON resistance_id = id WHERE hazard_id = $1"
    )))
    .bind(hazard_id)
    .fetch_all(pool)
    .await?)
}

async fn fetch_hazard_resistances_vs(
    pool: &PgPool,
    gs: GameSystem,
    res_id: i64,
) -> Result<(Vec<String>, Vec<String>)> {
    Ok(sqlx::query_as(sqlx::AssertSqlSafe(format!(
        "SELECT
            ARRAY(SELECT vs_name FROM {gs}_resistance_double_vs_table WHERE resistance_id = $1) AS double_vs,
            ARRAY(SELECT vs_name FROM {gs}_resistance_exception_vs_table WHERE resistance_id = $1) AS exception_vs"
    )))
        .bind(res_id)
        .fetch_one(pool)
        .await?)
}

async fn fetch_hazard_immunities(
    pool: &PgPool,
    gs: GameSystem,
    hazard_id: i64,
) -> Result<Vec<Option<String>>> {
    Ok(sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
        "SELECT name FROM {gs}_immunity_table INTERSECT SELECT immunity_id
         FROM {gs}_hazard_immunity_association_table WHERE hazard_id = $1"
    )))
    .bind(hazard_id)
    .fetch_all(pool)
    .await?)
}

async fn fetch_hazard_weaknesses(
    pool: &PgPool,
    gs: GameSystem,
    hazard_id: i64,
) -> Result<Vec<Weakness>> {
    Ok(sqlx::query_as(sqlx::AssertSqlSafe(format!(
        "SELECT id, name, value FROM {gs}_hazard_weakness_association_table JOIN
         pf_weakness_table ON weakness_id = id WHERE hazard_id = $1"
    )))
    .bind(hazard_id)
    .fetch_all(pool)
    .await?)
}

async fn fetch_hazard_weapons(
    pool: &PgPool,
    gs: GameSystem,
    hazard_id: i64,
) -> Result<Vec<Weapon>> {
    let weapons: Vec<Weapon> = sqlx::query_as(sqlx::AssertSqlSafe(format!(
        "
        SELECT DISTINCT
            wt.id AS weapon_id, wt.to_hit_bonus, wt.splash_dmg, wt.n_of_potency_runes,
            wt.n_of_striking_runes, wt.reload, wt.weapon_type, wt.base_item_id,
            it.*,
            rt.id AS range_id, rt.value AS range_value, rt.increment AS range_increment,
            rt.max AS range_max
        FROM {gs}_weapon_hazard_association_table ica
        LEFT JOIN {gs}_weapon_table wt                      ON wt.id = ica.weapon_id
        LEFT JOIN {gs}_item_table it                        ON it.id = wt.base_item_id
        LEFT JOIN {gs}_weapon_range_association_table wr    ON wr.weapon_id = wt.id
        LEFT JOIN {gs}_range_table rt                       ON rt.id = wr.range_id
        WHERE ica.hazard_id = $1
        ORDER BY name
        "
    )))
    .bind(hazard_id)
    .fetch_all(pool)
    .await?;
    Ok(join_all(weapons.into_iter().map(|mut el| {
        let pool = pool.clone();
        async move {
            el.item_core.traits = fetch_weapon_traits(&pool, gs, el.weapon_data.id)
                .await
                .unwrap_or_default();
            el.item_core.quantity =
                fetch_quantity(&pool, gs, hazard_id, el.weapon_data.id, "weapon")
                    .await
                    .unwrap_or(1);
            el.weapon_data.property_runes = fetch_weapon_runes(&pool, gs, el.weapon_data.id)
                .await
                .unwrap_or_default();
            el.weapon_data.damage_data = fetch_weapon_damage_data(&pool, gs, el.weapon_data.id)
                .await
                .unwrap_or_default();
            el.weapon_data.attack_effects = fetch_weapon_actions(&pool, gs, el.weapon_data.id)
                .await
                .unwrap_or_default();
            el
        }
    }))
    .await)
}

async fn fetch_quantity(
    pool: &PgPool,
    gs: GameSystem,
    hazard_id: i64,
    entity_id: i64,
    entity: &str,
) -> Result<i64> {
    Ok(i64::from(
        sqlx::query_scalar::<_, i32>(sqlx::AssertSqlSafe(format!(
            "SELECT quantity FROM {gs}_{entity}_hazard_association_table WHERE
        hazard_id = $1 AND {entity}_id = $2"
        )))
        .bind(hazard_id)
        .bind(entity_id)
        .fetch_one(pool)
        .await?,
    ))
}
