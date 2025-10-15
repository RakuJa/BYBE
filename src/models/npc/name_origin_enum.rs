use crate::models::npc::ancestry_enum::{PfAncestry, SfAncestry};
use crate::models::npc::culture_enum::PfCulture;
use crate::traits::random_enum::RandomEnum;
use nanorand::{Rng, WyRand};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIter, FromRepr};
use utoipa::ToSchema;

#[derive(
    Serialize,
    FromRepr,
    Deserialize,
    EnumCount,
    ToSchema,
    EnumIter,
    Clone,
    Eq,
    PartialEq,
    Hash,
    Display,
    Debug,
)]
pub enum PfNameOriginFilter {
    FromAncestry(Option<Vec<PfAncestry>>),
    FromCulture(Option<Vec<PfCulture>>),
}

impl RandomEnum for PfNameOriginFilter {
    fn from_repr(value: usize) -> Option<Self> {
        Self::from_repr(value)
    }
}

impl Default for PfNameOriginFilter {
    fn default() -> Self {
        Self::FromAncestry(None)
    }
}

#[derive(
    Serialize,
    FromRepr,
    Deserialize,
    EnumCount,
    ToSchema,
    EnumIter,
    Clone,
    Eq,
    PartialEq,
    Hash,
    Display,
    Debug,
)]
pub enum SfNameOriginFilter {
    FromAncestry(Option<Vec<SfAncestry>>),
}

impl RandomEnum for SfNameOriginFilter {
    fn from_repr(value: usize) -> Option<Self> {
        Self::from_repr(value)
    }
}

impl Default for SfNameOriginFilter {
    fn default() -> Self {
        Self::FromAncestry(None)
    }
}

#[derive(
    Serialize,
    FromRepr,
    Deserialize,
    EnumCount,
    ToSchema,
    EnumIter,
    Clone,
    Eq,
    PartialEq,
    Hash,
    Display,
    Debug,
)]
pub enum NameSystemOriginFilter {
    FromPf(Option<PfNameOriginFilter>),
    FromSf(Option<SfNameOriginFilter>),
}

impl RandomEnum for NameSystemOriginFilter {
    fn from_repr(value: usize) -> Option<Self> {
        Self::from_repr(value)
    }
}

impl Default for NameSystemOriginFilter {
    fn default() -> Self {
        Self::FromPf(None)
    }
}

#[derive(
    Serialize,
    FromRepr,
    Deserialize,
    EnumCount,
    ToSchema,
    EnumIter,
    Clone,
    Eq,
    PartialEq,
    Hash,
    Display,
    Debug,
)]
pub enum SfNameOrigin {
    FromAncestry(Option<SfAncestry>),
}

impl RandomEnum for SfNameOrigin {
    fn from_repr(value: usize) -> Option<Self> {
        Self::from_repr(value)
    }

    fn random() -> Self {
        match Self::from_repr(WyRand::new().generate_range(0..Self::COUNT)).unwrap_or_default() {
            Self::FromAncestry(_) => Self::FromAncestry(Some(SfAncestry::random())),
        }
    }
}

impl Default for SfNameOrigin {
    fn default() -> Self {
        Self::FromAncestry(None)
    }
}

#[derive(
    Serialize,
    FromRepr,
    Deserialize,
    EnumCount,
    ToSchema,
    EnumIter,
    Clone,
    Eq,
    PartialEq,
    Hash,
    Display,
    Debug,
)]
pub enum PfNameOrigin {
    FromAncestry(Option<PfAncestry>),
    FromCulture(Option<PfCulture>),
}

impl RandomEnum for PfNameOrigin {
    fn from_repr(value: usize) -> Option<Self> {
        Self::from_repr(value)
    }

    fn random() -> Self {
        match Self::from_repr(WyRand::new().generate_range(0..Self::COUNT)).unwrap_or_default() {
            Self::FromAncestry(_) => Self::FromAncestry(Some(PfAncestry::random())),
            Self::FromCulture(_) => Self::FromCulture(Some(PfCulture::random())),
        }
    }
}

impl Default for PfNameOrigin {
    fn default() -> Self {
        Self::FromAncestry(None)
    }
}

#[derive(
    Serialize,
    FromRepr,
    Deserialize,
    EnumCount,
    ToSchema,
    EnumIter,
    Clone,
    Eq,
    PartialEq,
    Hash,
    Display,
    Debug,
)]
pub enum NameSystemOrigin {
    FromPf(Option<PfNameOrigin>),
    FromSf(Option<SfNameOrigin>),
}

impl RandomEnum for NameSystemOrigin {
    fn from_repr(value: usize) -> Option<Self> {
        Self::from_repr(value)
    }

    fn random() -> Self {
        match Self::from_repr(WyRand::new().generate_range(0..Self::COUNT)).unwrap_or_default() {
            Self::FromPf(_) => Self::FromPf(Some(PfNameOrigin::random())),
            Self::FromSf(_) => Self::FromSf(Some(SfNameOrigin::random())),
        }
    }
}

impl Default for NameSystemOrigin {
    fn default() -> Self {
        Self::FromPf(None)
    }
}
