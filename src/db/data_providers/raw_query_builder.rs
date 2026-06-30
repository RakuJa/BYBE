use crate::models::bestiary_structs::{
    BestiaryFilterQuery, CreatureSortEnum, CreatureTableFieldsFilter,
};
use crate::models::creature::creature_field_filter::CreatureFieldFilters;
use crate::models::creature::creature_metadata::creature_role::CreatureRoleEnum;
use crate::models::hazard::hazard_field_filter::{HazardComplexityEnum, HazardFieldFilters};
use crate::models::hazard::hazard_listing_struct::{
    HazardFilterQuery, HazardSortEnum, HazardTableFieldsFilter,
};
use crate::models::item::item_field_filter::ItemFieldFilters;
use crate::models::item::item_metadata::type_enum::ItemTypeEnum;
use crate::models::item::shop_structs::{ItemSortEnum, ItemTableFieldsFilter, ShopFilterQuery};
use crate::models::routers_validator_structs::OrderEnum;
use crate::models::shared::game_system_enum::GameSystem;
use crate::models::shared::pf_version_enum::GameSystemVersionEnum;
use tracing::debug;

pub fn format_pagination_clause(cursor: i64, page_size: i16) -> String {
    if page_size < 0 {
        format!("LIMIT ALL OFFSET {cursor}")
    } else {
        format!("LIMIT {page_size} OFFSET {cursor}")
    }
}

pub fn prepare_filtered_get_items(gs: GameSystem, shop_filter_query: &ShopFilterQuery) -> String {
    let equipment_query = prepare_item_subquery(
        gs,
        &ItemTypeEnum::Equipment,
        shop_filter_query.n_of_equipment,
        &shop_filter_query.item_table_fields_filter,
        shop_filter_query.trait_whitelist_filter.iter(),
        shop_filter_query.trait_blacklist_filter.iter(),
    );
    let consumable_query = prepare_item_subquery(
        gs,
        &ItemTypeEnum::Consumable,
        shop_filter_query.n_of_consumables,
        &shop_filter_query.item_table_fields_filter,
        shop_filter_query.trait_whitelist_filter.iter(),
        shop_filter_query.trait_blacklist_filter.iter(),
    );
    let weapon_query = prepare_item_subquery(
        gs,
        &ItemTypeEnum::Weapon,
        shop_filter_query.n_of_weapons,
        &shop_filter_query.item_table_fields_filter,
        shop_filter_query.trait_whitelist_filter.iter(),
        shop_filter_query.trait_blacklist_filter.iter(),
    );
    let armor_query = prepare_item_subquery(
        gs,
        &ItemTypeEnum::Armor,
        shop_filter_query.n_of_armors,
        &shop_filter_query.item_table_fields_filter,
        shop_filter_query.trait_whitelist_filter.iter(),
        shop_filter_query.trait_blacklist_filter.iter(),
    );
    let shield_query = prepare_item_subquery(
        gs,
        &ItemTypeEnum::Shield,
        shop_filter_query.n_of_shields,
        &shop_filter_query.item_table_fields_filter,
        shop_filter_query.trait_whitelist_filter.iter(),
        shop_filter_query.trait_blacklist_filter.iter(),
    );
    let query = format!(
        "SELECT * FROM {gs}_item_table WHERE status = 'valid' AND id IN ( {equipment_query} ) OR id IN ({consumable_query} )
        OR id IN ({weapon_query} ) OR id IN ({armor_query} ) OR id IN ({shield_query} )"
    );
    debug!("{query}");
    query
}
pub fn prepare_filtered_get_creatures_core(
    gs: GameSystem,
    bestiary_filter_query: &BestiaryFilterQuery,
) -> String {
    let initial_statement = format!("SELECT id FROM {gs}_creature_core");
    let trait_query_tmp = prepare_trait_filter_statement(
        &prepare_creature_trait_filter(gs, bestiary_filter_query.trait_whitelist_filter.iter()),
        &prepare_creature_trait_filter(gs, bestiary_filter_query.trait_blacklist_filter.iter()),
    );
    let trait_query = if trait_query_tmp.is_empty() {
        String::new()
    } else {
        format!("AND {trait_query_tmp}")
    };
    let creature_fields_filter_query =
        prepare_creature_filter_statement(&bestiary_filter_query.creature_table_fields_filter);
    let where_query = format!(
        "{initial_statement} WHERE status = 'valid' AND {creature_fields_filter_query} {trait_query}"
    );
    let query = format!(
        "
    WITH CreatureRankedByLevel AS (
        SELECT *, ROW_NUMBER() OVER (PARTITION BY level ORDER BY RANDOM()) AS rn
        FROM {gs}_creature_core cc WHERE status = 'valid' AND cc.id IN ({where_query})
    )
    SELECT * FROM CreatureRankedByLevel WHERE id IN (
        SELECT id FROM CreatureRankedByLevel WHERE rn>1 ORDER BY RANDOM() LIMIT 20
    )
    UNION ALL
    SELECT * FROM CreatureRankedByLevel WHERE rn=1
    "
    );
    debug!("{query}");
    query
}

