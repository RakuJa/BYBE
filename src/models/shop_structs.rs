use crate::models::pf_version_enum::PathfinderVersionEnum;
use crate::models::routers_validator_structs::{Dice, OrderEnum, PaginatedRequest};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(
    Serialize, Deserialize, ToSchema, Default, EnumIter, Eq, PartialEq, Hash, Ord, PartialOrd, Clone,
)]
pub enum ShopTypeEnum {
    Blacksmith,
    Alchemist,
    #[default]
    General,
}

#[derive(Serialize, Deserialize, ToSchema, Validate, Clone)]
pub struct RandomShopData {
    #[validate(range(max = 30))]
    pub min_level: Option<u8>,
    #[validate(range(max = 30))]
    pub max_level: Option<u8>,
    #[validate(length(min = 1))]
    pub equipment_dices: Vec<Dice>,
    #[validate(length(min = 1))]
    pub consumable_dices: Vec<Dice>,
    pub shop_type: Option<ShopTypeEnum>,
    pub pathfinder_version: Option<PathfinderVersionEnum>,
}

pub struct ShopFilterQuery {
    //pub shop_type: ShopTypeEnum,
    pub min_level: u8,
    pub max_level: u8,
    pub n_of_equipment: i64,
    pub n_of_consumables: i64,
    pub n_of_weapons: i64,
    pub n_of_armors: i64,
    pub pathfinder_version: PathfinderVersionEnum,
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
    #[serde(alias = "type", alias = "TYPE")]
    Type,
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
