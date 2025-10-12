use serde::{Deserialize, Serialize};
use strum::Display;
use utoipa::ToSchema;

#[derive(
    PartialEq, Eq, Clone, Copy, Deserialize, Serialize, ToSchema, Hash, Default, Debug, Display,
)]
pub enum GameSystem {
    #[default]
    #[serde(
        alias = "pathfinder",
        alias = "PATHFINDER",
        alias = "Pathfinder",
        alias = "pf",
        alias = "PF",
        alias = "Pf"
    )]
    #[strum(to_string = "pf")]
    #[serde(rename = "pf")]
    Pathfinder,
    #[serde(
        alias = "starfinder",
        alias = "STARFINDER",
        alias = "Starfinder",
        alias = "sf",
        alias = "SF",
        alias = "Sf"
    )]
    #[strum(to_string = "sf")]
    #[serde(rename = "sf")]
    Starfinder,
}

impl From<&GameSystem> for i64 {
    fn from(game_system: &GameSystem) -> Self {
        match game_system {
            GameSystem::Pathfinder => 0,
            GameSystem::Starfinder => 1,
        }
    }
}

impl From<GameSystem> for i64 {
    fn from(game_system: GameSystem) -> Self {
        Self::from(&game_system)
    }
}
