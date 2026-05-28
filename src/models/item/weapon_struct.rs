use crate::models::item::item_metadata::type_enum::WeaponTypeEnum;
use crate::models::item::item_struct::Item;
use crate::models::routers_validator_structs::Dice;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::{Error, FromRow, Row};
use std::str::FromStr;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq, Debug)]
pub struct Weapon {
    pub item_core: Item,
    pub weapon_data: WeaponData,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq, Debug)]
pub struct WeaponData {
    pub id: i64,
    #[schema(example = 0)]
    pub to_hit_bonus: Option<i64>,
    pub damage_data: Vec<DamageData>,
    #[schema(example = 0)]
    pub n_of_potency_runes: i64,
    #[schema(example = 0)]
    pub n_of_striking_runes: i64,
    pub property_runes: Vec<String>,
    #[schema(example = 10)]
    pub range: Option<i64>,
    pub reload: Option<String>,
    pub weapon_type: WeaponTypeEnum,
    #[schema(example = 0)]
    pub splash_dmg: Option<i64>,
}

impl<'r> FromRow<'r, PgRow> for Weapon {
    fn from_row(row: &'r PgRow) -> Result<Self, Error> {
        let item_core = Item::from_row(row)?;
        let wp_type = WeaponTypeEnum::from_str(row.try_get("weapon_type")?);
        Ok(Self {
            item_core,
            weapon_data: WeaponData {
                id: row.try_get("weapon_id")?,
                to_hit_bonus: row
                    .try_get::<Option<i32>, _>("to_hit_bonus")
                    .ok()
                    .flatten()
                    .map(|v| v as i64),
                n_of_potency_runes: row.try_get::<i32, _>("n_of_potency_runes")? as i64,
                n_of_striking_runes: row.try_get::<i32, _>("n_of_striking_runes")? as i64,
                property_runes: vec![],
                range: row
                    .try_get::<Option<i32>, _>("range")
                    .ok()
                    .flatten()
                    .map(|v| v as i64),
                reload: row.try_get("reload")?,
                weapon_type: wp_type.unwrap_or(WeaponTypeEnum::Melee),
                damage_data: vec![],
                splash_dmg: row
                    .try_get::<Option<i32>, _>("splash_dmg")
                    .ok()
                    .flatten()
                    .map(|v| v as i64),
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
                x.clone().dice.map_or(0, |dice| dice.get_avg_dmg(b))
            })
            .sum()
    }
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq, Debug)]
pub struct DamageData {
    pub id: i64,
    #[schema(example = 10)]
    pub bonus_dmg: i64,
    pub dmg_type: Option<String>,
    pub dice: Option<Dice>,
}

impl<'r> FromRow<'r, PgRow> for DamageData {
    fn from_row(row: &'r PgRow) -> Result<Self, Error> {
        Ok(Self {
            id: row.try_get("id")?,
            bonus_dmg: row.try_get::<i32, _>("bonus_dmg")? as i64,
            dmg_type: row.try_get("dmg_type").ok(),
            dice: Dice::from_optional_dice_number_and_size(
                row.try_get::<Option<i32>, _>("number_of_dice")
                    .ok()
                    .flatten()
                    .map(|v| v as i16),
                row.try_get::<Option<i32>, _>("die_size")
                    .ok()
                    .flatten()
                    .map(|v| v as i16),
            ),
        })
    }
}
