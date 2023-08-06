use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Serialize, Deserialize, Display, Eq, Hash, PartialEq, Ord, PartialOrd, Debug, Default)]
pub enum AlignmentEnum {
    Ce,
    Cn,
    Cg,
    Ne,
    N,
    Ng,
    Le,
    Ln,
    Lg,
    No, // no alignment
    #[default]
    Any, // can be every alignment
}

#[derive(Serialize, Deserialize, Display, Eq, Hash, PartialEq, Ord, PartialOrd, Debug, Default)]
pub enum RarityEnum {
    Common,
    Uncommon,
    Rare,
    Unique,
    #[default]
    Any,
}

#[derive(Serialize, Deserialize, Display, Eq, Hash, PartialEq, Ord, PartialOrd, Debug, Default)]
pub enum SizeEnum {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
    Gargantuan,
    #[default]
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