pub fn prepare_filtered_get_hazards(
    gs: GameSystem,
    bestiary_filter_query: &HazardFilterQuery,
) -> String {
    let initial_statement = format!("SELECT id FROM {gs}_hazard_table");
    let trait_query_tmp = prepare_trait_filter_statement(
        &prepare_hazard_trait_filter(gs, bestiary_filter_query.trait_whitelist_filter.iter()),
        &prepare_hazard_trait_filter(gs, bestiary_filter_query.trait_blacklist_filter.iter()),
    );
    let trait_query = if trait_query_tmp.is_empty() {
        String::new()
    } else {
        format!("AND {trait_query_tmp}")
    };
    let creature_fields_filter_query =
        prepare_hazard_filter_statement(&bestiary_filter_query.hazard_table_fields_filter);
    let where_query =
        format!("{initial_statement} WHERE {creature_fields_filter_query} {trait_query}");
    let query = format!(
        "
    WITH HazardRankedByLevel AS (
        SELECT *, ROW_NUMBER() OVER (PARTITION BY level ORDER BY RANDOM()) AS rn
        FROM {gs}_hazard_table hz WHERE hz.id IN ({where_query})
    )
    SELECT * FROM HazardRankedByLevel WHERE id IN (
        SELECT id FROM HazardRankedByLevel WHERE rn>1 ORDER BY RANDOM() LIMIT 20
    )
    UNION ALL
    SELECT * FROM HazardRankedByLevel WHERE rn=1
    "
    );
    debug!("{query}");
    query
}

/// Prepares a 'bounded AND statement' aka checks if all columns values are in the bound given, ex
/// ```SQL
/// (brute_percentage >= 0 AND brute_percentage <= 0) AND (sniper_percentage >= 0 ...) ...
/// ```
fn prepare_bounded_and_check<I, S>(column_names: I, lower_bound: i64, upper_bound: i64) -> String
where
    I: Iterator<Item = S>,
    S: ToString,
{
    column_names
        .map(|x| x.to_string())
        .filter(|column| !column.is_empty())
        .map(|column| prepare_bounded_check(column.as_str(), lower_bound, upper_bound))
        .collect::<Vec<_>>()
        .join(" AND ")
}

/// Prepares an inclusive 'bounded statement' aka (x>=lb AND x<=ub)
fn prepare_bounded_check(column: &str, lower_bound: i64, upper_bound: i64) -> String {
    format!("({column} >= {lower_bound} AND {column} <= {upper_bound})")
}

/// Prepares a 'bounded statement' with regard to the optional parameters
///
/// # Arguments
///
/// * `min` - Optional minimum bound (inclusive). If `Some`, adds a `>= min` condition.
/// * `max` - Optional maximum bound (inclusive). If `Some`, adds a `<= max` condition.
/// * `column` - The db column name
///
/// # Returns
///
/// A `String` containing the filter condition:
/// - Both bounds: `"column >= min AND column <= max"`
/// - Min only: `"column >= min"`
/// - Max only: `"column <= max"`
/// - Neither: `""`
///
/// # Examples
///
/// ```Rust
/// assert_eq!(prepare_bounded_check_with_optional_limiters("column", Some(1), Some(10)), "column >= 1 AND column <= 10");
/// assert_eq!(prepare_bounded_check_with_optional_limiters("column", Some(1), None), "column >= 1");
/// assert_eq!(prepare_bounded_check_with_optional_limiters("column", None, Some(10)), "column <= 10");
/// assert_eq!(prepare_bounded_check_with_optional_limiters("column",None, None), "");
/// ```
fn prepare_bounded_check_with_optional_limiters(
    column: &str,
    min: Option<i64>,
    max: Option<i64>,
) -> String {
    match (min, max) {
        (Some(min), Some(max)) => format!("{} >= {} AND {} <= {}", column, min, column, max),
        (Some(min), None) => format!("{} >= {}", column, min),
        (None, Some(max)) => format!("{} <= {}", column, max),
        (None, None) => String::new(),
    }
}

/// Prepares a query that gets all the ids linked with a given list of traits, example
/// ```SQL
/// SELECT tcat.creature_id
/// FROM TRAIT_CREATURE_ASSOCIATION_TABLE tcat
/// RIGHT JOIN
/// (SELECT * FROM TRAIT_TABLE WHERE name IN ('good')) tt
/// ON tcat.trait_id = tt.name GROUP BY tcat.creature_id
///```
fn prepare_trait_filter<I, S>(
    gs: GameSystem,
    id_column: &str,
    association_table_name: &str,
    column_values: I,
) -> String
where
    I: Iterator<Item = S>,
    S: ToString,
{
    let mut in_string = String::new();
    in_string.push_str(prepare_case_insensitive_in_statement("tt.name", column_values).as_str());
    if !in_string.is_empty() {
        let select_query = format!("SELECT tcat.{id_column} FROM {gs}_{association_table_name}");
        let inner_query = format!("SELECT * FROM {gs}_trait_table tt WHERE {in_string}");
        return format!(
            "{select_query} tcat RIGHT JOIN ({inner_query}) jt ON tcat.trait_id = jt.name"
        );
    }
    in_string
}

