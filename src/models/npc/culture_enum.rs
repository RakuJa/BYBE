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
pub enum Culture {
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

impl RandomEnum for Culture {
    fn from_repr(value: usize) -> Option<Self> {
        Self::from_repr(value)
    }
}

impl Culture {
    pub const fn get_default_order_size(&self) -> usize {
        match self {
            Self::Ulfen | Self::Taldan => 3,
            _ => 2,
        }
    }

    pub const fn get_default_name_length(&self) -> usize {
        match self {
            Self::Shoanti | Self::Kellid | Self::Varisian => 8,
            Self::Garund | Self::Kelesh | Self::Ulfen => 9,
            Self::Taldan => 12,
            Self::Tian => 20,
            Self::Mwangi => 10,
        }
    }
}
