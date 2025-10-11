use crate::db::json_fetcher::get_names_from_json;
use crate::models::npc::ancestry_enum::{PfAncestry, SfAncestry};
use crate::models::npc::culture_enum::{PfCulture, SfCulture};
use crate::models::npc::gender_enum::Gender;
use crate::models::shared::game_system_enum::GameSystem;
use crate::services::shared::npc_service::{get_ancestry_name_builder, get_culture_name_builder};
use crate::traits::filter::Filter;
use crate::traits::name_system::{NameOrigin, NameOriginFilter};
use crate::traits::origin::average_name_length::AverageNameLength;
use crate::traits::origin::context_size::ContextSize;
use crate::traits::origin::has_valid_genders::HasValidGenders;
use crate::traits::random_enum::RandomEnum;
use anyhow::bail;
use nanorand::{Rng, WyRand};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum::{Display, EnumCount, EnumIter, FromRepr, IntoEnumIterator};
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

impl Filter for PfNameOriginFilter {}

impl From<PfNameOriginFilter> for GameSystem {
    fn from(_: PfNameOriginFilter) -> Self {
        Self::Pathfinder
    }
}

impl NameOriginFilter for PfNameOriginFilter {
    type NameOriginType = PfNameOrigin;
    type AncestryType = PfAncestry;

    type CultureType = PfCulture;

    fn get_ancestries(&self) -> Option<Vec<Self::AncestryType>> {
        match self {
            Self::FromAncestry(ancestries) => Some(ancestries.clone().unwrap_or_default()),
            _ => None,
        }
    }

    fn get_cultures(&self) -> Option<Vec<Self::CultureType>> {
        match self {
            Self::FromCulture(cultures) => Some(cultures.clone().unwrap_or_default()),
            _ => None,
        }
    }

    fn to_name_origin(
        &self,
        c: Option<Self::CultureType>,
        a: Option<Self::AncestryType>,
    ) -> anyhow::Result<Self::NameOriginType> {
        if c.is_some() {
            Ok(PfNameOrigin::FromCulture(c))
        } else if a.is_some() {
            Ok(PfNameOrigin::FromAncestry(a))
        } else {
            bail!("Invalid parameters passed. At least one must contain a valid value")
        }
    }
}

impl From<PfAncestry> for PfNameOriginFilter {
    fn from(ancestry: PfAncestry) -> Self {
        Self::FromAncestry(Some(vec![ancestry]))
    }
}

