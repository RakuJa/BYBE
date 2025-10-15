use crate::traits::origin::average_name_length::AverageNameLength;
use crate::traits::origin::context_size::ContextSize;
use crate::traits::origin::culture::Culture;
use crate::traits::random_enum::RandomEnum;
use serde::{Deserialize, Serialize};
use strum::EnumCount;
use strum::EnumIter;
use strum::FromRepr;
use strum::{Display, EnumString};
use utoipa::ToSchema;

#[derive(
    Serialize,
    FromRepr,
    Deserialize,
    EnumCount,
    EnumString,
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
pub enum PfCulture {
    #[default]
    Garund,
    Kelesh,
    Kellid,
    Mwangi,
    Shoanti,
    Taldan,
    Tian,
    Ulfen,
    Varisian,
}

impl RandomEnum for PfCulture {
    fn from_repr(value: usize) -> Option<Self> {
        Self::from_repr(value)
    }
}

impl ContextSize for PfCulture {
    fn context_size(&self) -> usize {
        match self {
            Self::Ulfen | Self::Taldan | Self::Tian => 3,
            _ => 2,
        }
    }
}

impl AverageNameLength for PfCulture {
    fn get_average_name_length(&self) -> usize {
        match self {
            Self::Shoanti | Self::Kellid | Self::Varisian => 8,
            Self::Garund | Self::Kelesh | Self::Ulfen => 9,
            Self::Taldan => 12,
            Self::Tian => 20,
            Self::Mwangi => 15,
        }
    }
}

impl Culture for PfCulture {}

#[derive(
    Serialize,
    FromRepr,
    Deserialize,
    EnumCount,
    EnumString,
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
pub enum SfCulture {
    #[default]
    Space,
}

impl RandomEnum for SfCulture {
    fn from_repr(value: usize) -> Option<Self> {
        Self::from_repr(value)
    }
}

impl ContextSize for SfCulture {
    fn context_size(&self) -> usize {
        2
    }
}

impl AverageNameLength for SfCulture {
    fn get_average_name_length(&self) -> usize {
        10
    }
}

impl Culture for SfCulture {}