fn prepare_in_statement_inner<I, S>(
    column_name: &str,
    column_values: I,
    case_insensitive: bool,
) -> String
where
    I: Iterator<Item = S>,
    S: ToString,
{
    let mut result_string = String::new();
    let mut x = column_values.peekable();
    if x.peek().is_some() {
        if case_insensitive {
            result_string.push_str(&format!("UPPER({column_name})"));
        } else {
            result_string.push_str(column_name);
        }
        result_string.push_str(" IN (");
        x.for_each(|v| {
            let s = v.to_string();
            if case_insensitive {
                result_string.push_str(&format!("UPPER('{s}')"));
            } else {
                result_string.push_str(&s);
            }
            result_string.push(',');
        });
        if result_string.ends_with(',') {
            result_string.remove(result_string.len() - 1);
        }
        result_string.push(')');
    }
    result_string
}

/// Prepares a case insensitive 'in' statement in the following format. Requires a string value in db
/// ```SQL
/// "UPPER(field) in (UPPER('el1'), UPPER('el2'), UPPER('el3'))"
/// ```
fn prepare_case_insensitive_in_statement<I, S>(column_name: &str, column_values: I) -> String
where
    I: Iterator<Item = S>,
    S: ToString,
{
    prepare_in_statement_inner(column_name, column_values, true)
}

/// Prepares an 'in' statement in the following format
/// ```SQL
/// 'field in (el1, el2, el3)'
/// ```
fn prepare_in_statement_for_generic_type<I, S>(column_name: &str, column_values: I) -> String
where
    I: Iterator<Item = S>,
    S: ToString,
{
    prepare_in_statement_inner(column_name, column_values, false)
}

fn prepare_item_subquery<I, S>(
    gs: GameSystem,
    item_type: &ItemTypeEnum,
    n_of_item: i64,
    shop_filter_vectors: &ItemTableFieldsFilter,
    trait_whitelist_filter: I,
    trait_blacklist_filter: I,
) -> String
where
    I: Iterator<Item = S>,
    S: ToString,
{
    let item_type_query = prepare_get_id_matching_item_type_query(item_type, gs);
    let initial_statement = format!("SELECT id FROM {gs}_item_table");
    let whitelist_query = prepare_item_trait_filter(gs, trait_whitelist_filter);
    let blacklist_query = prepare_item_trait_filter(gs, trait_blacklist_filter);
    let trait_query_tmp = prepare_trait_filter_statement(&whitelist_query, &blacklist_query);
    let trait_query = if trait_query_tmp.is_empty() {
        String::new()
    } else {
        format!("AND {trait_query_tmp}")
    };
    let item_fields_filter_query = prepare_item_filter_statement(shop_filter_vectors);
    format!(
        "{initial_statement} WHERE {item_fields_filter_query}
         AND id IN ( {item_type_query} ) {trait_query} ORDER BY RANDOM() LIMIT {n_of_item}"
    )
}

fn prepare_item_filter_statement(shop_filter_vectors: &ItemTableFieldsFilter) -> String {
    let remaster_query = prepare_in_statement_for_generic_type(
        "remaster",
        shop_filter_vectors.supported_version.iter(),
    );
    let filters_query = vec![
        prepare_case_insensitive_in_statement("size", shop_filter_vectors.size_filter.iter()),
        prepare_case_insensitive_in_statement("item_type", shop_filter_vectors.type_filter.iter()),
        prepare_case_insensitive_in_statement(
            "category",
            shop_filter_vectors.category_filter.iter(),
        ),
        prepare_case_insensitive_in_statement("rarity", shop_filter_vectors.rarity_filter.iter()),
        prepare_case_insensitive_in_statement("source", shop_filter_vectors.source_filter.iter()),
        prepare_bounded_check(
            &String::from("level"),
            i64::from(shop_filter_vectors.min_level),
            i64::from(shop_filter_vectors.max_level),
        ),
    ]
    .into_iter()
    .filter(|query| !query.is_empty())
    .collect::<Vec<String>>()
    .join(" AND ");
    if filters_query.is_empty() {
        remaster_query
    } else {
        format!("{remaster_query} AND {filters_query}")
    }
}

fn prepare_creature_filter_statement(
    bestiary_filter_vectors: &CreatureTableFieldsFilter,
) -> String {
    let remaster_query = prepare_in_statement_for_generic_type(
        "remaster",
        bestiary_filter_vectors.supported_version.iter(),
    );
    let filters_query = vec![
        prepare_case_insensitive_in_statement(
            "source",
            bestiary_filter_vectors.source_filter.iter(),
        ),
        prepare_case_insensitive_in_statement(
            "family",
            bestiary_filter_vectors.family_filter.iter(),
        ),
        prepare_case_insensitive_in_statement(
            "alignment",
            bestiary_filter_vectors.alignment_filter.iter(),
        ),
        prepare_case_insensitive_in_statement("size", bestiary_filter_vectors.size_filter.iter()),
        prepare_case_insensitive_in_statement(
            "rarity",
            bestiary_filter_vectors.rarity_filter.iter(),
        ),
        prepare_case_insensitive_in_statement(
            "cr_type",
            bestiary_filter_vectors.type_filter.iter(),
        ),
        prepare_in_statement_for_generic_type(
            "is_spellcaster",
            bestiary_filter_vectors.is_spellcaster_filter.iter(),
        ),
        prepare_in_statement_for_generic_type(
            "is_ranged",
            bestiary_filter_vectors.is_ranged_filter.iter(),
        ),
        prepare_in_statement_for_generic_type(
            "is_melee",
            bestiary_filter_vectors.is_melee_filter.iter(),
        ),
        prepare_bounded_and_check(
            bestiary_filter_vectors
                .role_filter
                .iter()
                .map(CreatureRoleEnum::to_db_column),
            i64::from(bestiary_filter_vectors.role_lower_threshold),
            i64::from(bestiary_filter_vectors.role_upper_threshold),
        ),
    ]
    .into_iter()
    .filter(|query| !query.is_empty())
    .collect::<Vec<String>>()
    .join(" AND ");
    if filters_query.is_empty() {
        remaster_query
    } else {
        format!("{remaster_query} AND {filters_query}")
    }
}

