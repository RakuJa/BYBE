use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use strum::EnumIter;
use utoipa::ToSchema;

#[derive(
    Serialize, Deserialize, ToSchema, Eq, Hash, PartialEq, Ord, PartialOrd, Type, EnumIter,
)]
pub enum ItemTypeEnum {
    #[serde(alias = "consumable", alias = "CONSUMABLE")]
    Consumable,
    #[serde(alias = "equipment", alias = "EQUIPMENT")]
    Equipment,
    #[serde(alias = "weapon", alias = "WEAPON")]
    Weapon,
    #[serde(alias = "armor", alias = "ARMOR")]
    Armor,
}

impl Clone for ItemTypeEnum {
    fn clone(&self) -> ItemTypeEnum {
        match self {
            ItemTypeEnum::Consumable => ItemTypeEnum::Consumable,
            ItemTypeEnum::Equipment => ItemTypeEnum::Equipment,
            ItemTypeEnum::Armor => ItemTypeEnum::Armor,
            ItemTypeEnum::Weapon => ItemTypeEnum::Weapon,
        }
    }
}

impl FromStr for ItemTypeEnum {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "CONSUMABLE" => Ok(ItemTypeEnum::Consumable),
            "EQUIPMENT" => Ok(ItemTypeEnum::Equipment),
            "WEAPON" => Ok(ItemTypeEnum::Weapon),
            "ARMOR" => Ok(ItemTypeEnum::Armor),
            _ => Err(()),
        }
    }
}

impl Display for ItemTypeEnum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ItemTypeEnum::Consumable => {
                write!(f, "consumable")
            }
            ItemTypeEnum::Equipment => {
                write!(f, "equipment")
            }
            ItemTypeEnum::Weapon => {
                write!(f, "weapon")
            }
            ItemTypeEnum::Armor => {
                write!(f, "armor")
            }
        }
    }
}

#[derive(
    Serialize, Deserialize, ToSchema, Eq, Hash, PartialEq, Ord, PartialOrd, Type, EnumIter,
)]
pub enum WeaponTypeEnum {
    #[serde(alias = "melee", alias = "MELEE")]
    Melee,
    #[serde(alias = "ranged", alias = "RANGED")]
    Ranged,
    #[serde(alias = "generic", alias = "GENERIC")]
    Generic,
}

impl Clone for WeaponTypeEnum {
    fn clone(&self) -> WeaponTypeEnum {
        match self {
            WeaponTypeEnum::Ranged => WeaponTypeEnum::Ranged,
            WeaponTypeEnum::Melee => WeaponTypeEnum::Melee,
            WeaponTypeEnum::Generic => WeaponTypeEnum::Generic,
        }
    }
}

impl FromStr for WeaponTypeEnum {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "RANGED" => Ok(WeaponTypeEnum::Ranged),
            "MELEE" => Ok(WeaponTypeEnum::Melee),
            "GENERIC" => Ok(WeaponTypeEnum::Generic),
            _ => Err(()),
        }
    }
}

impl Display for WeaponTypeEnum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WeaponTypeEnum::Melee => {
                write!(f, "melee")
            }
            WeaponTypeEnum::Ranged => {
                write!(f, "ranged")
            }
            WeaponTypeEnum::Generic => {
                write!(f, "generic")
            }
        }
    }
}
