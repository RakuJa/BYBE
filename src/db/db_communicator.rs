use crate::models::creature::Creature;
use crate::models::creature_metadata_enums::{AlignmentEnum, RarityEnum, SizeEnum};
use redis::{
    from_redis_value, Commands, Connection, ConnectionLike, FromRedisValue, JsonCommands,
    RedisError, RedisResult, Value,
};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
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
    // source: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RawJsonString {
    json_string: String,
}

impl FromRedisValue for RawJsonString {
    fn from_redis_value(v: &Value) -> RedisResult<Self> {
        let json_str: String = from_redis_value(v)?;
        serde_json::from_str(&json_str).map_err(redis::RedisError::from)
    }
}

fn from_raw_vec_to_creature(raw_vec: Vec<RawCreature>, id_vec: Vec<String>) -> Vec<Creature> {
    raw_vec
        .iter()
        .zip(id_vec.iter())
        .map(|(raw, identifier)| from_raw_to_creature(raw, identifier))
        .collect()
}

fn from_raw_to_creature(raw: &RawCreature, identifier: &str) -> Creature {
    Creature {
        id: identifier.parse::<i32>().unwrap_or(0),
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
        source: vec![],
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
    let raw_results: Vec<RawJsonString> = conn.json_get(ids.clone(), "$")?;

    // Convert each RawJsonString to RawCreature and collect the results into a Vec<RawCreature>
    let mut raw_creatures = Vec::new();
    for json_string in raw_results {
        let raw: RawCreature = serde_json::from_str(&json_string.json_string)?;
        raw_creatures.push(raw);
    }

    Ok(from_raw_vec_to_creature(raw_creatures, ids))
}

pub fn get_creature_by_id(id: &String) -> Result<Creature, RedisError> {
    let mut conn = get_connection()?;
    let json_string: RawJsonString = conn.json_get(id, "$")?;
    let raw: RawCreature = serde_json::from_str(&json_string.json_string)?;
    Ok(from_raw_to_creature(&raw, id))
}

pub fn fetch_and_parse_all_keys(pattern: &String) -> Result<Vec<String>, RedisError> {
    let mut conn = get_connection()?;
    let mut parse_pattern = pattern.clone();
    if !pattern.ends_with('*') {
        parse_pattern.push('*')
    }

    let x: Vec<String> = conn.keys(parse_pattern)?;
    Ok(remove_prefix(x, pattern))
}

pub fn is_redis_up() -> RedisResult<bool> {
    Ok(get_connection()?.is_open())
}