fn prepare_hazard_filter_statement(hazard_filter_vec: &HazardTableFieldsFilter) -> String {
    let remaster_query = prepare_in_statement_for_generic_type(
        "remaster",
        hazard_filter_vec.supported_version.iter(),
    );

    let filters_query = vec![
        prepare_case_insensitive_in_statement("source", hazard_filter_vec.source_filter.iter()),
        prepare_case_insensitive_in_statement("size", hazard_filter_vec.size_filter.iter()),
        prepare_case_insensitive_in_statement("rarity", hazard_filter_vec.rarity_filter.iter()),
        prepare_bounded_check_with_optional_limiters(
            "ac",
            hazard_filter_vec.min_ac,
            hazard_filter_vec.max_ac,
        ),
        prepare_bounded_check_with_optional_limiters(
            "hardness",
            hazard_filter_vec.min_hardness,
            hazard_filter_vec.max_hardness,
        ),
        prepare_bounded_check_with_optional_limiters(
            "hp",
            hazard_filter_vec.min_hp,
            hazard_filter_vec.max_hp,
        ),
        prepare_bounded_check_with_optional_limiters(
            "fortitude",
            hazard_filter_vec.min_fortitude,
            hazard_filter_vec.max_fortitude,
        ),
        prepare_bounded_check_with_optional_limiters(
            "reflex",
            hazard_filter_vec.min_reflex,
            hazard_filter_vec.max_reflex,
        ),
        prepare_bounded_check_with_optional_limiters(
            "will",
            hazard_filter_vec.min_will,
            hazard_filter_vec.max_will,
        ),
        prepare_bounded_check_with_optional_limiters(
            "stealth",
            hazard_filter_vec.min_stealth,
            hazard_filter_vec.max_stealth,
        ),
    ]
    .into_iter()
    .filter(|query| !query.is_empty())
    .collect::<Vec<String>>()
    .join(" AND ");
    if filters_query.is_empty() {
        remaster_query
    } else {
        format!("{remaster_query} AND {filters_query}")
    }
}

/// Prepares an 'in' statement, with the following logic
/// ```SQL
/// id NOT IN (bl_id1, bl_id2, bl_idn) AND id IN (wl_id1, wl_id2, wl_idn)
/// ```
fn prepare_trait_filter_statement<S>(whitelist: &S, blacklist: &S) -> String
where
    S: ToString,
{
    let whitelist_query = whitelist.to_string();
    let blacklist_query = blacklist.to_string();
    if whitelist_query.is_empty() && blacklist_query.is_empty() {
        String::new()
    } else if whitelist_query.is_empty() {
        format!("id NOT IN ({blacklist_query})")
    } else if blacklist_query.is_empty() {
        format!("id IN ({whitelist_query})")
    } else {
        format!("id IN ({whitelist_query}) AND id NOT IN ({blacklist_query})")
    }
}

fn prepare_get_id_matching_item_type_query(item_type: &ItemTypeEnum, gs: GameSystem) -> String {
    let (item_id_field, type_query) = match item_type {
        ItemTypeEnum::Consumable | ItemTypeEnum::Equipment => {
            ("id", format!("AND UPPER(item_type) = UPPER('{item_type}')"))
        }
        // There is no need for an and statement here, we already fetch from the "private" table.
        // Item instead contains a lot of item_type (it's the base for weapon/shield/etc)
        ItemTypeEnum::Weapon | ItemTypeEnum::Armor | ItemTypeEnum::Shield => {
            ("base_item_id", String::new())
        }
    };
    let tass_item_id_field = match item_type {
        ItemTypeEnum::Consumable | ItemTypeEnum::Equipment => "item_id",
        ItemTypeEnum::Weapon => "weapon_id",
        ItemTypeEnum::Armor => "armor_id",
        ItemTypeEnum::Shield => "shield_id",
    };
    format!(
        "
        SELECT {item_id_field} FROM {} tmain
        LEFT OUTER JOIN {} tass ON tmain.id = tass.{tass_item_id_field}
        WHERE tass.{tass_item_id_field} IS NULL {type_query}",
        item_type.to_db_main_table_name(gs),
        item_type.to_db_association_table_name(gs),
    )
}

/// Prepares a query that gets all the ids linked with a given list of traits, example
/// ```SQL
/// SELECT tcat.item_id
/// FROM TRAIT_ITEM_ASSOCIATION_TABLE tcat
/// RIGHT JOIN
/// (SELECT * FROM TRAIT_TABLE WHERE name IN ('good')) tt
/// ON tcat.trait_id = tt.name GROUP BY tcat.item_id
///```
fn prepare_item_trait_filter<I, S>(gs: GameSystem, column_values: I) -> String
where
    I: Iterator<Item = S>,
    S: ToString,
{
    prepare_trait_filter(gs, "item_id", "trait_item_association_table", column_values)
}

