use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;
use strum::{Display, EnumIter};
use utoipa::ToSchema;

#[derive(
    Serialize,
    Deserialize,
    ToSchema,
    Display,
    Eq,
    Hash,
    PartialEq,
    Ord,
    PartialOrd,
    Default,
    Type,
    EnumIter,
)]
pub enum CreatureTypeEnum {
    #[default]
    #[serde(alias = "monster", alias = "MONSTER")]
    Monster,
    #[serde(alias = "npc", alias = "NPC")]
    #[strum(to_string = "NPC")]
    #[serde(rename = "NPC")]
    Npc,
}

impl From<String> for CreatureTypeEnum {
    fn from(value: String) -> Self {
        Self::from_str(value.as_str()).unwrap_or_default()
    }
}

impl From<Option<String>> for CreatureTypeEnum {
    fn from(value: Option<String>) -> Self {
        Self::from_str(value.unwrap_or_default().as_str()).unwrap_or_default()
    }
}

impl From<&Option<String>> for CreatureTypeEnum {
    fn from(value: &Option<String>) -> Self {
        Self::from_str(value.clone().unwrap_or_default().as_str()).unwrap_or_default()
    }
}

impl FromStr for CreatureTypeEnum {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "NPC" => Ok(Self::Npc),
            _ => Ok(Self::Monster),
        }
    }
}

impl Clone for CreatureTypeEnum {
    fn clone(&self) -> Self {
        match self {
            Self::Monster => Self::Monster,
            Self::Npc => Self::Npc,
        }
    }
}
