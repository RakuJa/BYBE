use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum::Display;
use utoipa::ToSchema;

#[derive(
    Serialize, Deserialize, ToSchema, Display, Eq, Hash, PartialEq, Ord, PartialOrd, Default,
)]
pub enum AlignmentEnum {
    #[strum(to_string = "CE")]
    #[serde(rename = "CE")]
    Ce,
    #[strum(to_string = "CN")]
    #[serde(rename = "CN")]
    Cn,
    #[strum(to_string = "CG")]
    #[serde(rename = "CG")]
    Cg,
    #[strum(to_string = "NE")]
    #[serde(rename = "NE")]
    Ne,
    #[serde(alias = "n")]
    #[strum(to_string = "N")]
    N,
    #[strum(to_string = "NG")]
    #[serde(rename = "NG")]
    Ng,
    #[strum(to_string = "LE")]
    #[serde(rename = "LE")]
    Le,
    #[strum(to_string = "LN")]
    #[serde(rename = "LN")]
    Ln,
    #[strum(to_string = "LG")]
    #[serde(rename = "LG")]
    Lg,
    #[strum(to_string = "No Alignment")]
    #[serde(rename = "No Alignment")]
    #[default]
    No, // no alignment
    #[strum(to_string = "Any")]
    #[serde(rename = "Any")]
    Any, // can be every alignment
}

#[derive(
    Serialize, Deserialize, ToSchema, Display, Eq, Hash, PartialEq, Ord, PartialOrd, Default,
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
    Serialize, Deserialize, ToSchema, Display, Eq, Hash, PartialEq, Ord, PartialOrd, Default,
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
#[derive(
    Serialize, Deserialize, ToSchema, Display, Eq, Hash, PartialEq, Ord, PartialOrd, Default,
)]
pub enum CreatureTypeEnum {
    #[default]
    #[serde(alias = "monster", alias = "MONSTER")]
    #[strum(to_string = "Monster")]
    #[serde(rename = "Monster")]
    Monster,
    #[serde(alias = "npc", alias = "NPC")]
    #[strum(to_string = "NPC")]
    #[serde(rename = "NPC")]
    Npc,
}

#[derive(
    Serialize, Deserialize, ToSchema, Display, Eq, Hash, PartialEq, Ord, PartialOrd, Default,
)]
pub enum CreatureVariant {
    Weak,
    Elite,
    #[default]
    Base,
}

impl FromStr for AlignmentEnum {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "CE" => Ok(AlignmentEnum::Ce),
            "CN" => Ok(AlignmentEnum::Cn),
            "CG" => Ok(AlignmentEnum::Cg),
            "NE" => Ok(AlignmentEnum::Ne),
            "N" => Ok(AlignmentEnum::N),
            "NG" => Ok(AlignmentEnum::Ng),
            "LE" => Ok(AlignmentEnum::Le),
            "LN" => Ok(AlignmentEnum::Ln),
            "LG" => Ok(AlignmentEnum::Lg),
            "ANY" => Ok(AlignmentEnum::Any),
            _ => Ok(AlignmentEnum::No),
        }
    }
}

impl FromStr for CreatureTypeEnum {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Monster" => Ok(CreatureTypeEnum::Monster),
            "NPC" => Ok(CreatureTypeEnum::Npc),
            "Npc" => Ok(CreatureTypeEnum::Npc),
            "npc" => Ok(CreatureTypeEnum::Npc),
            _ => Ok(CreatureTypeEnum::Monster),
        }
    }
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

impl FromStr for RarityEnum {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "COMMON" => Ok(RarityEnum::Common),
            "common" => Ok(RarityEnum::Common),
            "UNCOMMON" => Ok(RarityEnum::Uncommon),
            "uncommon" => Ok(RarityEnum::Uncommon),
            "RARE" => Ok(RarityEnum::Rare),
            "rare" => Ok(RarityEnum::Rare),
            "UNIQUE" => Ok(RarityEnum::Unique),
            "unique" => Ok(RarityEnum::Unique),
            _ => Err(()),
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

impl FromStr for SizeEnum {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "TINY" => Ok(SizeEnum::Tiny),
            "tiny" => Ok(SizeEnum::Tiny),
            "SMALL" => Ok(SizeEnum::Small),
            "small" => Ok(SizeEnum::Small),
            "MEDIUM" => Ok(SizeEnum::Medium),
            "medium" => Ok(SizeEnum::Medium),
            "LARGE" => Ok(SizeEnum::Large),
            "large" => Ok(SizeEnum::Large),
            "HUGE" => Ok(SizeEnum::Huge),
            "huge" => Ok(SizeEnum::Huge),
            "GARGANTUAN" => Ok(SizeEnum::Gargantuan),
            "gargantuan" => Ok(SizeEnum::Gargantuan),
            _ => Err(()),
        }
    }
}

impl Clone for CreatureTypeEnum {
    fn clone(&self) -> CreatureTypeEnum {
        match self {
            CreatureTypeEnum::Monster => CreatureTypeEnum::Monster,
            CreatureTypeEnum::Npc => CreatureTypeEnum::Npc,
        }
    }
}

impl Clone for CreatureVariant {
    fn clone(&self) -> CreatureVariant {
        match self {
            CreatureVariant::Elite => CreatureVariant::Elite,
            CreatureVariant::Weak => CreatureVariant::Weak,
            CreatureVariant::Base => CreatureVariant::Base,
        }
    }
}

pub fn creature_variant_to_level_delta(creature_variant: CreatureVariant) -> i8 {
    match creature_variant {
        CreatureVariant::Weak => -1,
        CreatureVariant::Elite => 1,
        CreatureVariant::Base => 0,
    }
}

pub fn creature_variant_to_cache_index(creature_variant: CreatureVariant) -> i32 {
    match creature_variant {
        CreatureVariant::Base => 0,
        CreatureVariant::Weak => 1,
        CreatureVariant::Elite => 2,
    }
}

pub fn creature_type_to_url_string(creature_type: &CreatureTypeEnum) -> &str {
    match creature_type {
        CreatureTypeEnum::Monster => "Monsters",
        CreatureTypeEnum::Npc => "NPCs",
    }
}
