use crate::traits::random_enum::RandomEnum;
use serde::{Deserialize, Serialize};
use strum::EnumIter;
use strum::FromRepr;
use strum::{Display, EnumCount};
use utoipa::ToSchema;

#[derive(
    Serialize,
    FromRepr,
    Deserialize,
    EnumCount,
    Default,
    ToSchema,
    EnumIter,
    Clone,
    Debug,
    Eq,
    PartialEq,
    Hash,
    Display,
)]
pub enum PfClass {
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

impl RandomEnum for PfClass {
    fn from_repr(value: usize) -> Option<Self> {
        Self::from_repr(value)
    }
}

#[derive(
    Serialize,
    FromRepr,
    Deserialize,
    EnumCount,
    Default,
    ToSchema,
    EnumIter,
    Clone,
    Debug,
    Eq,
    PartialEq,
    Hash,
    Display,
)]
pub enum SfClass {
    Envoy,
    Mystic,
    Operative,
    Solarian,
    #[default]
    Soldier,
    Witchwarper,
}

impl RandomEnum for SfClass {
    fn from_repr(value: usize) -> Option<Self> {
        Self::from_repr(value)
    }
}

#[derive(
    Serialize,
    FromRepr,
    Deserialize,
    EnumCount,
    ToSchema,
    EnumIter,
    Clone,
    Eq,
    PartialEq,
    Hash,
    Display,
    Debug,
)]
pub enum ClassFilter {
    FromPf(Option<Vec<PfClass>>),
    FromSf(Option<Vec<SfClass>>),
}

impl RandomEnum for ClassFilter {
    fn from_repr(value: usize) -> Option<Self> {
        Self::from_repr(value)
    }
}

impl Default for ClassFilter {
    fn default() -> Self {
        Self::FromPf(None)
    }
}
