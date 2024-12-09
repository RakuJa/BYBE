use crate::models::bestiary_structs::{BestiaryFilterQuery, CreatureTableFieldsFilter};
use crate::models::creature::creature_metadata::alignment_enum::AlignmentEnum;
use crate::models::creature::creature_metadata::creature_role::CreatureRoleEnum;
use crate::models::item::item_metadata::type_enum::ItemTypeEnum;
use crate::models::shop_structs::{ItemTableFieldsFilter, ShopFilterQuery};
use log::debug;

pub fn prepare_filtered_get_items(shop_filter_query: &ShopFilterQuery) -> String {
    let equipment_query = prepare_item_subquery(
        &ItemTypeEnum::Equipment,
        shop_filter_query.n_of_equipment,
        &shop_filter_query.item_table_fields_filter,
        shop_filter_query.trait_whitelist_filter.iter(),
        shop_filter_query.trait_blacklist_filter.iter(),
    );
    let consumable_query = prepare_item_subquery(
        &ItemTypeEnum::Consumable,
        shop_filter_query.n_of_consumables,
        &shop_filter_query.item_table_fields_filter,
        shop_filter_query.trait_whitelist_filter.iter(),
        shop_filter_query.trait_blacklist_filter.iter(),
    );
    let weapon_query = prepare_item_subquery(
        &ItemTypeEnum::Weapon,
        shop_filter_query.n_of_weapons,
        &shop_filter_query.item_table_fields_filter,
        shop_filter_query.trait_whitelist_filter.iter(),
        shop_filter_query.trait_blacklist_filter.iter(),
    );
    let armor_query = prepare_item_subquery(
        &ItemTypeEnum::Armor,
        shop_filter_query.n_of_armors,
        &shop_filter_query.item_table_fields_filter,
        shop_filter_query.trait_whitelist_filter.iter(),
        shop_filter_query.trait_blacklist_filter.iter(),
    );
    let shield_query = prepare_item_subquery(
        &ItemTypeEnum::Shield,
        shop_filter_query.n_of_shields,
        &shop_filter_query.item_table_fields_filter,
        shop_filter_query.trait_whitelist_filter.iter(),
        shop_filter_query.trait_blacklist_filter.iter(),
    );
    let query = format!(
        "SELECT * FROM ITEM_TABLE WHERE id IN ( {equipment_query} ) OR id IN ({consumable_query} )
        OR id IN ({weapon_query} ) OR id IN ({armor_query} ) OR id IN ({shield_query} )"
    );
    debug!("{}", query);
    query
}
pub fn prepare_filtered_get_creatures_core(bestiary_filter_query: &BestiaryFilterQuery) -> String {
    let initial_statement = "SELECT id FROM CREATURE_CORE";
    let trait_query_tmp = prepare_trait_filter_statement(
        &prepare_creature_trait_filter(bestiary_filter_query.trait_whitelist_filter.iter()),
        &prepare_creature_trait_filter(bestiary_filter_query.trait_blacklist_filter.iter()),
    );
    let trait_query = if trait_query_tmp.is_empty() {
        String::new()
    } else {
        format!("AND {trait_query_tmp}")
    };
    let creature_fields_filter_query =
        prepare_creature_filter_statement(&bestiary_filter_query.creature_table_fields_filter);
    let where_query =
        format!("{initial_statement} WHERE {creature_fields_filter_query} {trait_query}");
    let query = format!(
        "
    WITH CreatureRankedByLevel AS (
        SELECT *, ROW_NUMBER() OVER (PARTITION BY level ORDER BY RANDOM()) AS rn
        FROM CREATURE_CORE cc WHERE cc.id IN ({where_query})
    )
    SELECT * FROM CreatureRankedByLevel WHERE id IN (
        SELECT id FROM CreatureRankedByLevel WHERE rn>1 ORDER BY RANDOM() LIMIT 20
    )
    UNION ALL
    SELECT * FROM CreatureRankedByLevel WHERE rn=1
    "
    );
    debug!("{}", query);
    query
}

/// Prepares a 'bounded OR statement' aka checks if all columns values are in the bound given, ex
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

/// Prepares a 'bounded statement' aka (x>=lb AND x<=ub)
fn prepare_bounded_check(column: &str, lower_bound: i64, upper_bound: i64) -> String {
    format!("({column} >= {lower_bound} AND {column} <= {upper_bound})")
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
        let select_query = format!("SELECT tcat.{id_column} FROM {association_table_name}");
        let inner_query = format!("SELECT * FROM TRAIT_TABLE tt WHERE {in_string}");
        return format!(
            "{select_query} tcat RIGHT JOIN ({inner_query}) jt ON tcat.trait_id = jt.name"
        );
    }
    in_string
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
    let mut result_string = String::new();
    let mut x = column_values.peekable();
    if x.peek().is_some() {
        result_string.push_str(format!("UPPER({column_name})").as_str());
        result_string.push_str(" IN (");
        x.for_each(|x| {
            let str = x.to_string();
            result_string.push_str(format!("UPPER('{str}')").as_str());
            result_string.push(',');
        });
        if result_string.ends_with(',') {
            result_string.remove(result_string.len() - 1);
        }
        result_string.push(')');
    }
    result_string
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
    let mut result_string = String::new();
    let mut x = column_values.peekable();
    if x.peek().is_some() {
        result_string.push_str(column_name);
        result_string.push_str(" IN (");
        x.for_each(|x| {
            result_string.push_str(x.to_string().as_str());
            result_string.push(',');
        });
        if result_string.ends_with(',') {
            result_string.remove(result_string.len() - 1);
        }
        result_string.push(')');
    }
    result_string
}

fn prepare_item_subquery<I, S>(
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
    let item_type_query = prepare_get_id_matching_item_type_query(item_type);
    let initial_statement = "SELECT id FROM ITEM_TABLE";
    let whitelist_query = prepare_item_trait_filter(trait_whitelist_filter);
    let blacklist_query = prepare_item_trait_filter(trait_blacklist_filter);
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
            bestiary_filter_vectors
                .alignment_filter
                .iter()
                .flat_map(AlignmentEnum::to_db_traits),
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

fn prepare_get_id_matching_item_type_query(item_type: &ItemTypeEnum) -> String {
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
        item_type.to_db_main_table_name(),
        item_type.to_db_association_table_name(),
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
fn prepare_item_trait_filter<I, S>(column_values: I) -> String
where
    I: Iterator<Item = S>,
    S: ToString,
{
    prepare_trait_filter("item_id", "TRAIT_ITEM_ASSOCIATION_TABLE", column_values)
}

/// Prepares a query that gets all the ids linked with a given list of traits, example
/// ```SQL
/// SELECT tcat.creature_id
/// FROM TRAIT_CREATURE_ASSOCIATION_TABLE tcat
/// RIGHT JOIN
/// (SELECT * FROM TRAIT_TABLE WHERE name IN ('good')) tt
/// ON tcat.trait_id = tt.name GROUP BY tcat.creature_id
///```
fn prepare_creature_trait_filter<I, S>(column_values: I) -> String
where
    I: Iterator<Item = S>,
    S: ToString,
{
    prepare_trait_filter(
        "creature_id",
        "TRAIT_CREATURE_ASSOCIATION_TABLE",
        column_values,
    )
}
