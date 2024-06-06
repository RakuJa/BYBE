use crate::models::item::item_struct::Item;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;
use sqlx::{Error, FromRow, Row};
use utoipa::ToSchema;

#[derive(Clone, ToSchema)]
pub struct Armor {
    pub item_core: Item,
    pub armor_core: ArmorData,
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct ArmorData {
    pub id: i64,
    pub ac_bonus: i64,
    pub check_penalty: i64,
    pub dex_cap: i64,
    pub n_of_potency_runes: i64,
    pub property_runes: Vec<String>,
    pub n_of_resilient_runes: i64,
    pub speed_penalty: i64,
    pub strength_required: i64,
}

impl<'r> FromRow<'r, SqliteRow> for Armor {
    fn from_row(row: &'r SqliteRow) -> Result<Self, Error> {
        let item_core = Item::from_row(row)?;
        Ok(Armor {
            item_core,
            armor_core: ArmorData {
                id: row.try_get("id")?,
                ac_bonus: row.try_get("ac_bonus")?,
                check_penalty: row.try_get("check_penalty")?,
                dex_cap: row.try_get("dex_cap")?,
                n_of_potency_runes: row.try_get("n_of_potency_runes")?,
                property_runes: vec![],
                n_of_resilient_runes: row.try_get("n_of_resilient_runes")?,
                speed_penalty: row.try_get("speed_penalty")?,
                strength_required: row.try_get("strength_required")?,
            },
        })
    }
}
