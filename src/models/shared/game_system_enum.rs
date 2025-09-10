use std::fmt::{Display, Formatter};

#[derive(PartialEq, Eq)]
pub enum GameSystem {
    Pathfinder,
    Starfinder,
}

impl Display for GameSystem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Pathfinder => "pf",
                Self::Starfinder => "sf",
            }
        )
    }
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
