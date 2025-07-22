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
pub enum Gender {
    Male,
    Female,
    #[default]
    NonBinary,
}

impl Gender {
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
}
