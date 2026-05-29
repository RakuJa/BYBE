use crate::models::db::pg_type_helper::get_i32_as_i64;
use crate::models::item::item_struct::Item;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::{Error, FromRow, Row};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq, Debug)]
pub struct Shield {
    pub item_core: Item,
    pub shield_data: ShieldData,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq, Debug)]
pub struct ShieldData {
    pub id: i64,
    #[schema(example = 0)]
    pub bonus_ac: i64,
    #[schema(example = 0)]
    pub n_of_reinforcing_runes: i64,
    #[schema(example = 0)]
    pub speed_penalty: i64,
}

impl<'r> FromRow<'r, PgRow> for Shield {
    fn from_row(row: &'r PgRow) -> Result<Self, Error> {
        let item_core = Item::from_row(row)?;
        Ok(Self {
            item_core,
            shield_data: ShieldData {
                id: row.try_get("shield_id")?,
                bonus_ac: get_i32_as_i64(row, "bonus_ac")?,
                n_of_reinforcing_runes: get_i32_as_i64(row, "n_of_reinforcing_runes")?,
                speed_penalty: get_i32_as_i64(row, "speed_penalty")?,
            },
        })
    }
}
