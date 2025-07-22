use crate::traits::random_enum::RandomEnum;
use nanorand::Rng;
use nanorand::WyRand;
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
    fn random() -> Self {
        Self::from_repr(WyRand::new().generate_range(0..Self::COUNT)).unwrap_or_default()
    }
}

impl Class {
    pub fn filtered_random(filter: &[Self]) -> Self {
        if filter.is_empty() {
            Self::random()
        } else {
            filter
                .get(WyRand::new().generate_range(0..filter.len()))
                .cloned()
                .unwrap_or_default()
        }
    }
}
