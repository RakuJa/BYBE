use crate::models::db::pg_type_helper::{get_i32_as_i64, get_opt_i32_as_i64};
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
                ac_bonus: get_i32_as_i64(row, "bonus_ac")?,
                check_penalty: get_i32_as_i64(row, "check_penalty")?,
                dex_cap: get_i32_as_i64(row, "dex_cap")?,
                n_of_potency_runes: get_i32_as_i64(row, "n_of_potency_runes")?,
                property_runes: vec![],
                n_of_resilient_runes: get_i32_as_i64(row, "n_of_resilient_runes")?,
                speed_penalty: get_i32_as_i64(row, "speed_penalty")?,
                strength_required: get_opt_i32_as_i64(row, "strength_required"),
            },
        })
    }
}
