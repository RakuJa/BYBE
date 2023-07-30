use crate::db::db_communicator::{
    fetch_and_parse_all_keys, get_creature_by_id, get_creatures_by_ids,
};
use crate::models::creature::Creature;
use redis::RedisError;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BestiaryResponse {
    results: Option<Vec<Creature>>,
    count: usize,
    next: Option<String>,
}

fn convert_result_to_bestiary_response(
    result: Result<Vec<Creature>, RedisError>,
) -> BestiaryResponse {
    match result {
        Ok(cr) => {
            println!("{:?}", cr);
            BestiaryResponse {
                results: Some(cr),
                count: 0, //cr.len(),
                next: Some(String::from("https")),
            }
        }
        Err(_) => BestiaryResponse {
            results: None,
            count: 0,
            next: None,
        },
    }
}
pub async fn get_creature(id: &String) -> Option<Creature> {
    match get_creature_by_id(id) {
        Ok(cr) => hashmap! {String::from("results") => Some(cr)},
        _ => hashmap! {String::from("results") => None},
    };
    match get_creature_by_id(id) {
        Ok(cr) => Some(cr),
        _ => None,
    }
}

pub async fn get_bestiary(ids: Vec<String>) -> BestiaryResponse {
    convert_result_to_bestiary_response(get_creatures_by_ids(ids))
}

pub async fn get_keys() -> Vec<String> {
    match fetch_and_parse_all_keys(&"creature:".to_string()) {
        Ok(cr) => cr,
        Err(err) => vec![],
    }
}
