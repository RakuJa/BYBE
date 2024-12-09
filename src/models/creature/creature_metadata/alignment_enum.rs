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

pub const ALIGNMENT_TRAITS: [&str; 4] = ["GOOD", "EVIL", "CHAOTIC", "LAWFUL"];

impl From<(&Vec<String>, bool)> for AlignmentEnum {
    fn from(tuple: (&Vec<String>, bool)) -> Self {
        // If remaster then it's always no alignment
        if tuple.1 {
            return Self::No;
        }
        let string_traits: Vec<String> = tuple.0.iter().map(|x| x.to_uppercase()).collect();
        let is_good = string_traits.contains(&"GOOD".to_string());
        let is_evil = string_traits.contains(&"EVIL".to_string());
        let is_chaos = string_traits.contains(&"CHAOTIC".to_string());
        let is_lawful = string_traits.contains(&"LAWFUL".to_string());
        if is_good {
            if is_chaos {
                return Self::Cg;
            }
            if is_lawful {
                return Self::Lg;
            }
            return Self::Ng;
        }
        if is_evil {
            if is_chaos {
                return Self::Ce;
            }
            if is_lawful {
                return Self::Le;
            }
            return Self::Ne;
        }
        if is_chaos {
            return Self::Cn;
        }
        if is_lawful {
            return Self::Ln;
        }
        Self::N
    }
}

impl AlignmentEnum {
    pub fn to_db_traits(&self) -> Vec<String> {
        match self {
            Self::Ce => {
                vec![String::from("CHAOTIC"), String::from("EVIL")]
            }
            Self::Cn => {
                vec![String::from("CHAOTIC")]
            }
            Self::Cg => {
                vec![String::from("CHAOTIC"), String::from("GOOD")]
            }
            Self::Ne => {
                vec![String::from("EVIL")]
            }
            Self::N | Self::No => {
                vec![]
            }
            Self::Ng => {
                vec![String::from("GOOD")]
            }
            Self::Le => {
                vec![String::from("LAWFUL"), String::from("EVIL")]
            }
            Self::Ln => {
                vec![String::from("LAWFUL")]
            }
            Self::Lg => {
                vec![String::from("LAWFUL"), String::from("GOOD")]
            }
            Self::Any => ALIGNMENT_TRAITS
                .to_vec()
                .iter()
                .map(ToString::to_string)
                .collect(),
        }
    }
}

impl From<String> for AlignmentEnum {
    fn from(value: String) -> Self {
        Self::from_str(value.as_str()).unwrap_or_default()
    }
}

impl From<&String> for AlignmentEnum {
    fn from(value: &String) -> Self {
        Self::from_str(value.as_str()).unwrap_or_default()
    }
}

impl FromStr for AlignmentEnum {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "CE" => Ok(Self::Ce),
            "CN" => Ok(Self::Cn),
            "CG" => Ok(Self::Cg),
            "NE" => Ok(Self::Ne),
            "N" => Ok(Self::N),
            "NG" => Ok(Self::Ng),
            "LE" => Ok(Self::Le),
            "LN" => Ok(Self::Ln),
            "LG" => Ok(Self::Lg),
            "ANY" => Ok(Self::Any),
            _ => Ok(Self::No),
        }
    }
}

impl Clone for AlignmentEnum {
    fn clone(&self) -> Self {
        match self {
            Self::Ce => Self::Ce,
            Self::Cn => Self::Cn,
            Self::Cg => Self::Cg,
            Self::Ne => Self::Ne,
            Self::N => Self::N,
            Self::Ng => Self::Ng,
            Self::Le => Self::Le,
            Self::Ln => Self::Ln,
            Self::Lg => Self::Lg,
            Self::No => Self::No,
            Self::Any => Self::Any,
        }
    }
}