/// Prepares a query that gets all the ids linked with a given list of traits, example
/// ```SQL
/// SELECT tcat.creature_id
/// FROM TRAIT_CREATURE_ASSOCIATION_TABLE tcat
/// RIGHT JOIN
/// (SELECT * FROM TRAIT_TABLE WHERE name IN ('good')) tt
/// ON tcat.trait_id = tt.name GROUP BY tcat.creature_id
///```
fn prepare_creature_trait_filter<I, S>(gs: GameSystem, column_values: I) -> String
where
    I: Iterator<Item = S>,
    S: ToString,
{
    prepare_trait_filter(
        gs,
        "creature_id",
        "trait_creature_association_table",
        column_values,
    )
}

/// Prepares a query that gets all the ids linked with a given list of traits, example
/// ```SQL
/// SELECT tcat.hazard_id
/// FROM TRAIT_HAZARD_ASSOCIATION_TABLE tcat
/// RIGHT JOIN
/// (SELECT * FROM TRAIT_TABLE WHERE name IN ('good')) tt
/// ON tcat.trait_id = tt.name GROUP BY tcat.hazard_id
///```
fn prepare_hazard_trait_filter<I, S>(gs: GameSystem, column_values: I) -> String
where
    I: Iterator<Item = S>,
    S: ToString,
{
    prepare_trait_filter(
        gs,
        "hazard_id",
        "trait_hazard_association_table",
        column_values,
    )
}

fn escape_sql_str(s: &str) -> String {
    s.replace('\'', "''")
}

const fn order_direction(order_by: OrderEnum) -> &'static str {
    match order_by {
        OrderEnum::Ascending => "ASC",
        OrderEnum::Descending => "DESC",
    }
}

fn prepare_creature_listing_where(gs: GameSystem, filters: &CreatureFieldFilters) -> String {
    let mut conditions = vec!["status = 'valid'".to_string()];

    if let Some(name) = &filters.name_filter {
        conditions.push(format!("name ILIKE '%{}%'", escape_sql_str(name)));
    }
    if let Some(sources) = &filters.source_filter {
        let s = prepare_case_insensitive_in_statement("source", sources.iter());
        if !s.is_empty() {
            conditions.push(s);
        }
    }
    if let Some(families) = &filters.family_filter
        && !families.is_empty()
    {
        let parts: Vec<String> = families
            .iter()
            .map(|f| format!("family ILIKE '%{}%'", escape_sql_str(f)))
            .collect();
        conditions.push(format!("({})", parts.join(" OR ")));
    }
    if let Some(rarities) = &filters.rarity_filter {
        let s = prepare_case_insensitive_in_statement("rarity", rarities.iter());
        if !s.is_empty() {
            conditions.push(s);
        }
    }
    if let Some(sizes) = &filters.size_filter {
        let s = prepare_case_insensitive_in_statement("size", sizes.iter());
        if !s.is_empty() {
            conditions.push(s);
        }
    }
    if let Some(alignments) = &filters.alignment_filter {
        let s = prepare_case_insensitive_in_statement("alignment", alignments.iter());
        if !s.is_empty() {
            conditions.push(s);
        }
    }
    if let Some(types) = &filters.type_filter {
        let s = prepare_case_insensitive_in_statement("cr_type", types.iter());
        if !s.is_empty() {
            conditions.push(s);
        }
    }
    if let Some(v) = filters.min_hp_filter {
        conditions.push(format!("hp >= {v}"));
    }
    if let Some(v) = filters.max_hp_filter {
        conditions.push(format!("hp <= {v}"));
    }
    if let Some(v) = filters.min_level_filter {
        conditions.push(format!("level >= {v}"));
    }
    if let Some(v) = filters.max_level_filter {
        conditions.push(format!("level <= {v}"));
    }
    if let Some(attacks) = &filters.attack_data_filter {
        for (attack, has_attack) in attacks {
            if let Some(has) = has_attack {
                let col = match attack.as_str() {
                    "melee" => "is_melee",
                    "ranged" => "is_ranged",
                    "spellcaster" => "is_spellcaster",
                    _ => continue,
                };
                conditions.push(format!("{col} = {has}"));
            }
        }
    }
    if let Some(roles) = &filters.role_filter
        && !roles.is_empty()
    {
        let threshold = filters.role_threshold.unwrap_or(0);
        let parts: Vec<String> = roles
            .iter()
            .map(|r| format!("{} >= {threshold}", r.to_db_column()))
            .collect();
        conditions.push(format!("({})", parts.join(" OR ")));
    }
    match filters.game_system_version.unwrap_or_default() {
        GameSystemVersionEnum::Legacy => conditions.push("remaster = false".to_string()),
        GameSystemVersionEnum::Remaster => conditions.push("remaster = true".to_string()),
        GameSystemVersionEnum::Any => {}
    }
    if let Some(whitelist) = &filters.trait_whitelist_filter
        && !whitelist.is_empty()
    {
        let parts: Vec<String> = whitelist
            .iter()
            .map(|t| format!("UPPER(trait_id) LIKE UPPER('%{}%')", escape_sql_str(t)))
            .collect();
        conditions.push(format!(
            "id IN (SELECT creature_id FROM {gs}_trait_creature_association_table WHERE {})",
            parts.join(" OR ")
        ));
    }
    if let Some(blacklist) = &filters.trait_blacklist_filter
        && !blacklist.is_empty()
    {
        let parts: Vec<String> = blacklist
            .iter()
            .map(|t| format!("UPPER(trait_id) = UPPER('{}')", escape_sql_str(t)))
            .collect();
        conditions.push(format!(
            "id NOT IN (SELECT creature_id FROM {gs}_trait_creature_association_table WHERE {})",
            parts.join(" OR ")
        ));
    }
    conditions.join(" AND ")
}

