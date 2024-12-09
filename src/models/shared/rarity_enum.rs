use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;
use strum::Display;
use utoipa::ToSchema;

#[derive(
    Serialize, Deserialize, ToSchema, Display, Eq, Hash, PartialEq, Ord, PartialOrd, Default, Type,
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
        Self::from_str(value.as_str()).unwrap_or_default()
    }
}

impl FromStr for RarityEnum {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "COMMON" => Ok(Self::Common),
            "UNCOMMON" => Ok(Self::Uncommon),
            "RARE" => Ok(Self::Rare),
            "UNIQUE" => Ok(Self::Unique),
            _ => Err(()),
        }
    }
}

impl Clone for RarityEnum {
    fn clone(&self) -> Self {
        match self {
            Self::Common => Self::Common,
            Self::Uncommon => Self::Uncommon,
            Self::Rare => Self::Rare,
            Self::Unique => Self::Unique,
        }
    }
}
