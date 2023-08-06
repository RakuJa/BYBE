use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum CreatureField {
    #[serde(alias = "id", alias = "ID")]
    Id,
    #[serde(alias = "name", alias = "NAME")]
    Name,
    #[serde(alias = "level", alias = "LEVEL")]
    Level,
    #[serde(alias = "hp", alias = "HP")]
    Hp,
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
}
