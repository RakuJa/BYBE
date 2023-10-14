use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum CreatureFilter {
    Id,
    Level,
    Family,
    Alignment,
    Size,
    Rarity,
    Melee,
    Ranged,
    SpellCaster,
}
