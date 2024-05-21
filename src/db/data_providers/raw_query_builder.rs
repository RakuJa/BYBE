use crate::models::creature::creature_filter_enum::CreatureFilter;
use log::debug;
use std::collections::{HashMap, HashSet};

const ACCURACY_THRESHOLD: i64 = 50;
pub fn prepare_filtered_get_creatures_core(
    key_value_filters: &HashMap<CreatureFilter, HashSet<String>>,
) -> String {
    let mut simple_core_query = String::new();
    let mut trait_query = String::new();
    for (key, value) in key_value_filters {
        match key {
            CreatureFilter::Level
            | CreatureFilter::Melee
            | CreatureFilter::Ranged
            | CreatureFilter::SpellCaster => {
                if !simple_core_query.is_empty() {
                    simple_core_query.push_str(" AND ")
                }
                simple_core_query.push_str(
                    prepare_in_statement_for_generic_type(key.to_string().as_str(), value).as_str(),
                )
            }
            CreatureFilter::Family
            | CreatureFilter::Alignment
            | CreatureFilter::Size
            | CreatureFilter::Rarity
            | CreatureFilter::CreatureTypes => {
                if !simple_core_query.is_empty() {
                    simple_core_query.push_str(" AND ")
                }
                simple_core_query.push_str(
                    prepare_in_statement_for_string_type(key.to_string().as_str(), value).as_str(),
                )
            }
            CreatureFilter::Traits => trait_query.push_str(prepare_trait_filter(value).as_str()),
            CreatureFilter::CreatureRoles => {
                if !simple_core_query.is_empty() {
                    simple_core_query.push_str(" AND ")
                }
                simple_core_query
                    .push_str(prepare_bounded_check(value, ACCURACY_THRESHOLD, 100).as_str())
            }
            _ => (),
        }
    }
    let mut where_query = simple_core_query.to_string();
    if !trait_query.is_empty() {
        where_query.push_str(format!(" AND id IN ({trait_query}) GROUP BY cc.id").as_str())
    }
    if !where_query.is_empty() {
        where_query = format!("WHERE {where_query}");
    }
    let query = format!("SELECT * FROM CREATURE_CORE cc {where_query} ORDER BY RANDOM() LIMIT 20");
    debug!("{}", query);
    query
}

/// Prepares a 'bounded AND statement' aka checks if all the columns are in the bound given
/// (brute_percentage >= 0 AND brute_percentage <= 0) AND (sniper_percentage >= 0 ...) ...
fn prepare_bounded_check(
    column_names: &HashSet<String>,
    lower_bound: i64,
    upper_bound: i64,
) -> String {
    let mut bounded_query = String::new();
    for column in column_names {
        if !bounded_query.is_empty() {
            bounded_query.push_str(" OR ");
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
    in_string.push_str(prepare_in_statement_for_string_type("tt.name", column_values).as_str());
    if !in_string.is_empty() {
        let select_query = "SELECT tcat.creature_id FROM TRAIT_CREATURE_ASSOCIATION_TABLE";
        let inner_query = format!("SELECT * FROM TRAIT_TABLE tt WHERE {in_string}");
        return format!(
            "{select_query} tcat RIGHT JOIN ({inner_query}) jt ON tcat.trait_id = jt.name"
        );
    }
    in_string
}

/// Prepares an 'in' statement in the following format. Assuming a string value
/// "UPPER(field) in (UPPER('el1'), UPPER('el2'), UPPER('el3'))"
fn prepare_in_statement_for_string_type(
    column_name: &str,
    column_values: &HashSet<String>,
) -> String {
    let mut result_string = String::new();
    if !column_values.is_empty() {
        result_string.push_str(format!("UPPER({column_name})").as_str());
        result_string.push_str(" IN (");
        column_values.iter().for_each(|x| {
            result_string.push_str(format!("UPPER('{x}')").as_str());
            result_string.push(',');
        });
        if result_string.ends_with(',') {
            result_string.remove(result_string.len() - 1);
        }
        result_string.push(')')
    }
    result_string
}

/// Prepares an 'in' statement in the following format
/// 'field in (el1, el2, el3)'
fn prepare_in_statement_for_generic_type(
    column_name: &str,
    column_values: &HashSet<String>,
) -> String {
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
