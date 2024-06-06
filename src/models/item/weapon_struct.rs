use crate::models::item::item_metadata::type_enum::WeaponTypeEnum;
use crate::models::item::item_struct::Item;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;
use sqlx::{Error, FromRow, Row};
use std::str::FromStr;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq)]
pub struct Weapon {
    pub item_core: Item,
    pub weapon_core: WeaponData,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq)]
pub struct WeaponData {
    pub id: i64,
    pub bonus_dmg: i64,
    pub to_hit_bonus: Option<i64>,
    pub dmg_type: Option<String>,
    pub number_of_dice: Option<i64>,
    pub die_size: Option<String>,
    pub splash_dmg: Option<i64>,
    pub n_of_potency_runes: i64,
    pub n_of_striking_runes: i64,
    pub property_runes: Vec<String>,
    pub range: Option<i64>,
    pub reload: Option<String>,
    pub weapon_type: WeaponTypeEnum,
}

impl<'r> FromRow<'r, SqliteRow> for Weapon {
    fn from_row(row: &'r SqliteRow) -> Result<Self, Error> {
        let item_core = Item::from_row(row)?;
        let wp_type = WeaponTypeEnum::from_str(row.try_get("weapon_type")?);
        Ok(Weapon {
            item_core,
            weapon_core: WeaponData {
                id: row.try_get("id")?,
                bonus_dmg: row.try_get("bonus_dmg")?,
                to_hit_bonus: row.try_get("to_hit_bonus")?,
                dmg_type: row.try_get("dmg_type")?,
                number_of_dice: row.try_get("number_of_dice")?,
                die_size: row.try_get("die_size")?,
                splash_dmg: row.try_get("splash_dmg")?,
                n_of_potency_runes: row.try_get("n_of_potency_runes")?,
                n_of_striking_runes: row.try_get("n_of_striking_runes")?,
                property_runes: vec![],
                range: row.try_get("range")?,
                reload: row.try_get("reload")?,
                weapon_type: wp_type.unwrap_or(WeaponTypeEnum::Melee),
            },
        })
    }
}

impl Weapon {
    pub fn get_avg_dmg(&self) -> Option<i64> {
        // avg dice value is
        // AVG = (((M+1)/2)∗N)+B
        //
        // M = max value of the dice
        // N = number of dices
        // B = bonus dmg
        let m = self
            .weapon_core
            .die_size
            .clone()?
            .split_once('d')?
            .1
            .parse::<f64>()
            .ok()?;
        let n = self.weapon_core.number_of_dice? as f64;
        let b = self.weapon_core.bonus_dmg as f64;

        let avg: f64 = (((m + 1.) / 2.) * n) + b;
        Some(avg.floor() as i64)
    }
}
