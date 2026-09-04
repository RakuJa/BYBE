use serde::{Deserialize, Serialize};
use strum::Display;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Default, Eq, PartialEq, Hash, Clone, Display, Copy)]
pub enum ItemSortEnum {
    #[serde(alias = "id", alias = "ID")]
    Id,
    #[default]
    #[serde(alias = "name", alias = "NAME")]
    Name,
    #[serde(alias = "level", alias = "LEVEL")]
    Level,
    #[serde(alias = "trait", alias = "TRAIT")]
    Trait,
    #[serde(alias = "type", alias = "TYPE")]
    Type,
    #[serde(alias = "rarity", alias = "RARITY")]
    Rarity,
    #[serde(alias = "source", alias = "SOURCE")]
    Source,
}
