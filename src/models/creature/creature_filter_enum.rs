use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Clone)]
pub enum CreatureFilter {
    #[serde(alias = "level", alias = "LEVEL")]
    Level,
    #[serde(alias = "family", alias = "FAMILY")]
    Family,
    #[serde(alias = "alignment", alias = "ALIGNMENT")]
    Alignment,
    #[serde(alias = "size", alias = "SIZE")]
    Size,
    #[serde(alias = "rarity", alias = "RARITY")]
    Rarity,
    #[serde(alias = "is_melee", alias = "IS_MELEE")]
    Melee,
    #[serde(alias = "is_ranged", alias = "IS_RANGED")]
    Ranged,
    #[serde(alias = "is_spell_caster", alias = "IS_SPELL_CASTER")]
    SpellCaster,
    #[serde(alias = "sources", alias = "SOURCES")]
    Sources,
    #[serde(alias = "traits", alias = "TRAITS")]
    Traits,
    #[serde(alias = "creature_types", alias = "CREATURE_TYPES")]
    CreatureTypes,
    #[serde(alias = "creature_roles", alias = "CREATURE_ROLE")]
    CreatureRoles,
    PathfinderVersion,
}

impl fmt::Display for CreatureFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CreatureFilter::Level => {
                write!(f, "level")
            }
            CreatureFilter::Family => {
                write!(f, "family")
            }
            CreatureFilter::Size => {
                write!(f, "size")
            }
            CreatureFilter::Rarity => {
                write!(f, "rarity")
            }
            CreatureFilter::Melee => {
                write!(f, "is_melee")
            }
            CreatureFilter::Ranged => {
                write!(f, "is_ranged")
            }
            CreatureFilter::SpellCaster => {
                write!(f, "is_spell_caster")
            }
            CreatureFilter::Traits => {
                write!(f, "traits")
            }
            CreatureFilter::CreatureTypes => {
                write!(f, "cr_type")
            }
            CreatureFilter::CreatureRoles => {
                write!(f, "creature_roles")
            }
            CreatureFilter::Alignment => {
                write!(f, "alignment")
            }
            CreatureFilter::PathfinderVersion => {
                write!(f, "remaster")
            }
            CreatureFilter::Sources => {
                write!(f, "sources")
            }
        }
    }
}

#[derive(Default, Eq, PartialEq, Clone)]
pub struct FieldsUniqueValuesStruct {
    pub list_of_levels: Vec<String>,
    pub list_of_families: Vec<String>,
    pub list_of_traits: Vec<String>,
    pub list_of_sources: Vec<String>,
    pub list_of_sizes: Vec<String>,
    pub list_of_rarities: Vec<String>,
}
