use crate::models::routers_validator_structs::{OrderEnum, PaginatedRequest};
use serde::{Deserialize, Serialize};
use strum::Display;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Serialize, Deserialize, ToSchema, Default, Eq, PartialEq, Hash, Clone, Display)]
pub enum CreatureSortEnum {
    #[serde(alias = "id", alias = "ID")]
    Id,
    #[default]
    #[serde(alias = "name", alias = "NAME")]
    Name,
    #[serde(alias = "level", alias = "LEVEL")]
    Level,
    #[serde(alias = "trait", alias = "TRAIT")]
    Trait,
    #[serde(alias = "size", alias = "SIZE")]
    Size,
    #[serde(alias = "type", alias = "TYPE")]
    Type,
    #[serde(alias = "hp", alias = "HP")]
    Hp,
    #[serde(alias = "rarity", alias = "RARITY")]
    Rarity,
    #[serde(alias = "family", alias = "FAMILY")]
    Family,
}

#[derive(Serialize, Deserialize, IntoParams, Validate, Eq, PartialEq, Hash, Default)]
pub struct BestiarySortData {
    pub sort_by: Option<CreatureSortEnum>,
    pub order_by: Option<OrderEnum>,
}

#[derive(Serialize, Deserialize, IntoParams, Validate, Eq, PartialEq, Hash)]
pub struct BestiaryPaginatedRequest {
    pub paginated_request: PaginatedRequest,
    pub bestiary_sort_data: BestiarySortData,
}
