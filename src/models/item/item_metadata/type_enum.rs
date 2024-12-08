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
    #[serde(alias = "shield", alias = "SHIELD")]
    Shield,
}

impl ItemTypeEnum {
    pub fn to_db_main_table_name(&self) -> String {
        format!("{}_TABLE", self.to_db_table_name())
    }

    pub fn to_db_association_table_name(&self) -> String {
        format!("{}_CREATURE_ASSOCIATION_TABLE", self.to_db_table_name())
    }

    /// Utility method to reduce code redundancy.
    /// It returns the generic table name of the given item type.
    fn to_db_table_name(&self) -> String {
        String::from(match self {
            Self::Consumable | Self::Equipment => "ITEM",
            Self::Weapon => "WEAPON",
            Self::Armor => "ARMOR",
            Self::Shield => "SHIELD",
        })
    }
}

impl Clone for ItemTypeEnum {
    fn clone(&self) -> Self {
        match self {
            Self::Consumable => Self::Consumable,
            Self::Equipment => Self::Equipment,
            Self::Armor => Self::Armor,
            Self::Weapon => Self::Weapon,
            Self::Shield => Self::Shield,
        }
    }
}

impl FromStr for ItemTypeEnum {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "CONSUMABLE" => Ok(Self::Consumable),
            "EQUIPMENT" => Ok(Self::Equipment),
            "WEAPON" => Ok(Self::Weapon),
            "ARMOR" => Ok(Self::Armor),
            "SHIELD" => Ok(Self::Shield),
            _ => Err(()),
        }
    }
}

impl Display for ItemTypeEnum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Consumable => {
                write!(f, "consumable")
            }
            Self::Equipment => {
                write!(f, "equipment")
            }
            Self::Weapon => {
                write!(f, "weapon")
            }
            Self::Armor => {
                write!(f, "armor")
            }
            Self::Shield => {
                write!(f, "shield")
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
    fn clone(&self) -> Self {
        match self {
            Self::Ranged => Self::Ranged,
            Self::Melee => Self::Melee,
            Self::Generic => Self::Generic,
        }
    }
}

impl FromStr for WeaponTypeEnum {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "RANGED" => Ok(Self::Ranged),
            "MELEE" => Ok(Self::Melee),
            "GENERIC" => Ok(Self::Generic),
            _ => Err(()),
        }
    }
}

impl Display for WeaponTypeEnum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Melee => {
                write!(f, "melee")
            }
            Self::Ranged => {
                write!(f, "ranged")
            }
            Self::Generic => {
                write!(f, "generic")
            }
        }
    }
}