impl From<PfCulture> for PfNameOriginFilter {
    fn from(culture: PfCulture) -> Self {
        Self::FromCulture(Some(vec![culture]))
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

impl Filter for SfNameOriginFilter {}

impl From<SfNameOriginFilter> for GameSystem {
    fn from(_: SfNameOriginFilter) -> Self {
        Self::Starfinder
    }
}

impl NameOriginFilter for SfNameOriginFilter {
    type NameOriginType = SfNameOrigin;
    type AncestryType = SfAncestry;

    type CultureType = SfCulture;

    fn get_ancestries(&self) -> Option<Vec<Self::AncestryType>> {
        match self {
            Self::FromAncestry(ancestries) => Some(ancestries.clone().unwrap_or_default()),
        }
    }

    fn get_cultures(&self) -> Option<Vec<Self::CultureType>> {
        None
    }

    fn to_name_origin(
        &self,
        _: Option<Self::CultureType>,
        a: Option<Self::AncestryType>,
    ) -> anyhow::Result<Self::NameOriginType> {
        if a.is_some() {
            Ok(SfNameOrigin::FromAncestry(a))
        } else {
            bail!("No valid cultures yet implemented for starfinder")
        }
    }
}

impl From<SfAncestry> for SfNameOriginFilter {
    fn from(ancestry: SfAncestry) -> Self {
        Self::FromAncestry(Some(vec![ancestry]))
    }
}

impl TryFrom<SfCulture> for SfNameOriginFilter {
    type Error = anyhow::Error;

    fn try_from(_: SfCulture) -> Result<Self, anyhow::Error> {
        bail!("Invalid conversion, cultures are not yet implemented!")
    }
}

/*














*/

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

impl ContextSize for SfNameOrigin {
    fn context_size(&self) -> usize {
        match self {
            Self::FromAncestry(ancestry) => ancestry.as_ref().map_or(2, |a| a.context_size()),
        }
    }
}

impl AverageNameLength for SfNameOrigin {
    fn get_average_name_length(&self) -> usize {
        match self {
            Self::FromAncestry(ancestry) => ancestry
                .as_ref()
                .map_or(10, |a| a.get_average_name_length()),
        }
    }
}

impl NameOrigin for SfNameOrigin {
    type AncestryType = SfAncestry;

    type CultureType = SfCulture;

    fn get_random_gender(&self) -> Gender {
        let valid_genders = match self {
            Self::FromAncestry(ancestry) => ancestry
                .as_ref()
                .map_or_else(|| Gender::iter().collect(), |a| a.get_valid_genders()),
        };
        Gender::filtered_random(valid_genders.as_slice())
    }
    fn get_random_ancestry() -> Self::AncestryType {
        SfAncestry::random()
    }

    fn get_ancestry(&self) -> Option<Self::AncestryType> {
        match self {
            Self::FromAncestry(a) => *a,
        }
    }

    fn get_culture(&self) -> Option<Self::CultureType> {
        None
    }

    fn get_name_builder(
        &self,
        json_path: &str,
    ) -> HashMap<(String, Gender), HashMap<String, Vec<char>>> {
        let names = get_names_from_json(json_path).unwrap();
        match self {
            Self::FromAncestry(_) => {
                get_ancestry_name_builder(names.sf_names.by_ancestry, GameSystem::Starfinder)
            }
        }
    }
}

impl From<SfAncestry> for SfNameOrigin {
    fn from(ancestry: SfAncestry) -> Self {
        Self::FromAncestry(Some(ancestry))
    }
}

impl TryFrom<SfCulture> for SfNameOrigin {
    type Error = anyhow::Error;

    fn try_from(_: SfCulture) -> Result<Self, Self::Error> {
        panic!("Cannot convert culture to SfNameOrigin");
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

impl AverageNameLength for PfNameOrigin {
    fn get_average_name_length(&self) -> usize {
        match self {
            Self::FromAncestry(ancestry) => ancestry
                .as_ref()
                .map_or(10, |a| a.get_average_name_length()),
            Self::FromCulture(culture) => {
                culture.as_ref().map_or(10, |c| c.get_average_name_length())
            }
        }
    }
}

impl ContextSize for PfNameOrigin {
    fn context_size(&self) -> usize {
        match self {
            Self::FromAncestry(ancestry) => ancestry.as_ref().map_or(2, |a| a.context_size()),
            Self::FromCulture(culture) => culture.as_ref().map_or(2, |c| c.context_size()),
        }
    }
}

impl NameOrigin for PfNameOrigin {
    type AncestryType = PfAncestry;

    type CultureType = PfCulture;

    fn get_random_gender(&self) -> Gender {
        let valid_genders = match self {
            Self::FromAncestry(ancestry) => ancestry
                .as_ref()
                .map_or_else(|| Gender::iter().collect(), |a| a.get_valid_genders()),
            _ => Gender::iter().collect(),
        };
        Gender::filtered_random(valid_genders.as_slice())
    }

    fn get_random_ancestry() -> Self::AncestryType {
        PfAncestry::random()
    }

    fn get_ancestry(&self) -> Option<Self::AncestryType> {
        match self {
            Self::FromAncestry(a) => a.clone(),
            _ => None,
        }
    }

    fn get_culture(&self) -> Option<Self::CultureType> {
        match self {
            Self::FromCulture(c) => c.clone(),
            _ => None,
        }
    }

    fn get_name_builder(
        &self,
        json_path: &str,
    ) -> HashMap<(String, Gender), HashMap<String, Vec<char>>> {
        let names = get_names_from_json(json_path).unwrap();
        match self {
            Self::FromAncestry(_) => {
                get_ancestry_name_builder(names.pf_names.by_ancestry, GameSystem::Pathfinder)
            }
            _ => get_culture_name_builder(names.pf_names.by_culture, GameSystem::Pathfinder),
        }
    }
}

impl From<PfAncestry> for PfNameOrigin {
    fn from(ancestry: PfAncestry) -> Self {
        Self::FromAncestry(Some(ancestry))
    }
}

impl From<PfCulture> for PfNameOrigin {
    fn from(culture: PfCulture) -> Self {
        Self::FromCulture(Some(culture))
    }
}
