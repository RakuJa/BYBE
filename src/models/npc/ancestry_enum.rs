use crate::models::npc::gender_enum::Gender;
use crate::traits::origin::ancestry::Ancestry;
use crate::traits::origin::average_name_length::AverageNameLength;
use crate::traits::origin::context_size::ContextSize;
use crate::traits::origin::has_valid_genders::HasValidGenders;
use crate::traits::random_enum::RandomEnum;
use serde::{Deserialize, Serialize};
use strum::EnumCount;
use strum::EnumIter;
use strum::FromRepr;
use strum::IntoEnumIterator;
use strum::{Display, EnumString};
use utoipa::ToSchema;

#[derive(
    Serialize,
    FromRepr,
    Deserialize,
    EnumCount,
    EnumString,
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
pub enum PfAncestry {
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

impl Ancestry for PfAncestry {}

impl RandomEnum for PfAncestry {
    fn from_repr(value: usize) -> Option<Self> {
        Self::from_repr(value)
    }
}

impl HasValidGenders for PfAncestry {
    fn get_valid_genders(&self) -> Vec<Gender> {
        match self {
            Self::Leshy => vec![Gender::NonBinary],
            _ => Gender::iter().collect(),
        }
    }
}

impl ContextSize for PfAncestry {
    fn context_size(&self) -> usize {
        match self {
            Self::Leshy => 3,
            _ => 2,
        }
    }
}

impl AverageNameLength for PfAncestry {
    fn get_average_name_length(&self) -> usize {
        match self {
            Self::Leshy => 30,
            _ => 15,
        }
    }
}

#[derive(
    Serialize,
    FromRepr,
    Deserialize,
    EnumCount,
    EnumString,
    Default,
    ToSchema,
    EnumIter,
    Clone,
    Eq,
    PartialEq,
    Hash,
    Display,
    Debug,
    Copy,
)]
pub enum SfAncestry {
    // Common
    Android,
    Astrazoan,
    Barathu,
    Contemplative,
    Dragonkin,
    #[default]
    Human,
    Ikeshti,
    Kalo,
    Kasatha,
    Lashunta,
    Pahtra,
    Sarcesian,
    Shirren,
    Shobhad,
    Skittermander,
    Vesk,
    Vlaka,
    Ysoki,
    // Uncommon
    Khizar,
}

impl RandomEnum for SfAncestry {
    fn from_repr(value: usize) -> Option<Self> {
        Self::from_repr(value)
    }
}

impl HasValidGenders for SfAncestry {
    fn get_valid_genders(&self) -> Vec<Gender> {
        match self {
            Self::Android => vec![Gender::NonBinary],
            _ => Gender::iter().collect(),
        }
    }
}

impl ContextSize for SfAncestry {
    fn context_size(&self) -> usize {
        match self {
            Self::Khizar | Self::Contemplative => 3,
            _ => 2,
        }
    }
}

impl AverageNameLength for SfAncestry {
    fn get_average_name_length(&self) -> usize {
        match self {
            Self::Khizar | Self::Contemplative => 35,
            Self::Ysoki => 10,
            _ => 15,
        }
    }
}

impl Ancestry for SfAncestry {}
