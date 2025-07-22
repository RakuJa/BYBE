use nanorand::Rng;
use nanorand::WyRand;
use serde::{Deserialize, Serialize};
use strum::Display;
use strum::EnumCount;
use strum::EnumIter;
use strum::FromRepr;
use strum::IntoEnumIterator;
use utoipa::ToSchema;

use crate::models::npc::gender_enum::Gender;
use crate::traits::random_enum::RandomEnum;

#[derive(
    Serialize,
    FromRepr,
    Deserialize,
    EnumCount,
    Default,
    ToSchema,
    EnumIter,
    Clone,
    Eq,
    PartialEq,
    Hash,
    Display,
    Debug,
)]
pub enum Ancestry {
    // Common
    Dwarf,
    Elf,
    Gnome,
    Goblin,
    Halfling,
    #[default]
    Human,
    Leshy,
    Orc,
    // Uncommon
    /*
    Athamaru,
    Azarketi,
    Catfolk,
    Centaur,
    Fetchling,
    Hobgoblin,
    Kholo,
    Kitsune,
    Kobold,
    Lizardfolk,
    Merfolk,
    Minotaur,
    Nagaji,
    Ratfolk,
    Samsaran,
    Tanuki,
    Tengu,
    Tripkee,
    Vanara,
    Wayang,
    // Rare
    Anadi,
    Android,
    Automaton,
    AwakenedAnimal,
    Conrasu,
    Fleshwarp,
    Ghoran,
    Goloma,
    Kashrishi,
    Poppet,
    Sarangay,
    Shisk,
    Shoony,
    Skeleton,
    Sprite,
    Strix,
    Surki,
    Vishkanya,
    Yaksha,
    Yaoguai,
    */
}

impl RandomEnum for Ancestry {
    fn random() -> Self {
        Self::from_repr(WyRand::new().generate_range(0..Self::COUNT)).unwrap_or_default()
    }
}

impl Ancestry {
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

    pub fn get_valid_genders(&self) -> Vec<Gender> {
        match self {
            Self::Leshy => vec![Gender::NonBinary],
            _ => Gender::iter().collect(),
        }
    }

    pub const fn get_default_name_length(&self) -> usize {
        match self {
            Self::Leshy => 30,
            _ => 15,
        }
    }

    pub const fn get_default_order_size(&self) -> usize {
        match self {
            Self::Leshy => 3,
            _ => 2,
        }
    }
}
