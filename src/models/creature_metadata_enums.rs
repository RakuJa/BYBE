use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Serialize, Deserialize, Display, Eq, Hash, PartialEq, Ord, PartialOrd, Debug, Default)]
pub enum AlignmentEnum {
    #[serde(alias = "ce", alias = "CE")]
    Ce,
    #[serde(alias = "cn", alias = "CN")]
    Cn,
    #[serde(alias = "cg", alias = "CG")]
    Cg,
    #[serde(alias = "ne", alias = "NE")]
    Ne,
    #[serde(alias = "n")]
    N,
    #[serde(alias = "ng", alias = "NG")]
    Ng,
    #[serde(alias = "le", alias = "LE")]
    Le,
    #[serde(alias = "ln", alias = "LN")]
    Ln,
    #[serde(alias = "lg", alias = "LG")]
    Lg,
    #[serde(alias = "no", alias = "NO")]
    No, // no alignment
    #[default]
    #[serde(alias = "any", alias = "ANY")]
    Any, // can be every alignment
}

#[derive(Serialize, Deserialize, Display, Eq, Hash, PartialEq, Ord, PartialOrd, Debug, Default)]
pub enum RarityEnum {
    #[serde(alias = "common", alias = "COMMON")]
    Common,
    #[serde(alias = "uncommon", alias = "UNCOMMON")]
    Uncommon,
    #[serde(alias = "rare", alias = "RARE")]
    Rare,
    #[serde(alias = "unique", alias = "UNIQUE")]
    Unique,
    #[default]
    #[serde(alias = "any", alias = "ANY")]
    Any,
}

#[derive(Serialize, Deserialize, Display, Eq, Hash, PartialEq, Ord, PartialOrd, Debug, Default)]
pub enum SizeEnum {
    #[serde(alias = "tiny", alias = "TINY")]
    Tiny,
    #[serde(alias = "small", alias = "SMALL")]
    Small,
    #[serde(alias = "medium", alias = "MEDIUM")]
    Medium,
    #[serde(alias = "large", alias = "LARGE")]
    Large,
    #[serde(alias = "huge", alias = "HUGE")]
    Huge,
    #[serde(alias = "gargantuan", alias = "GARGANTUAN")]
    Gargantuan,
    #[default]
    #[serde(alias = "any", alias = "ANY")]
    Any,
}

impl Clone for AlignmentEnum {
    fn clone(&self) -> AlignmentEnum {
        match self {
            AlignmentEnum::Ce => AlignmentEnum::Ce,
            AlignmentEnum::Cn => AlignmentEnum::Cn,
            AlignmentEnum::Cg => AlignmentEnum::Cg,
            AlignmentEnum::Ne => AlignmentEnum::Ne,
            AlignmentEnum::N => AlignmentEnum::N,
            AlignmentEnum::Ng => AlignmentEnum::Ng,
            AlignmentEnum::Le => AlignmentEnum::Le,
            AlignmentEnum::Ln => AlignmentEnum::Ln,
            AlignmentEnum::Lg => AlignmentEnum::Lg,
            AlignmentEnum::No => AlignmentEnum::No,
            AlignmentEnum::Any => AlignmentEnum::Any,
        }
    }
}

impl Clone for RarityEnum {
    fn clone(&self) -> RarityEnum {
        match self {
            RarityEnum::Common => RarityEnum::Common,
            RarityEnum::Uncommon => RarityEnum::Uncommon,
            RarityEnum::Rare => RarityEnum::Rare,
            RarityEnum::Unique => RarityEnum::Unique,
            RarityEnum::Any => RarityEnum::Any,
        }
    }
}

impl Clone for SizeEnum {
    fn clone(&self) -> SizeEnum {
        match self {
            SizeEnum::Tiny => SizeEnum::Tiny,
            SizeEnum::Small => SizeEnum::Small,
            SizeEnum::Medium => SizeEnum::Medium,
            SizeEnum::Large => SizeEnum::Large,
            SizeEnum::Huge => SizeEnum::Huge,
            SizeEnum::Gargantuan => SizeEnum::Gargantuan,
            SizeEnum::Any => SizeEnum::Any,
        }
    }
}
