use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Serialize, Deserialize, Display, Eq, Hash, PartialEq, Ord, PartialOrd, Debug)]
pub enum AlignmentEnum {
    CE,
    CN,
    CG,
    NE,
    N,
    NG,
    LE,
    LN,
    LG,
    NO,  // no alignment
    ANY, // can be every alignment
}

#[derive(Serialize, Deserialize, Display, Eq, Hash, PartialEq, Ord, PartialOrd, Debug)]
pub enum RarityEnum {
    COMMON,
    UNCOMMON,
    RARE,
    UNIQUE,
    ANY,
}

#[derive(Serialize, Deserialize, Display, Eq, Hash, PartialEq, Ord, PartialOrd, Debug)]
pub enum SizeEnum {
    TINY,
    SMALL,
    MEDIUM,
    LARGE,
    HUGE,
    GARGANTUAN,
    ANY,
}

impl Default for AlignmentEnum {
    fn default() -> Self {
        AlignmentEnum::ANY
    }
}

impl Default for RarityEnum {
    fn default() -> Self {
        RarityEnum::ANY
    }
}

impl Default for SizeEnum {
    fn default() -> Self {
        SizeEnum::ANY
    }
}

impl Clone for AlignmentEnum {
    fn clone(&self) -> AlignmentEnum {
        match self {
            AlignmentEnum::CE => AlignmentEnum::CE,
            AlignmentEnum::CN => AlignmentEnum::CN,
            AlignmentEnum::CG => AlignmentEnum::CG,
            AlignmentEnum::NE => AlignmentEnum::NE,
            AlignmentEnum::N => AlignmentEnum::N,
            AlignmentEnum::NG => AlignmentEnum::NG,
            AlignmentEnum::LE => AlignmentEnum::LE,
            AlignmentEnum::LN => AlignmentEnum::LN,
            AlignmentEnum::LG => AlignmentEnum::LG,
            AlignmentEnum::NO => AlignmentEnum::NO,
            AlignmentEnum::ANY => AlignmentEnum::ANY,
        }
    }
}

impl Clone for RarityEnum {
    fn clone(&self) -> RarityEnum {
        match self {
            RarityEnum::COMMON => RarityEnum::COMMON,
            RarityEnum::UNCOMMON => RarityEnum::UNCOMMON,
            RarityEnum::RARE => RarityEnum::RARE,
            RarityEnum::UNIQUE => RarityEnum::UNIQUE,
            RarityEnum::ANY => RarityEnum::ANY,
        }
    }
}

impl Clone for SizeEnum {
    fn clone(&self) -> SizeEnum {
        match self {
            SizeEnum::TINY => SizeEnum::TINY,
            SizeEnum::SMALL => SizeEnum::SMALL,
            SizeEnum::MEDIUM => SizeEnum::MEDIUM,
            SizeEnum::LARGE => SizeEnum::LARGE,
            SizeEnum::HUGE => SizeEnum::HUGE,
            SizeEnum::GARGANTUAN => SizeEnum::GARGANTUAN,
            SizeEnum::ANY => SizeEnum::ANY,
        }
    }
}