fn prepare_creature_listing_order(
    gs: GameSystem,
    sort_by: CreatureSortEnum,
    order_by: OrderEnum,
) -> String {
    let dir = order_direction(order_by);
    match sort_by {
        CreatureSortEnum::Id => format!("id {dir}"),
        CreatureSortEnum::Name => format!("name {dir}"),
        CreatureSortEnum::Level => format!("level {dir}"),
        CreatureSortEnum::Trait => format!(
            "(SELECT STRING_AGG(trait_id, ', ' ORDER BY trait_id) \
             FROM {gs}_trait_creature_association_table WHERE creature_id = id) {dir} NULLS LAST"
        ),
        CreatureSortEnum::Size => format!("size {dir}"),
        CreatureSortEnum::Type => format!("cr_type {dir}"),
        CreatureSortEnum::Hp => format!("hp {dir}"),
        CreatureSortEnum::Rarity => format!("rarity {dir}"),
        CreatureSortEnum::Family => format!("family {dir}"),
        CreatureSortEnum::Alignment => format!("alignment {dir}"),
        CreatureSortEnum::Attack => {
            format!("is_melee {dir}, is_ranged {dir}, is_spellcaster {dir}")
        }
        CreatureSortEnum::Role => format!(
            "GREATEST(brute_percentage, magical_striker_percentage, skill_paragon_percentage, \
             skirmisher_percentage, sniper_percentage, soldier_percentage, spellcaster_percentage) {dir}"
        ),
    }
}

pub fn prepare_paginated_get_creatures_listing(
    gs: GameSystem,
    filters: &CreatureFieldFilters,
    sort_by: CreatureSortEnum,
    order_by: OrderEnum,
    cursor: u32,
    page_size: i16,
) -> String {
    let where_clause = prepare_creature_listing_where(gs, filters);
    let order_clause = prepare_creature_listing_order(gs, sort_by, order_by);
    let pagination = format_pagination_clause(i64::from(cursor), page_size);
    format!(
        "SELECT * FROM {gs}_creature_core WHERE {where_clause} ORDER BY {order_clause} {pagination}"
    )
}

pub fn prepare_count_creatures_listing(gs: GameSystem, filters: &CreatureFieldFilters) -> String {
    let where_clause = prepare_creature_listing_where(gs, filters);
    format!("SELECT COUNT(*) FROM {gs}_creature_core WHERE {where_clause}")
}

fn prepare_hazard_listing_where(gs: GameSystem, filters: &HazardFieldFilters) -> String {
    let mut conditions: Vec<String> = vec![];

    if let Some(name) = &filters.name_filter {
        conditions.push(format!("name ILIKE '%{}%'", escape_sql_str(name)));
    }
    if let Some(sources) = &filters.source_filter {
        let s = prepare_case_insensitive_in_statement("source", sources.iter());
        if !s.is_empty() {
            conditions.push(s);
        }
    }
    if let Some(rarities) = &filters.rarity_filter {
        let s = prepare_case_insensitive_in_statement("rarity", rarities.iter());
        if !s.is_empty() {
            conditions.push(s);
        }
    }
    if let Some(sizes) = &filters.size_filter {
        let s = prepare_case_insensitive_in_statement("size", sizes.iter());
        if !s.is_empty() {
            conditions.push(s);
        }
    }
    match filters.complexity_filter.unwrap_or_default() {
        HazardComplexityEnum::Simple => conditions.push("is_complex = false".to_string()),
        HazardComplexityEnum::Complex => conditions.push("is_complex = true".to_string()),
        HazardComplexityEnum::Any => {}
    }
    if let Some(v) = filters.min_ac_filter {
        conditions.push(format!("ac >= {v}"));
    }
    if let Some(v) = filters.max_ac_filter {
        conditions.push(format!("ac <= {v}"));
    }
    if let Some(v) = filters.min_hardness_filter {
        conditions.push(format!("hardness >= {v}"));
    }
    if let Some(v) = filters.max_hardness_filter {
        conditions.push(format!("hardness <= {v}"));
    }
    if let Some(v) = filters.min_hp_filter {
        conditions.push(format!("hp >= {v}"));
    }
    if let Some(v) = filters.max_hp_filter {
        conditions.push(format!("hp <= {v}"));
    }
    if let Some(v) = filters.min_level_filter {
        conditions.push(format!("level >= {v}"));
    }
    if let Some(v) = filters.max_level_filter {
        conditions.push(format!("level <= {v}"));
    }
    if let Some(v) = filters.min_stealth_filter {
        conditions.push(format!("stealth >= {v}"));
    }
    if let Some(v) = filters.max_stealth_filter {
        conditions.push(format!("stealth <= {v}"));
    }
    // NULL passes bounds for optional columns
    if let Some(v) = filters.min_will_filter {
        conditions.push(format!("(will IS NULL OR will >= {v})"));
    }
    if let Some(v) = filters.max_will_filter {
        conditions.push(format!("(will IS NULL OR will <= {v})"));
    }
    if let Some(v) = filters.min_reflex_filter {
        conditions.push(format!("(reflex IS NULL OR reflex >= {v})"));
    }
    if let Some(v) = filters.max_reflex_filter {
        conditions.push(format!("(reflex IS NULL OR reflex <= {v})"));
    }
    if let Some(v) = filters.min_fortitude_filter {
        conditions.push(format!("(fortitude IS NULL OR fortitude >= {v})"));
    }
    if let Some(v) = filters.max_fortitude_filter {
        conditions.push(format!("(fortitude IS NULL OR fortitude <= {v})"));
    }
    match filters.game_system_version.unwrap_or_default() {
        GameSystemVersionEnum::Legacy => conditions.push("remaster = false".to_string()),
        GameSystemVersionEnum::Remaster => conditions.push("remaster = true".to_string()),
        GameSystemVersionEnum::Any => {}
    }
    if let Some(whitelist) = &filters.trait_whitelist_filter
        && !whitelist.is_empty()
    {
        let parts: Vec<String> = whitelist
            .iter()
            .map(|t| format!("UPPER(trait_id) LIKE UPPER('%{}%')", escape_sql_str(t)))
            .collect();
        conditions.push(format!(
            "id IN (SELECT hazard_id FROM {gs}_trait_hazard_association_table WHERE {})",
            parts.join(" OR ")
        ));
    }
    if let Some(blacklist) = &filters.trait_blacklist_filter
        && !blacklist.is_empty()
    {
        let parts: Vec<String> = blacklist
            .iter()
            .map(|t| format!("UPPER(trait_id) = UPPER('{}')", escape_sql_str(t)))
            .collect();
        conditions.push(format!(
            "id NOT IN (SELECT hazard_id FROM {gs}_trait_hazard_association_table WHERE {})",
            parts.join(" OR ")
        ));
    }
    if conditions.is_empty() {
        "TRUE".to_string()
    } else {
        conditions.join(" AND ")
    }
}

