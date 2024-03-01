use crate::models::creature::{CoreCreatureData, Creature, ExtraCreatureData, VariantCreatureData};
use crate::models::creature_metadata::alignment_enum::AlignmentEnum;
use crate::models::creature_metadata::rarity_enum::RarityEnum;
use crate::models::creature_metadata::size_enum::SizeEnum;
use crate::models::creature_metadata::type_enum::CreatureTypeEnum;
use crate::models::creature_metadata::variant_enum::CreatureVariant;
use crate::models::items::spell::Spell;
use crate::models::items::weapon::Weapon;
use crate::services::url_calculator::generate_archive_link;
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, Pool, Sqlite};

#[derive(Serialize, Deserialize, FromRow)]
pub struct RawCreature {
    id: i64,
    aon_id: Option<i64>,
    name: String,
    charisma: i64,
    constitution: i64,
    dexterity: i64,
    intelligence: i64,
    strength: i64,
    wisdom: i64,
    ac: i64,
    hp: i64,
    hp_detail: String,
    ac_detail: String,
    language_detail: Option<String>,
    level: i64,
    license: String,
    remaster: bool,
    source: String,
    initiative_ability: String,
    perception: i64,
    perception_detail: String,
    fortitude: i64,
    reflex: i64,
    will: i64,
    fortitude_detail: String,
    reflex_detail: String,
    will_detail: String,
    rarity: RarityEnum,
    size: SizeEnum,
    cr_type: CreatureTypeEnum,
    family: Option<String>,
    is_spell_caster: bool,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct CreatureTrait {
    pub name: String,
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
    let archive_link = generate_archive_link(raw.aon_id, &raw.cr_type);

    let traits = sqlx::query_as!(
        CreatureTrait,
        "SELECT * FROM TRAIT_TABLE INTERSECT SELECT trait_id FROM TRAIT_CREATURE_ASSOCIATION_TABLE WHERE creature_id == ($1)", raw.id
    ).fetch_all(conn).await.unwrap_or_default();
    let weapons = sqlx::query_as!(
        Weapon,
        "SELECT * FROM WEAPON_TABLE WHERE creature_id == ($1)",
        raw.id
    )
    .fetch_all(conn)
    .await
    .unwrap_or_default();

    let spells = sqlx::query_as!(
        Spell,
        "SELECT * FROM SPELL_TABLE WHERE creature_id == ($1)",
        raw.id
    )
    .fetch_all(conn)
    .await
    .unwrap_or_default();

    let alignment_enum = AlignmentEnum::from_trait_vec(&traits, raw.remaster);
    Creature {
        core_data: CoreCreatureData {
            id: raw.id as i32,
            aon_id: raw.aon_id.map(|x| x as i32),
            name: raw.name.clone(),
            hp: raw.hp as i16,
            base_level: raw.level as i8,
            alignment: alignment_enum,
            size: raw.size.clone(),
            family: raw.family.clone(),
            rarity: raw.rarity.clone(),
            is_spell_caster: raw.is_spell_caster,
            source: raw.source.clone(),
            traits: traits
                .into_iter()
                .map(|curr_trait| curr_trait.name)
                .collect(),
            creature_type: raw.cr_type.clone(),
            archive_link: archive_link.clone(),
            variant: CreatureVariant::Base,
            is_ranged: CoreCreatureData::is_ranged(&weapons),
            is_melee: CoreCreatureData::is_melee(&weapons),
        },
        variant_data: VariantCreatureData {
            level: raw.level as i8,
            archive_link,
        },
        extra_data: ExtraCreatureData { weapons, spells },
    }
}

pub async fn fetch_creatures(conn: &Pool<Sqlite>) -> Result<Vec<Creature>, Error> {
    Ok(from_raw_vec_to_creature(
        conn,
        sqlx::query_as!(RawCreature, "SELECT * FROM CREATURE_TABLE ORDER BY name")
            .fetch_all(conn)
            .await?,
    )
    .await)
}
