use crate::models::creature::creature_metadata::alignment_enum::AlignmentEnum;
use crate::models::creature::creature_metadata::creature_role::CreatureRoleEnum;
use crate::models::creature::creature_metadata::type_enum::CreatureTypeEnum;
use crate::models::routers_validator_structs::{OrderEnum, PaginatedRequest};
use crate::models::shared::rarity_enum::RarityEnum;
use crate::models::shared::size_enum::SizeEnum;
use serde::{Deserialize, Serialize};
use strum::Display;
use utoipa::{IntoParams, ToSchema};

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
    #[serde(alias = "alignment", alias = "ALIGNMENT")]
    Alignment,
    #[serde(alias = "attacks", alias = "ATTACKS")]
    Attacks,
    #[serde(alias = "roles", alias = "ROLES")]
    Roles,
}

#[derive(Serialize, Deserialize, IntoParams, ToSchema, Eq, PartialEq, Hash, Default)]
pub struct BestiarySortData {
    pub sort_by: Option<CreatureSortEnum>,
    pub order_by: Option<OrderEnum>,
}

#[derive(Serialize, Deserialize, IntoParams, Eq, PartialEq, Hash)]
pub struct BestiaryPaginatedRequest {
    pub paginated_request: PaginatedRequest,
    pub bestiary_sort_data: BestiarySortData,
}

#[derive(Clone)]
pub struct CreatureTableFieldsFilter {
    pub source_filter: Vec<String>,
    pub family_filter: Vec<String>,
    pub alignment_filter: Vec<AlignmentEnum>,
    pub size_filter: Vec<SizeEnum>,
    pub rarity_filter: Vec<RarityEnum>,
    pub type_filter: Vec<CreatureTypeEnum>,
    pub role_filter: Vec<CreatureRoleEnum>,
    pub role_lower_threshold: u8,
    pub role_upper_threshold: u8,
    pub is_melee_filter: Vec<bool>,
    pub is_ranged_filter: Vec<bool>,
    pub is_spellcaster_filter: Vec<bool>,
    pub supported_version: Vec<String>,

    pub level_filter: Vec<i64>,
}

impl CreatureTableFieldsFilter {
    pub const fn default_lower_threshold() -> u8 {
        50
    }

    pub const fn default_upper_threshold() -> u8 {
        100
    }
}

#[derive(Clone)]
pub struct BestiaryFilterQuery {
    pub creature_table_fields_filter: CreatureTableFieldsFilter,
    pub trait_whitelist_filter: Vec<String>,
    pub trait_blacklist_filter: Vec<String>,
}
