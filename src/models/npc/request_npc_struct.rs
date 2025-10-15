use crate::models::npc::gender_enum::Gender;
use crate::models::routers_validator_structs::LevelData;
use crate::traits::class_enum::ClassEnum;
use crate::traits::job_enum::JobEnum;
use crate::traits::name_system::{NameOrigin, NameOriginFilter};
use crate::traits::origin::average_name_length::AverageNameLength;
use crate::traits::origin::has_valid_genders::HasValidGenders;
use crate::traits::random_enum::RandomEnum;
pub use schemas::*;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

#[allow(clippy::option_if_let_else)]
mod schemas {
    use super::*;

    #[derive(Serialize, Deserialize, ToSchema, Clone, Debug, Default)]
    pub struct RandomNpcData<C: ClassEnum, N: NameOriginFilter, J: JobEnum> {
        pub name_origin_filter: N,
        pub gender_filter: Option<Vec<Gender>>,
        pub class_filter: Option<Vec<C>>,
        pub level_filter: Option<LevelData>,
        pub job_filter: Option<Vec<J>>,
        pub name_max_length: Option<usize>,
        pub generate_nickname: Option<bool>,
    }

    #[derive(Serialize, Deserialize, ToSchema, Clone)]
    pub struct AncestryData {
        pub ancestry: String,
        pub valid_genders: Vec<Gender>,
    }

    #[derive(Serialize, Deserialize, ToSchema, Clone)]
    pub struct RandomNameData<N: NameOrigin> {
        #[schema(minimum = 2, maximum = 40, example = 10)]
        pub name_max_length: Option<usize>,
        #[schema(minimum = 1, maximum = 100, example = 10)]
        pub max_n_of_names: Option<usize>,
        pub origin: N,
        pub gender: Option<Gender>,
    }
}

impl<N: NameOrigin> RandomNameData<N> {
    pub fn default_with_system(origin: N) -> Self {
        let ancestry = N::get_random_ancestry();
        Self {
            name_max_length: Some(ancestry.get_average_name_length()),
            max_n_of_names: Some(10),
            origin,
            gender: Some(Gender::filtered_random(&ancestry.get_valid_genders())),
        }
    }
}

impl<C: ClassEnum, N: NameOriginFilter, J: JobEnum> RandomNpcData<C, N, J> {
    pub fn is_valid(&self) -> bool {
        self.level_filter
            .as_ref()
            .is_none_or(LevelData::is_data_valid)
            && self.gender_filter.clone().is_none_or(|g_filter| {
                self.name_origin_filter
                    .ancestries_have_at_least_one_valid_gender(g_filter)
            })
    }
}

impl<N: NameOrigin> RandomNameData<N> {
    pub fn is_valid(&self) -> bool {
        if let Some(g) = &self.gender
            && let Some(a) = self.origin.get_ancestry()
        {
            a.get_valid_genders().contains(g)
        } else {
            true
        }
    }
}
