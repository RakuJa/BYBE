use crate::models::npc::gender_enum::Gender;
use crate::traits::random_enum::RandomEnum;
use serde::{Deserialize, Serialize};
use strum::Display;
use strum::EnumCount;
use strum::EnumIter;
use strum::FromRepr;
use strum::IntoEnumIterator;
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
    fn from_repr(value: usize) -> Option<Self> {
        Self::from_repr(value)
    }
}
impl Ancestry {
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
