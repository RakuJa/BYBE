use crate::models::hazard::hazard_field_filter::HazardComplexityEnum;
use crate::models::routers_validator_structs::{OrderEnum, PaginatedRequest};
use crate::models::shared::rarity_enum::RarityEnum;
use crate::models::shared::size_enum::SizeEnum;
use serde::{Deserialize, Serialize};
use strum::Display;
use utoipa::{IntoParams, ToSchema};

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct HazardTableFieldsFilter {
    pub source_filter: Vec<String>,
    pub rarity_filter: Vec<RarityEnum>,
    pub size_filter: Vec<SizeEnum>,
    pub supported_version: Vec<String>,

    pub(crate) level_filter: Vec<(HazardComplexityEnum, i64)>,

    #[schema(example = 0)]
    pub min_ac: Option<i64>,
    #[schema(example = 5)]
    pub max_ac: Option<i64>,

    #[schema(example = 0)]
    pub min_hardness: Option<i64>,
    #[schema(example = 5)]
    pub max_hardness: Option<i64>,

    #[schema(example = 0)]
    pub min_hp: Option<i64>,
    #[schema(example = 5)]
    pub max_hp: Option<i64>,

    #[schema(example = 0)]
    pub min_will: Option<i64>,
    #[schema(example = 5)]
    pub max_will: Option<i64>,

    #[schema(example = 0)]
    pub min_reflex: Option<i64>,
    #[schema(example = 5)]
    pub max_reflex: Option<i64>,

    #[schema(example = 0)]
    pub min_fortitude: Option<i64>,
    #[schema(example = 5)]
    pub max_fortitude: Option<i64>,
}

pub struct HazardFilterQuery {
    pub hazard_table_fields_filter: HazardTableFieldsFilter,
    pub trait_whitelist_filter: Vec<String>,
    pub trait_blacklist_filter: Vec<String>,
}

#[derive(Serialize, Deserialize, ToSchema, Default, Eq, PartialEq, Hash, Clone, Display)]
pub enum HazardSortEnum {
    #[serde(alias = "id", alias = "ID")]
    Id,
    #[default]
    #[serde(alias = "name", alias = "NAME")]
    Name,
    #[serde(alias = "ac", alias = "AC")]
    Ac,
    #[serde(alias = "hardness", alias = "HARDNESS")]
    Hardness,
    #[serde(alias = "hp", alias = "HP")]
    Hp,
    #[serde(alias = "kind", alias = "KIND")]
    Kind,
    #[serde(alias = "level", alias = "LEVEL")]
    Level,
    #[serde(alias = "trait", alias = "TRAIT")]
    Trait,
    #[serde(alias = "rarity", alias = "RARITY")]
    Rarity,
    #[serde(alias = "size", alias = "SIZE")]
    Size,
    #[serde(alias = "source", alias = "SOURCE")]
    Source,
    #[serde(alias = "fortitude", alias = "FORTITUDE")]
    Fortitude,
    #[serde(alias = "reflex", alias = "REFLEX")]
    Reflex,
    #[serde(alias = "will", alias = "WILL")]
    Will,
}

#[derive(Serialize, Deserialize, IntoParams, ToSchema, Eq, PartialEq, Hash, Default)]
pub struct HazardListingSortData {
    // Optional here for swagger, kinda bad but w/e
    pub sort_by: Option<HazardSortEnum>,
    pub order_by: Option<OrderEnum>,
}

#[derive(Serialize, Deserialize, IntoParams, Eq, PartialEq, Hash)]
pub struct HazardListingPaginatedRequest {
    pub paginated_request: PaginatedRequest,
    pub hazard_sort_data: HazardListingSortData,
}
