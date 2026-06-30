use crate::models::db::pg_type_helper::{get_i32_as_i64, get_opt_i32_as_i64};
use crate::models::hazard::hazard_field_filter::HazardComplexityEnum;
use crate::models::shared::rarity_enum::RarityEnum;
use crate::models::shared::size_enum::SizeEnum;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::{Error, FromRow, Row};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq, Debug)]
pub struct HazardEssentialData {
    pub id: i64,
    pub name: String,

    // Attributes
    pub ac: i64,
    pub hardness: i64,
    pub has_health: bool,
    pub hp: i64,
    pub stealth: i64,
    pub stealth_detail: String,

    // Details
    pub description: String,
    pub disable_description: String,
    pub reset_description: String,
    pub routine_description: String,
    pub complexity: HazardComplexityEnum,
    pub level: i64,
    pub license: String,
    pub remaster: bool,
    pub source: String,

    pub will: Option<i64>,
    pub reflex: Option<i64>,
    pub fortitude: Option<i64>,
    pub rarity: RarityEnum,
    pub size: SizeEnum,
}

impl<'r> FromRow<'r, PgRow> for HazardEssentialData {
    fn from_row(row: &'r PgRow) -> Result<Self, Error> {
        let rarity: String = row.try_get("rarity")?;
        let size: String = row.try_get("size")?;
        let is_complex: bool = row.try_get("is_complex")?;
        Ok(Self {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            ac: get_i32_as_i64(row, "ac")?,
            hardness: get_i32_as_i64(row, "hardness")?,
            has_health: row.try_get("has_health")?,
            hp: get_i32_as_i64(row, "hp")?,
            stealth: get_i32_as_i64(row, "stealth")?,
            stealth_detail: row.try_get("stealth_detail")?,
            description: row.try_get("description")?,
            disable_description: row.try_get("disable_description")?,
            reset_description: row.try_get("reset_description")?,
            routine_description: row.try_get("routine_description")?,
            complexity: HazardComplexityEnum::from(is_complex),
            size: SizeEnum::from(size),
            rarity: RarityEnum::from(rarity),
            license: row.try_get("license")?,
            remaster: row.try_get("remaster")?,
            source: row.try_get("source")?,
            will: get_opt_i32_as_i64(row, "will"),
            reflex: get_opt_i32_as_i64(row, "reflex"),
            level: get_i32_as_i64(row, "level")?,
            fortitude: get_opt_i32_as_i64(row, "fortitude"),
        })
    }
}
