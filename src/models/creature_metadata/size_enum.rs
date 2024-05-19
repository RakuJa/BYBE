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
    fn clone(&self) -> SizeEnum {
        match self {
            SizeEnum::Tiny => SizeEnum::Tiny,
            SizeEnum::Small => SizeEnum::Small,
            SizeEnum::Medium => SizeEnum::Medium,
            SizeEnum::Large => SizeEnum::Large,
            SizeEnum::Huge => SizeEnum::Huge,
            SizeEnum::Gargantuan => SizeEnum::Gargantuan,
        }
    }
}

impl From<String> for SizeEnum {
    fn from(value: String) -> Self {
        SizeEnum::from_str(value.as_str()).unwrap_or_default()
    }
}

impl FromStr for SizeEnum {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "TINY" => Ok(SizeEnum::Tiny),
            "SMALL" => Ok(SizeEnum::Small),
            "MEDIUM" => Ok(SizeEnum::Medium),
            "LARGE" => Ok(SizeEnum::Large),
            "HUGE" => Ok(SizeEnum::Huge),
            "GARGANTUAN" => Ok(SizeEnum::Gargantuan),
            _ => Err(()),
        }
    }
}
