use crate::models::pf_version_enum::GameSystemVersionEnum;
use crate::models::shared::rarity_enum::RarityEnum;
use crate::models::shared::size_enum::SizeEnum;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(
    Default,
    Serialize,
    Deserialize,
    Debug,
    ToSchema,
    Copy,
    Clone,
    Eq,
    Hash,
    PartialEq,
    Ord,
    PartialOrd,
)]
pub enum HazardComplexityEnum {
    Simple,
    Complex,
    #[default]
    Any,
}

impl From<bool> for HazardComplexityEnum {
    fn from(is_complex: bool) -> Self {
        if is_complex {
            Self::Complex
        } else {
            Self::Simple
        }
    }
}

#[derive(Serialize, Deserialize, IntoParams, ToSchema)]
pub struct HazardFieldFilters {
    pub name_filter: Option<String>,
    pub source_filter: Option<Vec<String>>,
    pub complexity_filter: Option<HazardComplexityEnum>,
    pub rarity_filter: Option<Vec<RarityEnum>>,
    pub size_filter: Option<Vec<SizeEnum>>,

    pub trait_whitelist_filter: Option<Vec<String>>,
    pub trait_blacklist_filter: Option<Vec<String>>,

    #[schema(minimum = 0, example = 0)]
    pub min_ac_filter: Option<i64>,
    #[schema(minimum = 0, example = 100)]
    pub max_ac_filter: Option<i64>,

    #[schema(minimum = 0, example = 0)]
    pub min_hardness_filter: Option<i64>,
    #[schema(minimum = 0, example = 100)]
    pub max_hardness_filter: Option<i64>,

    #[schema(minimum = 0, example = 0)]
    pub min_hp_filter: Option<i64>,
    #[schema(minimum = 0, example = 100)]
    pub max_hp_filter: Option<i64>,

    #[schema(minimum = -1, example = -1)]
    pub min_level_filter: Option<i64>,
    #[schema(minimum = -1, example = 5)]
    pub max_level_filter: Option<i64>,

    #[schema(minimum = 0, example = 0)]
    pub min_will_filter: Option<i64>,
    #[schema(minimum = 0, example = 100)]
    pub max_will_filter: Option<i64>,

    #[schema(minimum = 0, example = 0)]
    pub min_fortitude_filter: Option<i64>,
    #[schema(minimum = 0, example = 100)]
    pub max_fortitude_filter: Option<i64>,

    #[schema(minimum = 0, example = 0)]
    pub min_reflex_filter: Option<i64>,
    #[schema(minimum = 0, example = 100)]
    pub max_reflex_filter: Option<i64>,

    pub game_system_version: Option<GameSystemVersionEnum>,
}
