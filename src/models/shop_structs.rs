use crate::models::pf_version_enum::PathfinderVersionEnum;
use crate::models::routers_validator_structs::Dice;
use serde::{Deserialize, Serialize};
use strum::EnumIter;
use utoipa::ToSchema;
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
    pub pathfinder_version: PathfinderVersionEnum,
}
