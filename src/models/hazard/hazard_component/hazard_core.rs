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
            ac: row.try_get::<i32, _>("ac")? as i64,
            hardness: row.try_get::<i32, _>("hardness")? as i64,
            has_health: row.try_get("has_health")?,
            hp: row.try_get::<i32, _>("hp")? as i64,
            stealth: row.try_get::<i32, _>("stealth")? as i64,
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
            will: row
                .try_get::<Option<i32>, _>("will")
                .ok()
                .flatten()
                .map(|v| v as i64),
            reflex: row
                .try_get::<Option<i32>, _>("reflex")
                .ok()
                .flatten()
                .map(|v| v as i64),
            level: row.try_get::<i32, _>("level")? as i64,
            fortitude: row
                .try_get::<Option<i32>, _>("fortitude")
                .ok()
                .flatten()
                .map(|v| v as i64),
        })
    }
}
