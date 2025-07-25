use crate::traits::random_enum::RandomEnum;
use serde::{Deserialize, Serialize};
use strum::Display;
use strum::EnumCount;
use strum::EnumIter;
use strum::FromRepr;
use utoipa::ToSchema;

#[derive(
    Serialize,
    FromRepr,
    Deserialize,
    EnumCount,
    Default,
    ToSchema,
    EnumIter,
    Clone,
    Eq,
    PartialEq,
    Hash,
    Display,
    Debug,
)]
pub enum Gender {
    Male,
    Female,
    #[default]
    NonBinary,
}

impl RandomEnum for Gender {
    fn from_repr(value: usize) -> Option<Self> {
        Self::from_repr(value)
    }
}
