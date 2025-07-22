use nanorand::Rng;
use nanorand::WyRand;
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

impl Culture {
    pub fn random() -> Self {
        Self::from_repr(WyRand::new().generate_range(0..Self::COUNT)).unwrap_or_default()
    }

    pub fn filtered_random(filter: &[Self]) -> Self {
        if filter.is_empty() {
            Self::random()
        } else {
            filter
                .get(WyRand::new().generate_range(0..filter.len()))
                .cloned()
                .unwrap_or_default()
        }
    }

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
