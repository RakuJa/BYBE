use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;
use strum::{Display, EnumIter};
use utoipa::ToSchema;

#[derive(
    Serialize, Deserialize, ToSchema, Display, Eq, Hash, PartialEq, Ord, PartialOrd, Type, EnumIter,
)]
pub enum ItemTypeEnum {
    #[serde(alias = "consumable", alias = "CONSUMABLE")]
    Consumable,
    #[serde(alias = "equipment", alias = "EQUIPMENT")]
    Equipment,
}

impl Clone for ItemTypeEnum {
    fn clone(&self) -> ItemTypeEnum {
        match self {
            ItemTypeEnum::Consumable => ItemTypeEnum::Consumable,
            ItemTypeEnum::Equipment => ItemTypeEnum::Equipment,
        }
    }
}

impl FromStr for ItemTypeEnum {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "CONSUMABLE" => Ok(ItemTypeEnum::Consumable),
            "EQUIPMENT" => Ok(ItemTypeEnum::Equipment),
            _ => Err(()),
        }
    }
}
