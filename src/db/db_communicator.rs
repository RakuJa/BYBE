use crate::models::creature::Creature;
use crate::models::creature_metadata_enums::{
    AlignmentEnum, CreatureTypeEnum, RarityEnum, SizeEnum,
};
use crate::services::url_calculator::generate_archive_link;
use log::warn;
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, Pool, Sqlite};
use std::str::FromStr;

#[derive(Serialize, Deserialize, FromRow)]
pub struct RawCreature {
    id: i32,
    aon_id: i32,
    name: String,
    hp: i16,
    level: i8,
    alignment: String,
    size: String,
    family: Option<String>,
    rarity: String,
    is_melee: i8,
    is_ranged: i8,
    is_spell_caster: i8,
    creature_type: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct CreatureTrait {
    name: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct CreatureSource {
    name: String,
}

fn from_raw_vec_to_creature(raw_vec: Vec<RawCreature>) -> Vec<Creature> {
    raw_vec.iter().map(from_raw_to_creature).collect()
}

fn from_raw_to_creature(raw: &RawCreature) -> Creature {
    let creature_type =
        CreatureTypeEnum::from_str(raw.creature_type.clone().as_str()).unwrap_or_default();
    let alignment_enum = AlignmentEnum::from_str(raw.alignment.as_str()).unwrap_or_default();
    let size_enum = SizeEnum::from_str(raw.size.as_str()).unwrap_or_default();
    let rarity_enum = RarityEnum::from_str(raw.rarity.as_str()).unwrap_or_default();
    let archive_link = generate_archive_link(raw.aon_id, &creature_type);
    Creature {
        id: raw.id,
        aon_id: raw.aon_id,
        name: raw.name.clone(),
        hp: raw.hp,
        level: raw.level,
        alignment: alignment_enum,
        size: size_enum,
        family: raw.family.clone(),
        rarity: rarity_enum,
        is_melee: raw.is_melee != 0,
        is_ranged: raw.is_ranged != 0,
        is_spell_caster: raw.is_spell_caster != 0,
        sources: Vec::new(),
        traits: Vec::new(),
        creature_type,
        archive_link,
    }
}

pub async fn fetch_creatures(conn: &Pool<Sqlite>) -> Result<Vec<Creature>, Error> {
    let x = sqlx::query_as::<_, RawCreature>("SELECT * FROM CREATURE_TABLE")
        .fetch_all(conn)
        .await;
    match x {
        Ok(creature_list) => Ok(from_raw_vec_to_creature(creature_list)),
        Err(err) => {
            warn!("Error converting data from db {}", err);
            Err(err)
        }
    }
}
