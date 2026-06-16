use crate::models::db::pg_type_helper::get_i32_as_i64;
use crate::models::shared::range_data::RangeData;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::{Error, FromRow, Row};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq, Debug)]
pub struct Spell {
    pub id: i64,
    pub name: String,
    pub area_type: Option<String>,
    #[schema(example = 5)]
    pub area_value: Option<i32>,
    pub counteraction: bool,

    pub basic_saving_throw: Option<bool>,
    pub saving_throw: Option<String>,
    pub sustained: bool,

    pub duration: Option<String>,
    #[schema(example = 1)]
    pub level: i64,
    pub range: Option<RangeData>,
    pub target: String,
    pub actions: String,

    pub license: String,
    pub remaster: bool,
    pub source: String,
    pub rarity: String, // use rarityenum

    pub slot: i64,
    pub creature_id: i64,
    pub spellcasting_entry_id: i64,
}

impl<'r> FromRow<'r, PgRow> for Spell {
    fn from_row(row: &'r PgRow) -> Result<Self, Error> {
        Ok(Self {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            area_type: row.try_get("area_type")?,
            area_value: row.try_get("area_value")?,
            counteraction: row.try_get("counteraction")?,
            basic_saving_throw: row.try_get("basic_saving_throw")?,
            saving_throw: row.try_get("saving_throw")?,
            sustained: row.try_get("sustained")?,
            duration: row.try_get("duration")?,
            level: get_i32_as_i64(row, "level")?,
            target: row.try_get("target")?,
            actions: row.try_get("actions")?,
            license: row.try_get("license")?,
            remaster: row.try_get("remaster")?,
            source: row.try_get("source")?,
            rarity: row.try_get("rarity")?,
            slot: get_i32_as_i64(row, "slot")?,
            creature_id: row.try_get("creature_id")?,
            spellcasting_entry_id: row.try_get("spellcasting_entry_id")?,
            range: RangeData::from_row(row).ok(),
        })
    }
}
