use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum CreatureFilter {
    Level,
    Family,
    Size,
    Rarity,
    Melee,
    Ranged,
    SpellCaster,
    Traits,
    Alignment,
    CreatureTypes,
    #[serde(alias = "creature_roles")]
    CreatureRoles,
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
                write!(f, "creature_types")
            }
            CreatureFilter::CreatureRoles => {
                write!(f, "creature_roles")
            }
            CreatureFilter::Alignment => {
                write!(f, "alignment")
            }
        }
    }
}
