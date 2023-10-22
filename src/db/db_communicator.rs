use crate::models::creature::Creature;
use crate::models::creature_metadata_enums::{AlignmentEnum, RarityEnum, SizeEnum};
use crate::services::url_calculator::generate_archive_link;
use log::warn;
use redis::{
    from_redis_value, Commands, Connection, ConnectionLike, FromRedisValue, JsonCommands,
    RedisError, RedisResult, Value,
};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize)]
pub struct RawCreature {
    name: String,
    hp: i16,
    level: i8,
    alignment: AlignmentEnum,
    size: SizeEnum,
    family: Option<String>,
    rarity: RarityEnum,
    is_melee: i8,
    is_ranged: i8,
    is_spell_caster: i8,
    sources: String,
    traits: String,
}

impl FromRedisValue for RawCreature {
    fn from_redis_value(v: &Value) -> RedisResult<Self> {
        let json_str: String = from_redis_value(v)?;
        let vec_of_raw_strings: serde_json::error::Result<Vec<String>> =
            serde_json::from_str(&json_str);
        match vec_of_raw_strings {
            Ok(mut raw) => {
                // raw is a vec of one element, we can pop it and forget
                let x: serde_json::error::Result<RawCreature> =
                    serde_json::from_str(&(raw.pop().unwrap_or(String::from(""))));
                x
            }
            Err(err) => Err(err),
        }
        .map_err(redis::RedisError::from)
    }
}

fn from_raw_vec_to_creature(raw_vec: Vec<RawCreature>, id_vec: Vec<String>) -> Vec<Creature> {
    raw_vec
        .iter()
        .zip(id_vec.iter())
        .map(|(raw, identifier)| from_raw_to_creature(raw, identifier))
        .collect()
}

fn extract_vec_from_raw_string(raw_vector: &str) -> Vec<String> {
    // Extracts a vec of string from a json string representing a vector
    // ex "['hi', 'man']" => ['hi', 'man']
    // complex string ex "oneword's secondword" are stored in double quotes
    // simple strings in ''
    let double_quotes_re = Regex::new("\"([^\"]+)\"").unwrap();
    let single_quotes_re = Regex::new(r"'([^']+)'").unwrap();

    let resulting_vector: Vec<String> = double_quotes_re
        .captures_iter(raw_vector)
        .filter(|capture| capture.len() >= 2)
        .map(|capture| capture[1].to_string())
        .collect();
    if !resulting_vector.is_empty() {
        resulting_vector
    } else {
        let resulting_vector: Vec<String> = single_quotes_re
            .captures_iter(raw_vector)
            .map(|capture| capture[1].to_string())
            .collect();
        resulting_vector
    }
}

fn from_raw_to_creature(raw: &RawCreature, identifier: &str) -> Creature {
    let id = identifier.parse::<i32>().unwrap_or(0);
    let sources_list = extract_vec_from_raw_string(&raw.sources);
    let traits_list = extract_vec_from_raw_string(&raw.traits);

    Creature {
        id,
        name: raw.name.clone(),
        hp: raw.hp,
        level: raw.level,
        alignment: raw.alignment.clone(),
        size: raw.size.clone(),
        family: raw.family.clone(),
        rarity: raw.rarity.clone(),
        is_melee: raw.is_melee != 0,
        is_ranged: raw.is_ranged != 0,
        is_spell_caster: raw.is_spell_caster != 0,
        sources: sources_list,
        traits: traits_list,
        archive_link: generate_archive_link(id),
    }
}

fn remove_prefix(strings: Vec<String>, prefix: &String) -> Vec<String> {
    strings
        .iter()
        // removes prefix, if it could not be removed it filters out the value
        .filter_map(|curr_str| curr_str.as_str().strip_prefix(prefix))
        .map(str::to_string) //convert &str to String
        .collect()
}
fn get_redis_url() -> String {
    let redis_password = env::var("REDIS_KEY").unwrap_or_else(|_| "".to_string());

    let redis_ip = env::var("REDIS_IP").unwrap_or_else(|_| "localhost".to_string());
    let redis_port = env::var("REDIS_PORT").unwrap_or_else(|_| "6379".to_string());

    format!("redis://:{}@{}:{}", redis_password, redis_ip, redis_port)
}

fn get_connection() -> RedisResult<Connection> {
    let client = redis::Client::open(get_redis_url())?;
    let conn = client.get_connection()?;
    Ok(conn)
}

pub fn get_creatures_by_ids(ids: Vec<String>) -> Result<Vec<Creature>, RedisError> {
    let mut conn = get_connection()?;
    let raw_results: RedisResult<Vec<RawCreature>> = conn.json_get(ids.clone(), "$");
    // Convert each RawJsonString to RawCreature and collect the results into a Vec<RawCreature>
    let mut raw_creatures = Vec::new();
    match raw_results {
        Ok(creature_list) => {
            for raw in creature_list {
                raw_creatures.push(raw);
            }
        }
        Err(err) => warn!("Error converting data from db {}", err),
    }

    Ok(from_raw_vec_to_creature(raw_creatures, ids))
}

pub fn fetch_and_parse_all_keys(pattern: &String) -> Result<Vec<String>, RedisError> {
    let mut conn = get_connection()?;
    let mut parse_pattern = pattern.clone();
    if !pattern.ends_with('*') {
        parse_pattern.push('*')
    }

    let keys: Vec<String> = conn.scan_match(parse_pattern)?.collect();
    Ok(remove_prefix(keys, pattern))
}

pub fn is_redis_up() -> RedisResult<bool> {
    Ok(get_connection()?.is_open())
}
