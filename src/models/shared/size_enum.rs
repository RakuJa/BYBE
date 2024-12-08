use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;
use strum::Display;
use utoipa::ToSchema;

#[derive(
    Serialize, Deserialize, ToSchema, Display, Eq, Hash, PartialEq, Ord, PartialOrd, Default, Type,
)]
pub enum SizeEnum {
    #[serde(alias = "tiny", alias = "TINY")]
    Tiny,
    #[serde(alias = "small", alias = "SMALL")]
    Small,
    #[serde(alias = "medium", alias = "MEDIUM")]
    #[default]
    Medium,
    #[serde(alias = "large", alias = "LARGE")]
    Large,
    #[serde(alias = "huge", alias = "HUGE")]
    Huge,
    #[serde(alias = "gargantuan", alias = "GARGANTUAN")]
    Gargantuan,
}

impl Clone for SizeEnum {
    fn clone(&self) -> Self {
        match self {
            Self::Tiny => Self::Tiny,
            Self::Small => Self::Small,
            Self::Medium => Self::Medium,
            Self::Large => Self::Large,
            Self::Huge => Self::Huge,
            Self::Gargantuan => Self::Gargantuan,
        }
    }
}

impl From<String> for SizeEnum {
    fn from(value: String) -> Self {
        Self::from_str(value.as_str()).unwrap_or_default()
    }
}

impl FromStr for SizeEnum {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "TINY" => Ok(Self::Tiny),
            "SMALL" => Ok(Self::Small),
            "MEDIUM" => Ok(Self::Medium),
            "LARGE" => Ok(Self::Large),
            "HUGE" => Ok(Self::Huge),
            "GARGANTUAN" => Ok(Self::Gargantuan),
            _ => Err(()),
        }
    }
}
