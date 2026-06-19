use crate::models::item::weapon_struct::DamageData;
use crate::models::shared::action::{Action, CoreAction};
use crate::models::shared::alignment_enum::ALIGNMENT_TRAITS;
use crate::models::shared::game_system_enum::GameSystem;
use crate::models::shared::trait_data::TraitData;
use crate::traits::traits_enrichable::TraitsEnrichable;
use anyhow::Result;
use sqlx::PgPool;

/// Fetches traits for any entity using the shared `{gs}_trait_{entity}_association_table` convention.
pub(crate) async fn fetch_entity_traits(
    pool: &PgPool,
    gs: GameSystem,
    entity: &str,
    entity_id: i64,
) -> Result<Vec<TraitData>> {
    Ok(sqlx::query_as(sqlx::AssertSqlSafe(format!(
        "SELECT t.name, t.description, t.display_name FROM {gs}_trait_table t JOIN
        {gs}_trait_{entity}_association_table a ON a.trait_id = t.name
        WHERE a.{entity}_id = $1 ORDER BY t.name"
    )))
    .bind(entity_id)
    .fetch_all(pool)
    .await?)
}

pub async fn fetch_item_traits(
    pool: &PgPool,
    gs: GameSystem,
    item_id: i64,
) -> Result<Vec<TraitData>> {
    fetch_entity_traits(pool, gs, "item", item_id).await
}

pub async fn fetch_weapon_traits(
    pool: &PgPool,
    gs: GameSystem,
    weapon_id: i64,
) -> Result<Vec<TraitData>> {
    fetch_entity_traits(pool, gs, "weapon", weapon_id).await
}

pub async fn fetch_shield_traits(
    pool: &PgPool,
    gs: GameSystem,
    shield_id: i64,
) -> Result<Vec<TraitData>> {
    fetch_entity_traits(pool, gs, "shield", shield_id).await
}

pub async fn fetch_armor_traits(
    pool: &PgPool,
    gs: GameSystem,
    armor_id: i64,
) -> Result<Vec<TraitData>> {
    fetch_entity_traits(pool, gs, "armor", armor_id).await
}

pub async fn fetch_weapon_runes(pool: &PgPool, gs: GameSystem, wp_id: i64) -> Result<Vec<String>> {
    Ok(sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
        "SELECT name FROM {gs}_rune_table INTERSECT
         SELECT rune_id FROM {gs}_rune_weapon_association_table WHERE weapon_id = $1
         ORDER BY name"
    )))
    .bind(wp_id)
    .fetch_all(pool)
    .await?)
}

pub async fn fetch_weapon_damage_data(
    pool: &PgPool,
    gs: GameSystem,
    wp_id: i64,
) -> Result<Vec<DamageData>> {
    Ok(sqlx::query_as(sqlx::AssertSqlSafe(format!(
        "SELECT id, bonus_dmg, dmg_type, number_of_dice, die_size
             FROM {gs}_weapon_damage_table dm RIGHT JOIN (
             SELECT id AS wp_id FROM {gs}_weapon_table WHERE id = $1
             ) wt ON wt.wp_id = dm.weapon_id",
    )))
    .bind(wp_id)
    .fetch_all(pool)
    .await?)
}

pub async fn fetch_weapon_actions(
    pool: &PgPool,
    gs: GameSystem,
    wp_id: i64,
) -> Result<Vec<Action>> {
    let core_actions = sqlx::query_as(sqlx::AssertSqlSafe(format!(
        "
        SELECT a.* FROM {gs}_action_table AS a
        JOIN {gs}_weapon_action_association_table AS wa ON wa.action_id = a.id
        WHERE wa.weapon_id = $1"
    )))
    .bind(wp_id)
    .fetch_all(pool)
    .await?;
    fetch_actions_from_cores(pool, gs, core_actions).await
}

pub async fn fetch_actions_from_cores(
    pool: &PgPool,
    gs: GameSystem,
    core_actions: Vec<CoreAction>,
) -> Result<Vec<Action>> {
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

pub async fn fetch_armor_runes(pool: &PgPool, gs: GameSystem, wp_id: i64) -> Result<Vec<String>> {
    Ok(sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
        "SELECT name FROM {gs}_rune_table INTERSECT
         SELECT rune_id FROM {gs}_rune_armor_association_table WHERE armor_id = $1
         ORDER BY name"
    )))
    .bind(wp_id)
    .fetch_all(pool)
    .await?)
}

pub async fn fetch_action_traits(
    pool: &PgPool,
    gs: GameSystem,
    action_id: i64,
) -> Result<Vec<TraitData>> {
    Ok(sqlx::query_as(sqlx::AssertSqlSafe(format!(
        "SELECT tt.name, tt.description, tt.display_name
        FROM {gs}_trait_action_association_table tcat
            LEFT JOIN {gs}_trait_table tt ON tcat.trait_id = tt.name WHERE action_id = $1 GROUP BY tt.name",
    ))).bind(action_id)
        .fetch_all(pool)
        .await?)
}

pub async fn fetch_unique_values_of_field(
    pool: &PgPool,
    table: &str,
    field: &str,
) -> Result<Vec<String>> {
    let query = format!(
        "SELECT CAST(t1.{field} AS TEXT) FROM ((SELECT DISTINCT ({field}) FROM {table})) t1"
    );
    Ok(sqlx::query_scalar(sqlx::AssertSqlSafe(query))
        .fetch_all(pool)
        .await?)
}

/// Fetches MIN and MAX of an integer column.
/// `from_clause` is the `FROM table WHERE ...` portion of the SQL.
pub async fn fetch_col_range(pool: &PgPool, column: &str, from_clause: &str) -> Result<(i64, i64)> {
    let min: i64 = sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
        "SELECT COALESCE(MIN({column}), 0)::bigint {from_clause}"
    )))
    .fetch_one(pool)
    .await?;
    let max: i64 = sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
        "SELECT COALESCE(MAX({column}), 0)::bigint {from_clause}"
    )))
    .fetch_one(pool)
    .await?;
    Ok((min, max))
}

pub async fn fetch_col_range_f64(
    pool: &PgPool,
    column: &str,
    from_clause: &str,
) -> Result<(f64, f64)> {
    let min: f64 = sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
        "SELECT COALESCE(MIN({column}), 0.0) {from_clause}"
    )))
    .fetch_one(pool)
    .await?;
    let max: f64 = sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
        "SELECT COALESCE(MAX({column}), 0.0) {from_clause}"
    )))
    .fetch_one(pool)
    .await?;
    Ok((min, max))
}

/// Enriches each entity in `entities` with the linked traits fetched from the DB.
/// When `filter_alignment` is true, removes alignment traits (e.g. Good, Evil, Chaotic, Lawful).
pub async fn enrich_with_traits<T: TraitsEnrichable>(
    pool: &PgPool,
    gs: GameSystem,
    mut entities: Vec<T>,
    filter_alignment: bool,
) -> Vec<T> {
    for item in &mut entities {
        let id = item.entity_id();
        let traits = fetch_entity_traits(pool, gs, T::entity_name(), id)
            .await
            .unwrap_or_default();
        let traits = if filter_alignment {
            traits
                .into_iter()
                .filter(|x| !ALIGNMENT_TRAITS.contains(&&*x.name.as_str().to_uppercase()))
                .collect()
        } else {
            traits
        };
        item.set_traits(traits);
    }
    entities
}