fn prepare_hazard_listing_order(
    gs: GameSystem,
    sort_by: HazardSortEnum,
    order_by: OrderEnum,
) -> String {
    let dir = order_direction(order_by);
    match sort_by {
        HazardSortEnum::Id => format!("id {dir}"),
        HazardSortEnum::Name => format!("name {dir}"),
        HazardSortEnum::Ac => format!("ac {dir}"),
        HazardSortEnum::Hardness => format!("hardness {dir}"),
        HazardSortEnum::Hp => format!("hp {dir}"),
        HazardSortEnum::Complexity => format!("is_complex {dir}"),
        HazardSortEnum::Level => format!("level {dir}"),
        HazardSortEnum::Trait => format!(
            "(SELECT STRING_AGG(trait_id, ', ' ORDER BY trait_id) \
             FROM {gs}_trait_hazard_association_table WHERE hazard_id = id) {dir} NULLS LAST"
        ),
        HazardSortEnum::Rarity => format!("rarity {dir}"),
        HazardSortEnum::Size => format!("size {dir}"),
        HazardSortEnum::Source => format!("source {dir}"),
        HazardSortEnum::Fortitude => format!("fortitude {dir} NULLS LAST"),
        HazardSortEnum::Reflex => format!("reflex {dir} NULLS LAST"),
        HazardSortEnum::Will => format!("will {dir} NULLS LAST"),
        HazardSortEnum::Stealth => format!("stealth {dir}"),
    }
}

pub fn prepare_paginated_get_hazards_listing(
    gs: GameSystem,
    filters: &HazardFieldFilters,
    sort_by: HazardSortEnum,
    order_by: OrderEnum,
    cursor: u32,
    page_size: i16,
) -> String {
    let where_clause = prepare_hazard_listing_where(gs, filters);
    let order_clause = prepare_hazard_listing_order(gs, sort_by, order_by);
    let pagination = format_pagination_clause(i64::from(cursor), page_size);
    format!(
        "SELECT * FROM {gs}_hazard_table WHERE {where_clause} ORDER BY {order_clause} {pagination}"
    )
}

pub fn prepare_count_hazards_listing(gs: GameSystem, filters: &HazardFieldFilters) -> String {
    let where_clause = prepare_hazard_listing_where(gs, filters);
    format!("SELECT COUNT(*) FROM {gs}_hazard_table WHERE {where_clause}")
}

