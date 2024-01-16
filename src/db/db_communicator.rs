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

async fn from_raw_vec_to_creature(conn: &Pool<Sqlite>, raw_vec: Vec<RawCreature>) -> Vec<Creature> {
    let mut creature_list = Vec::new();
    for el in raw_vec {
        creature_list.push(from_raw_to_creature(conn, &el).await);
    }
    creature_list
}

async fn from_raw_to_creature(conn: &Pool<Sqlite>, raw: &RawCreature) -> Creature {
    let creature_type = CreatureTypeEnum::from_str(raw.creature_type.as_str()).unwrap_or_default();
    let alignment_enum = AlignmentEnum::from_str(raw.alignment.as_str()).unwrap_or_default();
    let size_enum = SizeEnum::from_str(raw.size.as_str()).unwrap_or_default();
    let rarity_enum = RarityEnum::from_str(raw.rarity.as_str()).unwrap_or_default();
    let archive_link = generate_archive_link(raw.aon_id, &creature_type);

    let sources = sqlx::query_as::<_, CreatureSource>(
        &format!("SELECT * FROM source_table INTERSECT SELECT source_id FROM SOURCE_ASSOCIATION_TABLE WHERE creature_id == {}", raw.id)
    ).fetch_all(conn).await.unwrap_or_default();

    let traits = sqlx::query_as::<_, CreatureTrait>(
        &format!("SELECT * FROM trait_table INTERSECT SELECT trait_id FROM TRAIT_ASSOCIATION_TABLE WHERE creature_id == {}", raw.id)
    ).fetch_all(conn).await.unwrap_or_default();
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
        sources: sources
            .into_iter()
            .map(|curr_source| curr_source.name)
            .collect(),
        traits: traits
            .into_iter()
            .map(|curr_trait| curr_trait.name)
            .collect(),
        creature_type,
        archive_link,
        is_weak: false,
        is_elite: false,
    }
}

pub async fn fetch_creatures(conn: &Pool<Sqlite>) -> Result<Vec<Creature>, Error> {
    let creatures = sqlx::query_as::<_, RawCreature>("SELECT * FROM CREATURE_TABLE ORDER BY name")
        .fetch_all(conn)
        .await;
    match creatures {
        Ok(creature_list) => Ok(from_raw_vec_to_creature(conn, creature_list).await),
        Err(err) => {
            warn!("Error converting data from db {}", err);
            Err(err)
        }
    }
}
