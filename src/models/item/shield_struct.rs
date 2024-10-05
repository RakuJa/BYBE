use crate::models::item::item_struct::Item;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;
use sqlx::{Error, FromRow, Row};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq)]
pub struct Shield {
    pub item_core: Item,
    pub shield_data: ShieldData,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq)]
pub struct ShieldData {
    pub id: i64,
    #[schema(example = 0)]
    pub bonus_ac: i64,
    #[schema(example = 0)]
    pub n_of_reinforcing_runes: i64,
    #[schema(example = 0)]
    pub speed_penalty: i64,
}

impl<'r> FromRow<'r, SqliteRow> for Shield {
    fn from_row(row: &'r SqliteRow) -> Result<Self, Error> {
        let item_core = Item::from_row(row)?;
        Ok(Shield {
            item_core,
            shield_data: ShieldData {
                id: row.try_get("shield_id")?,
                bonus_ac: row.try_get("bonus_ac")?,
                n_of_reinforcing_runes: row.try_get("n_of_reinforcing_runes")?,
                speed_penalty: row.try_get("speed_penalty")?,
            },
        })
    }
}