fn prepare_item_listing_where(gs: GameSystem, filters: &ItemFieldFilters) -> String {
    let mut conditions = vec![
        "is_derived = false".to_string(),
        "status = 'valid'".to_string(),
    ];

    if let Some(name) = &filters.name_filter {
        conditions.push(format!("name ILIKE '%{}%'", escape_sql_str(name)));
    }
    if let Some(categories) = &filters.category_filter
        && !categories.is_empty()
    {
        let parts: Vec<String> = categories
            .iter()
            .map(|c| format!("COALESCE(category, '') ILIKE '%{}%'", escape_sql_str(c)))
            .collect();
        conditions.push(format!("({})", parts.join(" OR ")));
    }
    if let Some(sources) = &filters.source_filter
        && !sources.is_empty()
    {
        let parts: Vec<String> = sources
            .iter()
            .map(|s| format!("source ILIKE '%{}%'", escape_sql_str(s)))
            .collect();
        conditions.push(format!("({})", parts.join(" OR ")));
    }
    if let Some(rarities) = &filters.rarity_filter {
        let s = prepare_case_insensitive_in_statement("rarity", rarities.iter());
        if !s.is_empty() {
            conditions.push(s);
        }
    }
    if let Some(sizes) = &filters.size_filter {
        let s = prepare_case_insensitive_in_statement("size", sizes.iter());
        if !s.is_empty() {
            conditions.push(s);
        }
    }
    if let Some(types) = &filters.type_filter {
        let s = prepare_case_insensitive_in_statement("item_type", types.iter());
        if !s.is_empty() {
            conditions.push(s);
        }
    }
    if let Some(v) = filters.min_hp_filter {
        conditions.push(format!("hp >= {v}"));
    }
    if let Some(v) = filters.max_hp_filter {
        conditions.push(format!("hp <= {v}"));
    }
    if let Some(v) = filters.min_level_filter {
        conditions.push(format!("level >= {v}"));
    }
    if let Some(v) = filters.max_level_filter {
        conditions.push(format!("level <= {v}"));
    }
    if let Some(v) = filters.min_price_filter {
        conditions.push(format!("price >= {v}"));
    }
    if let Some(v) = filters.max_price_filter {
        conditions.push(format!("price <= {v}"));
    }
    if let Some(v) = filters.min_hardness_filter {
        conditions.push(format!("hardness >= {v}"));
    }
    if let Some(v) = filters.max_hardness_filter {
        conditions.push(format!("hardness <= {v}"));
    }
    if let Some(v) = filters.min_bulk_filter {
        conditions.push(format!("bulk >= {v}"));
    }
    if let Some(v) = filters.max_bulk_filter {
        conditions.push(format!("bulk <= {v}"));
    }
    // min uses requires NOT NULL; max uses allows NULL (item without uses passes)
    if let Some(v) = filters.min_n_of_uses_filter {
        conditions.push(format!(
            "(number_of_uses IS NOT NULL AND number_of_uses >= {v})"
        ));
    }
    if let Some(v) = filters.max_n_of_uses_filter {
        conditions.push(format!("(number_of_uses IS NULL OR number_of_uses <= {v})"));
    }
    match filters.game_system_version.unwrap_or_default() {
        GameSystemVersionEnum::Legacy => conditions.push("remaster = false".to_string()),
        GameSystemVersionEnum::Remaster => conditions.push("remaster = true".to_string()),
        GameSystemVersionEnum::Any => {}
    }
    if let Some(whitelist) = &filters.trait_whitelist_filter
        && !whitelist.is_empty()
    {
        let parts: Vec<String> = whitelist
            .iter()
            .map(|t| format!("UPPER(trait_id) LIKE UPPER('%{}%')", escape_sql_str(t)))
            .collect();
        conditions.push(format!(
            "id IN (SELECT item_id FROM {gs}_trait_item_association_table WHERE {})",
            parts.join(" OR ")
        ));
    }
    if let Some(blacklist) = &filters.trait_blacklist_filter
        && !blacklist.is_empty()
    {
        let parts: Vec<String> = blacklist
            .iter()
            .map(|t| format!("UPPER(trait_id) = UPPER('{}')", escape_sql_str(t)))
            .collect();
        conditions.push(format!(
            "id NOT IN (SELECT item_id FROM {gs}_trait_item_association_table WHERE {})",
            parts.join(" OR ")
        ));
    }
    conditions.join(" AND ")
}

fn prepare_item_listing_order(
    gs: GameSystem,
    sort_by: ItemSortEnum,
    order_by: OrderEnum,
) -> String {
    let dir = order_direction(order_by);
    match sort_by {
        ItemSortEnum::Id => format!("id {dir}"),
        ItemSortEnum::Name => format!("name {dir}"),
        ItemSortEnum::Level => format!("level {dir}"),
        ItemSortEnum::Trait => format!(
            "(SELECT STRING_AGG(trait_id, ', ' ORDER BY trait_id) \
             FROM {gs}_trait_item_association_table WHERE item_id = id) {dir} NULLS LAST"
        ),
        ItemSortEnum::Type => format!("item_type {dir}"),
        ItemSortEnum::Rarity => format!("rarity {dir}"),
        ItemSortEnum::Source => format!("source {dir}"),
    }
}

pub fn prepare_paginated_get_items_listing(
    gs: GameSystem,
    filters: &ItemFieldFilters,
    sort_by: ItemSortEnum,
    order_by: OrderEnum,
    cursor: u32,
    page_size: i16,
) -> String {
    let where_clause = prepare_item_listing_where(gs, filters);
    let order_clause = prepare_item_listing_order(gs, sort_by, order_by);
    let pagination = format_pagination_clause(i64::from(cursor), page_size);
    format!(
        "SELECT * FROM {gs}_item_table WHERE {where_clause} ORDER BY {order_clause} {pagination}"
    )
}

pub fn prepare_count_items_listing(gs: GameSystem, filters: &ItemFieldFilters) -> String {
    let where_clause = prepare_item_listing_where(gs, filters);
    format!("SELECT COUNT(*) FROM {gs}_item_table WHERE {where_clause}")
}
