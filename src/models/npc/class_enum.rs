use crate::traits::random_enum::RandomEnum;
use serde::{Deserialize, Serialize};
use strum::EnumCount;
use strum::EnumIter;
use strum::FromRepr;
use utoipa::ToSchema;

#[derive(
    Serialize, FromRepr, Deserialize, EnumCount, Default, ToSchema, EnumIter, Clone, Debug,
)]
pub enum Class {
    Alchemist,
    Animist,
    Druid,
    Fighter,
    Barbarian,
    Bard,
    Investigator,
    Kineticist,
    Champion,
    #[default]
    Cleric,
    Magus,
    Monk,
    Oracle,
    Phychic,
    Swashbuckler,
    Thaumaturge,
    Ranger,
    Rogue,
    Witch,
    Wizard,
    Sorcerer,
    Summoner,
    // Uncommon
    Gunslinger,
    Inventor,
    // Rare
    Exemplar,
}

impl RandomEnum for Class {
    fn from_repr(value: usize) -> Option<Self> {
        Self::from_repr(value)
    }
}
