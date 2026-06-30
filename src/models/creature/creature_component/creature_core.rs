use crate::models::creature::creature_metadata::type_enum::CreatureTypeEnum;
use crate::models::db::pg_type_helper::{get_i32_as_i64, get_opt_i32_as_i64};
use crate::models::shared::alignment_enum::AlignmentEnum;
use crate::models::shared::rarity_enum::RarityEnum;
use crate::models::shared::size_enum::SizeEnum;
use crate::models::shared::status_enum::Status;
use crate::traits::traits_enrichable::TraitsEnrichable;
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::json;
use sqlx::postgres::PgRow;
use sqlx::{Error, FromRow, Row};
use std::collections::BTreeMap;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq, Debug)]
pub struct CreatureCoreData {
    pub essential: EssentialData,
    pub derived: DerivedData,
    pub traits: Vec<String>,
}
#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq, Debug)]
pub struct EssentialData {
    pub id: i64,
    pub aon_id: Option<i64>,
    pub name: String,
    #[schema(example = 0)]
    pub hp: i64,
    #[schema(example = 0)]
    pub base_level: i64,
    pub size: SizeEnum,
    pub family: String,
    pub rarity: RarityEnum,
    pub license: String,
    pub remaster: bool,
    pub source: String,
    pub cr_type: CreatureTypeEnum,
    pub alignment: AlignmentEnum,
    pub focus_points: i64,
    pub status: Status,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq, Debug)]
pub struct DerivedData {
    pub archive_link: Option<String>,

    #[schema(example = json!({"melee": true, "ranged": false, "spellcaster": true}))]
    pub attack_data: BTreeMap<String, bool>,
    #[schema(example = json!({"brute": 50, "magical_striker": 30, "skill_paragon": 2, "skirmisher": 3, "sniper": 0, "soldier": 30, "spellcaster": 90}))]
    pub role_data: BTreeMap<String, i64>,
}

impl<'r> FromRow<'r, PgRow> for EssentialData {
    fn from_row(row: &'r PgRow) -> Result<Self, Error> {
        let rarity: String = row.try_get("rarity")?;
        let size: String = row.try_get("size")?;
        let alignment: String = row.try_get("alignment")?;
        let status_str: String = row.try_get("status").unwrap_or_default();
        Ok(Self {
            id: row.try_get("id")?,
            aon_id: get_opt_i32_as_i64(row, "aon_id"),
            name: row.try_get("name")?,
            hp: get_i32_as_i64(row, "hp")?,
            base_level: get_i32_as_i64(row, "level")?,
            size: SizeEnum::from(size),
            family: row.try_get("family").unwrap_or_else(|_| String::from("-")),
            rarity: RarityEnum::from(rarity),
            license: row.try_get("license")?,
            remaster: row.try_get("remaster")?,
            source: row.try_get("source")?,
            cr_type: CreatureTypeEnum::from(row.try_get("cr_type").ok()),
            alignment: AlignmentEnum::from(alignment),
            focus_points: get_i32_as_i64(row, "focus_points")?,
            status: status_str.into(),
        })
    }
}

impl<'r> FromRow<'r, PgRow> for DerivedData {
    fn from_row(row: &'r PgRow) -> Result<Self, Error> {
        let mut attack_list = BTreeMap::new();
        attack_list.insert(String::from("melee"), row.try_get("is_melee")?);
        attack_list.insert(String::from("ranged"), row.try_get("is_ranged")?);
        attack_list.insert(String::from("spellcaster"), row.try_get("is_spellcaster")?);

        let mut role_list = BTreeMap::new();
        role_list.insert(String::from("brute"), row.try_get("brute_percentage")?);
        role_list.insert(
            String::from("magical_striker"),
            row.try_get("magical_striker_percentage")?,
        );
        role_list.insert(
            String::from("skill_paragon"),
            row.try_get("skill_paragon_percentage")?,
        );
        role_list.insert(
            String::from("skirmisher"),
            row.try_get("skirmisher_percentage")?,
        );
        role_list.insert(String::from("sniper"), row.try_get("sniper_percentage")?);
        role_list.insert(String::from("soldier"), row.try_get("soldier_percentage")?);
        role_list.insert(
            String::from("spellcaster"),
            row.try_get("spellcaster_percentage")?,
        );
        Ok(Self {
            archive_link: row.try_get("archive_link").ok(),
            attack_data: attack_list,
            role_data: role_list,
        })
    }
}

impl<'r> FromRow<'r, PgRow> for CreatureCoreData {
    fn from_row(row: &'r PgRow) -> Result<Self, Error> {
        Ok(Self {
            essential: EssentialData::from_row(row)?,
            derived: DerivedData::from_row(row)?,
            traits: vec![],
        })
    }
}

impl TraitsEnrichable for CreatureCoreData {
    fn entity_id(&self) -> i64 {
        self.essential.id
    }
    fn set_traits(&mut self, traits: Vec<String>) {
        self.traits = traits;
    }
    fn entity_name() -> &'static str {
        "creature"
    }
}
