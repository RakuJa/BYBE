use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum::Display;
use utoipa::ToSchema;

#[derive(
    Serialize, Deserialize, ToSchema, Display, Eq, Hash, PartialEq, Ord, PartialOrd, Default,
)]
pub enum CreatureTypeEnum {
    #[default]
    #[serde(alias = "monster", alias = "MONSTER")]
    #[strum(to_string = "Monster")]
    #[serde(rename = "Monster")]
    Monster,
    #[serde(alias = "npc", alias = "NPC")]
    #[strum(to_string = "NPC")]
    #[serde(rename = "NPC")]
    Npc,
}

impl From<Option<String>> for CreatureTypeEnum {
    fn from(value: Option<String>) -> Self {
        CreatureTypeEnum::from_str(value.unwrap_or_default().as_str()).unwrap_or_default()
    }
}

impl FromStr for CreatureTypeEnum {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "MONSTER" => Ok(CreatureTypeEnum::Monster),
            "NPC" => Ok(CreatureTypeEnum::Npc),
            _ => Ok(CreatureTypeEnum::Monster),
        }
    }
}

impl Clone for CreatureTypeEnum {
    fn clone(&self) -> CreatureTypeEnum {
        match self {
            CreatureTypeEnum::Monster => CreatureTypeEnum::Monster,
            CreatureTypeEnum::Npc => CreatureTypeEnum::Npc,
        }
    }
}

impl CreatureTypeEnum {
    pub fn to_url_string(&self) -> &str {
        match self {
            CreatureTypeEnum::Monster => "Monsters",
            CreatureTypeEnum::Npc => "NPCs",
        }
    }
}
