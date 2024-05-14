use crate::models::db::raw_trait::RawTrait;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum::{Display, EnumIter};
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
    EnumIter,
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

impl From<(&Vec<RawTrait>, bool)> for AlignmentEnum {
    fn from(tuple: (&Vec<RawTrait>, bool)) -> AlignmentEnum {
        // If remaster then it's always no alignment
        if tuple.1 {
            return AlignmentEnum::No;
        }
        let string_traits: Vec<String> = tuple
            .0
            .iter()
            .map(|x| x.name.clone().to_uppercase())
            .collect();
        let is_good = string_traits.contains(&"GOOD".to_string());
        let is_evil = string_traits.contains(&"EVIL".to_string());
        let is_chaos = string_traits.contains(&"CHAOTIC".to_string());
        let is_lawful = string_traits.contains(&"LAWFUL".to_string());
        if is_good {
            if is_chaos {
                return AlignmentEnum::Cg;
            }
            if is_lawful {
                return AlignmentEnum::Lg;
            }
            return AlignmentEnum::Ng;
        }
        if is_evil {
            if is_chaos {
                return AlignmentEnum::Ce;
            }
            if is_lawful {
                return AlignmentEnum::Le;
            }
            return AlignmentEnum::Ne;
        }
        AlignmentEnum::N
    }
}

impl From<String> for AlignmentEnum {
    fn from(value: String) -> Self {
        AlignmentEnum::from_str(value.as_str()).unwrap_or_default()
    }
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

impl AlignmentEnum {
    pub fn to_traits(&self) -> Vec<String> {
        match self {
            AlignmentEnum::Ce => {
                vec![String::from("chaotic"), String::from("evil")]
            }
            AlignmentEnum::Cn => {
                vec![String::from("chaotic"), String::from("neutral")]
            }
            AlignmentEnum::Cg => {
                vec![String::from("chaotic"), String::from("good")]
            }
            AlignmentEnum::Ne => {
                vec![String::from("neutral"), String::from("evil")]
            }
            AlignmentEnum::N => {
                vec![String::from("neutral")]
            }
            AlignmentEnum::Ng => {
                vec![String::from("neutral"), String::from("good")]
            }
            AlignmentEnum::Le => {
                vec![String::from("lawful"), String::from("evil")]
            }
            AlignmentEnum::Ln => {
                vec![String::from("lawful"), String::from("neutral")]
            }
            AlignmentEnum::Lg => {
                vec![String::from("lawful"), String::from("good")]
            }
            AlignmentEnum::No | AlignmentEnum::Any => {
                vec![]
            }
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
