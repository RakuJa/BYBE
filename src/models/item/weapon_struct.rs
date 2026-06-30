use crate::models::db::pg_type_helper::{get_i32_as_i64, get_opt_i32_as_i16, get_opt_i32_as_i64};
use crate::models::item::item_metadata::type_enum::WeaponTypeEnum;
use crate::models::item::item_struct::Item;
use crate::models::routers_validator_structs::Dice;
use crate::models::shared::action::Action;
use crate::models::shared::range_data::RangeData;
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
    pub range: Option<RangeData>,
    pub reload: Option<String>,
    pub weapon_type: WeaponTypeEnum,
    #[schema(example = 0)]
    pub splash_dmg: Option<i64>,
    pub attack_effects: Vec<Action>,
}

impl<'r> FromRow<'r, PgRow> for Weapon {
    fn from_row(row: &'r PgRow) -> Result<Self, Error> {
        let item_core = Item::from_row(row)?;
        let wp_type = WeaponTypeEnum::from_str(row.try_get("weapon_type")?);
        Ok(Self {
            item_core,
            weapon_data: WeaponData {
                id: row.try_get("weapon_id")?,
                to_hit_bonus: get_opt_i32_as_i64(row, "to_hit_bonus"),
                n_of_potency_runes: get_i32_as_i64(row, "n_of_potency_runes")?,
                n_of_striking_runes: get_i32_as_i64(row, "n_of_striking_runes")?,
                property_runes: vec![],
                range: RangeData::from_row(row).ok(),
                reload: row.try_get("reload")?,
                weapon_type: wp_type.unwrap_or(WeaponTypeEnum::Melee),
                damage_data: vec![],
                splash_dmg: get_opt_i32_as_i64(row, "splash_dmg"),
                attack_effects: vec![],
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
            bonus_dmg: get_i32_as_i64(row, "bonus_dmg")?,
            dmg_type: row.try_get("dmg_type").ok(),
            dice: Dice::from_optional_dice_number_and_size(
                get_opt_i32_as_i16(row, "number_of_dice"),
                get_opt_i32_as_i16(row, "die_size"),
            ),
        })
    }
}
