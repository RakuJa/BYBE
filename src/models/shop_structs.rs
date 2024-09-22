use crate::models::item::item_metadata::type_enum::ItemTypeEnum;
use crate::models::pf_version_enum::PathfinderVersionEnum;
use crate::models::routers_validator_structs::{Dice, OrderEnum, PaginatedRequest};
use crate::models::shared::rarity_enum::RarityEnum;
use crate::models::shared::size_enum::SizeEnum;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(
    Serialize, Deserialize, ToSchema, Default, EnumIter, Eq, PartialEq, Hash, Ord, PartialOrd, Clone,
)]
pub enum ShopTemplateEnum {
    Blacksmith,
    Alchemist,
    #[default]
    General,
}

#[derive(Serialize, Deserialize, ToSchema, Validate, Clone)]
pub struct RandomShopData {
    pub category_filter: Option<Vec<String>>,
    pub source_filter: Option<Vec<String>>,
    pub trait_whitelist_filter: Option<Vec<String>>,
    pub trait_blacklist_filter: Option<Vec<String>>,
    pub type_filter: Option<Vec<ItemTypeEnum>>,
    pub rarity_filter: Option<Vec<RarityEnum>>,
    pub size_filter: Option<Vec<SizeEnum>>,

    #[validate(range(max = 30))]
    pub min_level: Option<u8>,
    #[validate(range(max = 30))]
    pub max_level: Option<u8>,
    #[validate(length(min = 1))]
    pub equipment_dices: Vec<Dice>,
    #[validate(length(min = 1))]
    pub consumable_dices: Vec<Dice>,
    pub shop_template: Option<ShopTemplateEnum>,
    pub pathfinder_version: Option<PathfinderVersionEnum>,
}

pub struct ItemTableFieldsFilter {
    pub category_filter: Vec<String>,
    pub source_filter: Vec<String>,
    pub type_filter: Vec<ItemTypeEnum>,
    pub rarity_filter: Vec<RarityEnum>,
    pub size_filter: Vec<SizeEnum>,
    pub supported_version: Vec<String>,

    pub min_level: u8,
    pub max_level: u8,
}

pub struct ShopFilterQuery {
    pub item_table_fields_filter: ItemTableFieldsFilter,
    pub trait_whitelist_filter: Vec<String>,
    pub trait_blacklist_filter: Vec<String>,
    pub n_of_equipment: i64,
    pub n_of_consumables: i64,
    pub n_of_weapons: i64,
    pub n_of_armors: i64,
    pub n_of_shields: i64,
}

#[derive(Serialize, Deserialize, ToSchema, Default, Eq, PartialEq, Hash, Clone, Display)]
pub enum ItemSortEnum {
    #[serde(alias = "id", alias = "ID")]
    Id,
    #[default]
    #[serde(alias = "name", alias = "NAME")]
    Name,
    #[serde(alias = "level", alias = "LEVEL")]
    Level,
    #[serde(alias = "trait", alias = "TRAIT")]
    Trait,
    #[serde(alias = "type", alias = "TYPE")]
    Type,
    #[serde(alias = "rarity", alias = "RARITY")]
    Rarity,
    #[serde(alias = "source", alias = "SOURCE")]
    Source,
}

#[derive(Serialize, Deserialize, IntoParams, Validate, Eq, PartialEq, Hash, Default)]
pub struct ShopSortData {
    // Optional here for swagger, kinda bad but w/e
    pub sort_by: Option<ItemSortEnum>,
    pub order_by: Option<OrderEnum>,
}

#[derive(Serialize, Deserialize, IntoParams, Validate, Eq, PartialEq, Hash)]
pub struct ShopPaginatedRequest {
    pub paginated_request: PaginatedRequest,
    pub shop_sort_data: ShopSortData,
}
