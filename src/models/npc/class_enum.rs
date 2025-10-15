use crate::traits::class_enum::ClassEnum;
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

impl ClassEnum for PfClass {}

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

impl ClassEnum for SfClass {}
