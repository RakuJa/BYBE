use crate::models::item::item_metadata::type_enum::WeaponTypeEnum;
use crate::models::item::item_struct::Item;
use crate::models::routers_validator_structs::Dice;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;
use sqlx::{Error, FromRow, Row};
use std::str::FromStr;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq)]
pub struct Weapon {
    pub item_core: Item,
    pub weapon_data: WeaponData,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq)]
pub struct WeaponData {
    pub id: i64,
    pub to_hit_bonus: Option<i64>,
    pub damage_data: Vec<DamageData>,
    pub n_of_potency_runes: i64,
    pub n_of_striking_runes: i64,
    pub property_runes: Vec<String>,
    pub range: Option<i64>,
    pub reload: Option<String>,
    pub weapon_type: WeaponTypeEnum,
    pub splash_dmg: Option<i64>,
}

impl<'r> FromRow<'r, SqliteRow> for Weapon {
    fn from_row(row: &'r SqliteRow) -> Result<Self, Error> {
        let item_core = Item::from_row(row)?;
        let wp_type = WeaponTypeEnum::from_str(row.try_get("weapon_type")?);
        Ok(Weapon {
            item_core,
            weapon_data: WeaponData {
                id: row.try_get("weapon_id")?,
                to_hit_bonus: row.try_get("to_hit_bonus")?,
                n_of_potency_runes: row.try_get("n_of_potency_runes")?,
                n_of_striking_runes: row.try_get("n_of_striking_runes")?,
                property_runes: vec![],
                range: row.try_get("range")?,
                reload: row.try_get("reload")?,
                weapon_type: wp_type.unwrap_or(WeaponTypeEnum::Melee),
                damage_data: vec![],
                splash_dmg: row.try_get("splash_dmg").ok(),
            },
        })
    }
}

impl Weapon {
    pub fn get_avg_dmg(&self) -> i64 {
        self.weapon_data
            .damage_data
            .iter()
            .map(|x| {
                let b = x.bonus_dmg as f64;
                if let Some(dice) = x.clone().dice {
                    dice.get_avg_dmg(b)
                } else {
                    0
                }
            })
            .sum()
    }
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq)]
pub struct DamageData {
    pub id: i64,
    pub bonus_dmg: i64,
    pub dmg_type: Option<String>,
    pub dice: Option<Dice>,
}

impl<'r> FromRow<'r, SqliteRow> for DamageData {
    fn from_row(row: &'r SqliteRow) -> Result<Self, Error> {
        Ok(DamageData {
            id: row.try_get("id")?,
            bonus_dmg: row.try_get("bonus_dmg")?,
            dmg_type: row.try_get("dmg_type").ok(),
            dice: Dice::from_optional_dice_number_and_size(
                row.try_get("number_of_dice").ok(),
                row.try_get("die_size").ok(),
            ),
        })
    }
}
