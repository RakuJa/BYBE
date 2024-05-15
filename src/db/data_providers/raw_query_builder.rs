use crate::models::creature_filter_enum::CreatureFilter;
use std::collections::{HashMap, HashSet};
const ACCURACY_THRESHOLD: i64 = 50;
pub fn prepare_filtered_get_creatures_core(
    key_value_filters: &HashMap<CreatureFilter, HashSet<String>>,
) -> String {
    let mut simple_core_query = String::new();
    let mut trait_query = String::new();
    for (key, value) in key_value_filters {
        if !simple_core_query.is_empty() {
            simple_core_query.push_str(" AND ")
        }
        match key {
            CreatureFilter::Level
            | CreatureFilter::Family
            | CreatureFilter::Size
            | CreatureFilter::Rarity
            | CreatureFilter::Melee
            | CreatureFilter::Ranged
            | CreatureFilter::SpellCaster
            | CreatureFilter::CreatureTypes => simple_core_query
                .push_str(prepare_in_statement(key.to_string().as_str(), value).as_str()),
            CreatureFilter::Traits => trait_query.push_str(prepare_trait_filter(value).as_str()),
            CreatureFilter::CreatureRoles => simple_core_query
                .push_str(prepare_bounded_check(value, 0, ACCURACY_THRESHOLD).as_str()),
        }
    }
    let mut where_query = simple_core_query.to_string();
    if !trait_query.is_empty() {
        where_query.push_str(format!("AND id IN ({trait_query}").as_str());
    };
    if !where_query.is_empty() {
        where_query = format!("WHERE {where_query}")
    }
    format!("SELECT * FROM CREATURE_CORE {where_query} ORDER BY RANDOM() LIMIT 20")
}

/// Prepares a 'bounded AND statement' aka checks if all the columns are in the bound given
/// (brute_percentage >= 0 AND brute_percentage <= 0) AND (sniper_percentage >= 0 ...) ...
fn prepare_bounded_check(
    column_names: &HashSet<String>,
    lower_bound: i64,
    upper_bound: i64,
) -> String {
    let mut bounded_query = String::new();
    if column_names.is_empty() {
        return bounded_query;
    }
    for column in column_names {
        if !bounded_query.is_empty() {
            bounded_query.push_str(" AND ");
        }
        bounded_query.push_str(
            format!("({column} >= {lower_bound} AND {column} <= {upper_bound})").as_str(),
        );
    }
    bounded_query
}

/// Prepares a query that gets all the ids linked with a given list of traits, example
/// SELECT tcat.creature_id
/// FROM TRAIT_CREATURE_ASSOCIATION_TABLE tcat
/// RIGHT JOIN
/// (SELECT * FROM TRAIT_TABLE WHERE name IN ('good')) tt
/// ON tcat.trait_id = tt.name GROUP BY tcat.creature_id
///
fn prepare_trait_filter(column_values: &HashSet<String>) -> String {
    let mut in_string = String::new();
    in_string.push_str(prepare_in_statement("tt.name", column_values).as_str());
    if !in_string.is_empty() {
        let select_query = "SELECT tcat.creature_id FROM TRAIT_CREATURE_ASSOCIATION_TABLE";
        let inner_query = format!("SELECT * FROM TRAIT_TABLE WHERE {in_string}");
        return format!("{select_query} tcat RIGHT JOIN ({inner_query}) tt ON tcat.trait_id = tt.name GROUP BY tcat.creature_id");
    }
    in_string
}

/// Prepares an 'in' statement in the following format
/// 'field in (el1, el2, el3)'
fn prepare_in_statement(column_name: &str, column_values: &HashSet<String>) -> String {
    let mut result_string = String::new();
    if !column_values.is_empty() {
        result_string.push_str(column_name);
        result_string.push_str(" IN (");
        column_values.iter().for_each(|x| {
            result_string.push_str(x);
            result_string.push(',');
        });
        if result_string.ends_with(',') {
            result_string.remove(result_string.len() - 1);
        }
        result_string.push(')')
    }
    result_string
}
