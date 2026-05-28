use crate::models::item::item_struct::Item;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::{Error, FromRow, Row};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq, Debug)]
pub struct Armor {
    pub item_core: Item,
    pub armor_data: ArmorData,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq, Debug)]
pub struct ArmorData {
    pub id: i64,
    #[schema(example = 0)]
    pub ac_bonus: i64,
    #[schema(example = 0)]
    pub check_penalty: i64,
    #[schema(example = 0)]
    pub dex_cap: i64,
    #[schema(example = 0)]
    pub n_of_potency_runes: i64,
    pub property_runes: Vec<String>,
    #[schema(example = 0)]
    pub n_of_resilient_runes: i64,
    #[schema(example = 0)]
    pub speed_penalty: i64,
    #[schema(example = 0)]
    pub strength_required: Option<i64>,
}

impl<'r> FromRow<'r, PgRow> for Armor {
    fn from_row(row: &'r PgRow) -> Result<Self, Error> {
        let item_core = Item::from_row(row)?;
        Ok(Self {
            item_core,
            armor_data: ArmorData {
                id: row.try_get("armor_id")?,
                ac_bonus: row.try_get::<i32, _>("bonus_ac")? as i64,
                check_penalty: row.try_get::<i32, _>("check_penalty")? as i64,
                dex_cap: row.try_get::<i32, _>("dex_cap")? as i64,
                n_of_potency_runes: row.try_get::<i32, _>("n_of_potency_runes")? as i64,
                property_runes: vec![],
                n_of_resilient_runes: row.try_get::<i32, _>("n_of_resilient_runes")? as i64,
                speed_penalty: row.try_get::<i32, _>("speed_penalty")? as i64,
                strength_required: row
                    .try_get::<Option<i32>, _>("strength_required")
                    .ok()
                    .flatten()
                    .map(|v| v as i64),
            },
        })
    }
}
