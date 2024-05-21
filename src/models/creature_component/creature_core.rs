use crate::models::creature_metadata::alignment_enum::AlignmentEnum;
use crate::models::creature_metadata::rarity_enum::RarityEnum;
use crate::models::creature_metadata::size_enum::SizeEnum;
use crate::models::creature_metadata::type_enum::CreatureTypeEnum;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;
use sqlx::{Error, FromRow, Row};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq)]
pub struct CreatureCoreData {
    pub essential: EssentialData,
    pub derived: DerivedData,
    pub traits: Vec<String>,
}
#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq, FromRow)]
pub struct EssentialData {
    pub id: i64,
    pub aon_id: Option<i64>,
    pub name: String,
    pub hp: i64,
    pub level: i64,
    pub size: SizeEnum,
    pub family: String,
    pub rarity: RarityEnum,
    pub license: String,
    pub remaster: bool,
    pub source: String,
    pub cr_type: CreatureTypeEnum,
    pub alignment: AlignmentEnum,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq, FromRow)]
pub struct DerivedData {
    pub archive_link: Option<String>,

    pub is_melee: bool,
    pub is_ranged: bool,
    pub is_spell_caster: bool,

    pub brute_percentage: i64,
    pub magical_striker_percentage: i64,
    pub skill_paragon_percentage: i64,
    pub skirmisher_percentage: i64,
    pub sniper_percentage: i64,
    pub soldier_percentage: i64,
    pub spell_caster_percentage: i64,
}

impl<'r> FromRow<'r, SqliteRow> for CreatureCoreData {
    fn from_row(row: &'r SqliteRow) -> Result<Self, Error> {
        let rarity: String = row.try_get("rarity")?;
        let size: String = row.try_get("size")?;
        let alignment: String = row.try_get("alignment")?;
        Ok(CreatureCoreData {
            essential: EssentialData {
                id: row.try_get("id")?,
                aon_id: row.try_get("aon_id").ok(),
                name: row.try_get("name")?,
                hp: row.try_get("hp")?,
                level: row.try_get("level")?,
                size: SizeEnum::from(size),
                family: row.try_get("family").unwrap_or(String::from("-")),
                rarity: RarityEnum::from(rarity),
                license: row.try_get("license")?,
                remaster: row.try_get("remaster")?,
                source: row.try_get("source")?,
                cr_type: CreatureTypeEnum::from(row.try_get("cr_type").ok()),
                alignment: AlignmentEnum::from(alignment),
            },
            derived: DerivedData {
                archive_link: row.try_get("archive_link").ok(),
                is_melee: row.try_get("is_melee")?,
                is_ranged: row.try_get("is_ranged")?,
                is_spell_caster: row.try_get("is_spell_caster")?,
                brute_percentage: row.try_get("brute_percentage")?,
                magical_striker_percentage: row.try_get("magical_striker_percentage")?,
                skill_paragon_percentage: row.try_get("skill_paragon_percentage")?,
                skirmisher_percentage: row.try_get("skirmisher_percentage")?,
                sniper_percentage: row.try_get("sniper_percentage")?,
                soldier_percentage: row.try_get("soldier_percentage")?,
                spell_caster_percentage: row.try_get("spell_caster_percentage")?,
            },
            traits: vec![],
        })
    }
}
