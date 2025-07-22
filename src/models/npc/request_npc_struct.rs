use crate::models::npc::ancestry_enum::Ancestry;
use crate::models::npc::class_enum::Class;
use crate::models::npc::culture_enum::Culture;
use crate::models::npc::gender_enum::Gender;
use crate::models::npc::job_enum::Job;
use crate::models::routers_validator_structs::LevelData;
use crate::traits::random_enum::RandomEnum;
use nanorand::{Rng, WyRand};
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashSet;
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
pub enum NameOrigin {
    FromAncestry(Option<Ancestry>),
    FromCulture(Option<Culture>),
}

impl RandomEnum for NameOrigin {
    fn from_repr(value: usize) -> Option<Self> {
        Self::from_repr(value)
    }

    fn random() -> Self {
        match Self::from_repr(WyRand::new().generate_range(0..Self::COUNT)).unwrap_or_default() {
            Self::FromAncestry(_) => Self::FromAncestry(Some(Ancestry::random())),
            Self::FromCulture(_) => Self::FromCulture(Some(Culture::random())),
        }
    }
}

impl Default for NameOrigin {
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
pub enum NameOriginFilter {
    FromAncestry(Option<Vec<Ancestry>>),
    FromCulture(Option<Vec<Culture>>),
}

impl Default for NameOriginFilter {
    fn default() -> Self {
        Self::FromAncestry(None)
    }
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug, Default)]
pub struct RandomNpcData {
    pub name_origin_filter: NameOriginFilter,
    pub gender_filter: Option<Vec<Gender>>,
    pub class_filter: Option<Vec<Class>>,
    pub level_filter: Option<LevelData>,
    pub job_filter: Option<Vec<Job>>,
    pub name_max_length: Option<usize>,
    pub generate_nickname: Option<bool>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct AncestryData {
    pub ancestry: Ancestry,
    pub valid_genders: Vec<Gender>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct RandomNameData {
    #[schema(minimum = 2, maximum = 40, example = 10)]
    pub name_max_length: Option<usize>,
    #[schema(minimum = 1, maximum = 100, example = 10)]
    pub max_n_of_names: Option<usize>,
    pub origin: NameOrigin,
    pub gender: Option<Gender>,
}

impl Default for RandomNameData {
    fn default() -> Self {
        let ancestry = Ancestry::random();
        let gender = Gender::filtered_random(&Ancestry::get_valid_genders(&ancestry));
        Self {
            name_max_length: Some(Ancestry::get_default_name_length(&ancestry)),
            max_n_of_names: Some(10),
            origin: NameOrigin::FromAncestry(Some(ancestry)),
            gender: Some(gender),
        }
    }
}

impl RandomNpcData {
    pub fn is_valid(&self) -> bool {
        self.level_filter
            .as_ref()
            .is_none_or(LevelData::is_data_valid)
            && match &self.name_origin_filter {
                NameOriginFilter::FromAncestry(ancestry_filter) => {
                    if let Some(g_filter) = self.gender_filter.clone()
                        && let Some(a_list) = ancestry_filter.clone()
                    {
                        // we check for ancestries/genders conflicts
                        let a_genders: HashSet<_> = a_list
                            .iter()
                            .flat_map(Ancestry::get_valid_genders)
                            .collect();
                        a_genders.iter().any(|a| g_filter.contains(a))
                    } else {
                        true
                    }
                }
                NameOriginFilter::FromCulture(_) => true,
            }
    }
}

impl RandomNameData {
    pub fn is_valid(&self) -> bool {
        match &self.origin {
            NameOrigin::FromAncestry(and) => {
                if let Some(g) = &self.gender
                    && let Some(a) = and
                {
                    a.get_valid_genders().contains(g)
                } else {
                    true
                }
            }
            NameOrigin::FromCulture(_) => true,
        }
    }
}
