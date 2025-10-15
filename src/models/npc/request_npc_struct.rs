use crate::models::npc::ancestry_enum::{PfAncestry, SfAncestry};
use crate::models::npc::class_enum::ClassFilter;
use crate::models::npc::gender_enum::Gender;
use crate::models::npc::job_enum::JobFilter;
use crate::models::npc::name_origin_enum::{NameSystemOrigin, PfNameOrigin, SfNameOrigin};
use crate::models::npc::name_origin_enum::{
    NameSystemOriginFilter, PfNameOriginFilter, SfNameOriginFilter,
};
use crate::models::routers_validator_structs::LevelData;
use crate::models::shared::game_system_enum::GameSystem;
use crate::traits::ancestry::average_name_length::AverageNameLength;
use crate::traits::ancestry::has_valid_genders::HasValidGenders;
use crate::traits::random_enum::RandomEnum;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashSet;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug, Default)]
pub struct RandomNpcData {
    pub name_origin_filter: NameSystemOriginFilter,
    pub gender_filter: Option<Vec<Gender>>,
    pub class_filter: Option<ClassFilter>,
    pub level_filter: Option<LevelData>,
    pub job_filter: Option<JobFilter>,
    pub name_max_length: Option<usize>,
    pub generate_nickname: Option<bool>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct AncestryData {
    pub ancestry: String,
    pub valid_genders: Vec<Gender>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct RandomNameData {
    #[schema(minimum = 2, maximum = 40, example = 10)]
    pub name_max_length: Option<usize>,
    #[schema(minimum = 1, maximum = 100, example = 10)]
    pub max_n_of_names: Option<usize>,
    pub origin: NameSystemOrigin,
    pub gender: Option<Gender>,
}

impl RandomNameData {
    pub fn default_with_system(game_system: GameSystem) -> Self {
        let (max_length, gender, origin) = match game_system {
            GameSystem::Pathfinder => {
                let ancestry = PfAncestry::random();
                (
                    ancestry.get_average_name_length(),
                    Gender::filtered_random(&ancestry.get_valid_genders()),
                    NameSystemOrigin::FromPf(Some(PfNameOrigin::FromAncestry(Some(ancestry)))),
                )
            }
            GameSystem::Starfinder => {
                let ancestry = SfAncestry::random();
                (
                    ancestry.get_average_name_length(),
                    Gender::filtered_random(&ancestry.get_valid_genders()),
                    NameSystemOrigin::FromSf(Some(SfNameOrigin::FromAncestry(Some(ancestry)))),
                )
            }
        };

        Self {
            name_max_length: Some(max_length),
            max_n_of_names: Some(10),
            origin,
            gender: Some(gender),
        }
    }
}

impl Default for RandomNameData {
    fn default() -> Self {
        let ancestry = PfAncestry::random();
        let gender = Gender::filtered_random(&PfAncestry::get_valid_genders(&ancestry));
        Self {
            name_max_length: Some(PfAncestry::get_average_name_length(&ancestry)),
            max_n_of_names: Some(10),
            origin: NameSystemOrigin::FromPf(Some(PfNameOrigin::FromAncestry(Some(ancestry)))),
            gender: Some(gender),
        }
    }
}

impl RandomNpcData {
    pub fn default_with_system(game_system: GameSystem) -> Self {
        let (class_filter, origin) = match game_system {
            GameSystem::Pathfinder => (
                ClassFilter::FromPf(None),
                NameSystemOriginFilter::FromPf(None),
            ),
            GameSystem::Starfinder => (
                ClassFilter::FromSf(None),
                NameSystemOriginFilter::FromSf(None),
            ),
        };

        Self {
            name_origin_filter: origin,
            gender_filter: None,
            class_filter: Some(class_filter),
            level_filter: None,
            job_filter: None,
            name_max_length: None,
            generate_nickname: None,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.level_filter
            .as_ref()
            .is_none_or(LevelData::is_data_valid)
            && match self.name_origin_filter.clone() {
                NameSystemOriginFilter::FromPf(pf) => {
                    pf.as_ref().is_none_or(|pf_filter| match pf_filter {
                        PfNameOriginFilter::FromAncestry(ancestry_filter) => {
                            if let Some(g_filter) = self.gender_filter.clone()
                                && let Some(a_list) = ancestry_filter.clone()
                            {
                                // we check for ancestries/genders conflicts
                                let a_genders: HashSet<_> = a_list
                                    .iter()
                                    .flat_map(PfAncestry::get_valid_genders)
                                    .collect();
                                a_genders.iter().any(|a| g_filter.contains(a))
                            } else {
                                true
                            }
                        }
                        PfNameOriginFilter::FromCulture(_) => true,
                    })
                }
                NameSystemOriginFilter::FromSf(sf) => {
                    sf.as_ref().is_none_or(|sf_filter| match sf_filter {
                        SfNameOriginFilter::FromAncestry(ancestry_filter) => {
                            if let Some(g_filter) = self.gender_filter.clone()
                                && let Some(a_list) = ancestry_filter.clone()
                            {
                                // we check for ancestries/genders conflicts
                                let a_genders: HashSet<_> = a_list
                                    .iter()
                                    .flat_map(SfAncestry::get_valid_genders)
                                    .collect();
                                a_genders.iter().any(|a| g_filter.contains(a))
                            } else {
                                true
                            }
                        }
                    })
                }
            }
    }
}

impl RandomNameData {
    pub fn is_valid(&self) -> bool {
        match &self.origin {
            NameSystemOrigin::FromPf(pf) => match pf {
                Some(PfNameOrigin::FromAncestry(pfanc)) => {
                    if let Some(g) = &self.gender
                        && let Some(a) = pfanc
                    {
                        a.get_valid_genders().contains(g)
                    } else {
                        true
                    }
                }
                _ => true,
            },
            NameSystemOrigin::FromSf(sf) => match sf {
                Some(SfNameOrigin::FromAncestry(sfanc)) => {
                    if let Some(g) = &self.gender
                        && let Some(a) = sfanc
                    {
                        a.get_valid_genders().contains(g)
                    } else {
                        true
                    }
                }
                _ => true,
            },
        }
    }
}
