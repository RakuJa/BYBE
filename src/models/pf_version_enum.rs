use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};
use utoipa::ToSchema;

#[derive(
    Serialize, Deserialize, Eq, PartialEq, Hash, Default, ToSchema, Clone, EnumIter, Display,
)]
pub enum GameSystemVersionEnum {
    #[serde(alias = "legacy", alias = "LEGACY", alias = "2")]
    Legacy,
    #[serde(alias = "remaster", alias = "REMASTER", alias = "2.5")]
    Remaster,
    #[default]
    #[serde(alias = "any", alias = "ANY")]
    Any,
}

impl GameSystemVersionEnum {
    pub fn to_db_value(&self) -> Vec<String> {
        match self {
            // The db column is a boolean called "remaster" so we translate the enum to
            // FALSE if legacy, TRUE if remaster and TRUE, FALSE if both
            Self::Legacy => vec![String::from("FALSE")],
            Self::Remaster => vec![String::from("TRUE")],
            Self::Any => {
                vec![String::from("TRUE"), String::from("FALSE")]
            }
        }
    }
}
