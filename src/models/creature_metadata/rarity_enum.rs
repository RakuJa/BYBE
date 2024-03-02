use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum::Display;
use utoipa::ToSchema;

#[derive(
    Serialize, Deserialize, ToSchema, Display, Eq, Hash, PartialEq, Ord, PartialOrd, Default,
)]
pub enum RarityEnum {
    #[default]
    #[serde(alias = "common", alias = "COMMON")]
    Common,
    #[serde(alias = "uncommon", alias = "UNCOMMON")]
    Uncommon,
    #[serde(alias = "rare", alias = "RARE")]
    Rare,
    #[serde(alias = "unique", alias = "UNIQUE")]
    Unique,
}

impl From<String> for RarityEnum {
    fn from(value: String) -> Self {
        RarityEnum::from_str(value.as_str()).unwrap_or_default()
    }
}

impl FromStr for RarityEnum {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "COMMON" => Ok(RarityEnum::Common),
            "UNCOMMON" => Ok(RarityEnum::Uncommon),
            "RARE" => Ok(RarityEnum::Rare),
            "UNIQUE" => Ok(RarityEnum::Unique),
            _ => Err(()),
        }
    }
}

impl Clone for RarityEnum {
    fn clone(&self) -> RarityEnum {
        match self {
            RarityEnum::Common => RarityEnum::Common,
            RarityEnum::Uncommon => RarityEnum::Uncommon,
            RarityEnum::Rare => RarityEnum::Rare,
            RarityEnum::Unique => RarityEnum::Unique,
        }
    }
}
