use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};
use utoipa::ToSchema;

#[derive(
    Serialize, Deserialize, Eq, PartialEq, Hash, Default, ToSchema, Clone, EnumIter, Display,
)]
pub enum PathfinderVersionEnum {
    #[serde(alias = "legacy", alias = "LEGACY")]
    Legacy,
    #[serde(alias = "remaster", alias = "REMASTER")]
    Remaster,
    #[default]
    #[serde(alias = "any", alias = "ANY")]
    Any,
}

impl PathfinderVersionEnum {
    pub fn to_db_value(&self) -> Vec<String> {
        match self {
            // The db column is a boolean called "remaster" so we translate the enum to
            // FALSE if legacy, TRUE if remaster and TRUE, FALSE if both
            PathfinderVersionEnum::Legacy => vec![String::from("FALSE")],
            PathfinderVersionEnum::Remaster => vec![String::from("TRUE")],
            PathfinderVersionEnum::Any => {
                vec![String::from("TRUE"), String::from("FALSE")]
            }
        }
    }
}
