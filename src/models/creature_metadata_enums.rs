use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use utoipa::ToSchema;

#[derive(
    Serialize,
    Deserialize,
    ToSchema,
    Display,
    Eq,
    Hash,
    PartialEq,
    Ord,
    PartialOrd,
    Default,
    EnumString,
)]
#[strum(serialize_all = "UPPERCASE")]
pub enum AlignmentEnum {
    #[serde(alias = "ce", alias = "Ce")]
    #[strum(to_string = "CE")]
    #[serde(rename = "CE")]
    Ce,
    #[serde(alias = "cn", alias = "Cn")]
    #[strum(to_string = "CN")]
    #[serde(rename = "CN")]
    Cn,
    #[serde(alias = "cg", alias = "Cg")]
    #[strum(to_string = "CG")]
    #[serde(rename = "CG")]
    Cg,
    #[serde(alias = "ne", alias = "Ne")]
    #[strum(to_string = "NE")]
    #[serde(rename = "NE")]
    Ne,
    #[serde(alias = "n")]
    #[strum(to_string = "N")]
    N,
    #[serde(alias = "ng", alias = "Ng")]
    #[strum(to_string = "NG")]
    #[serde(rename = "NG")]
    Ng,
    #[serde(alias = "le", alias = "Le")]
    #[strum(to_string = "LE")]
    #[serde(rename = "LE")]
    Le,
    #[serde(alias = "ln", alias = "LN")]
    #[strum(to_string = "LN")]
    #[serde(rename = "LN")]
    Ln,
    #[serde(alias = "lg", alias = "Lg")]
    #[strum(to_string = "LG")]
    #[serde(rename = "LG")]
    Lg,
    #[serde(alias = "no", alias = "NO")]
    #[strum(to_string = "No Alignment")]
    #[serde(rename = "No Alignment")]
    No, // no alignment
    #[default]
    #[serde(alias = "any", alias = "ANY")]
    Any, // can be every alignment
}

#[derive(
    Serialize,
    Deserialize,
    ToSchema,
    Display,
    Eq,
    Hash,
    PartialEq,
    Ord,
    PartialOrd,
    Default,
    EnumString,
)]
pub enum RarityEnum {
    #[default]
    #[serde(alias = "common", alias = "COMMON")]
    Common,
    #[serde(alias = "uncommon", alias = "UNCOMMON")]
    Uncommon,
    #[serde(alias = "rare", alias = "RARE")]
    Rare,
    #[serde(alias = "unique", alias = "UNIQUE")]
    Unique,
}

#[derive(
    Serialize,
    Deserialize,
    ToSchema,
    Display,
    Eq,
    Hash,
    PartialEq,
    Ord,
    PartialOrd,
    Default,
    EnumString,
)]
pub enum SizeEnum {
    #[serde(alias = "tiny", alias = "TINY")]
    Tiny,
    #[serde(alias = "small", alias = "SMALL")]
    Small,
    #[serde(alias = "medium", alias = "MEDIUM")]
    #[default]
    Medium,
    #[serde(alias = "large", alias = "LARGE")]
    Large,
    #[serde(alias = "huge", alias = "HUGE")]
    Huge,
    #[serde(alias = "gargantuan", alias = "GARGANTUAN")]
    Gargantuan,
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
        }
    }
}
