use crate::models::item::item_metadata::type_enum::ItemTypeEnum;
use crate::models::item::shop_structs::item_type_percentages::ItemTypesPercentages;
use crate::models::routers_validator_structs::Dice;
use crate::models::shared::pf_version_enum::GameSystemVersionEnum;
use crate::models::shared::rarity_enum::RarityEnum;
use crate::models::shared::size_enum::SizeEnum;
use crate::traits::template_enum::{GenericTemplate, ItemTemplate};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[allow(clippy::option_if_let_else)]
pub mod schemas {
    use super::*;
    #[derive(Serialize, Deserialize, ToSchema, Clone)]
    pub struct RandomShopData<T: GenericTemplate + ItemTemplate> {
        pub category_filter: Option<Vec<String>>,
        pub source_filter: Option<Vec<String>>,
        pub trait_whitelist_filter: Option<Vec<String>>,
        pub trait_blacklist_filter: Option<Vec<String>>,
        pub type_filter: Option<Vec<ItemTypeEnum>>,
        pub rarity_filter: Option<Vec<RarityEnum>>,
        pub size_filter: Option<Vec<SizeEnum>>,

        #[schema(minimum = 0, maximum = 30, example = 0)]
        pub min_level: Option<u8>,
        #[schema(minimum = 0, maximum = 30, example = 5)]
        pub max_level: Option<u8>,

        #[schema(min_items = 1)]
        pub equippable_dices: Vec<Dice>,
        #[schema(min_items = 1)]
        pub consumable_dices: Vec<Dice>,

        pub percentages: ItemTypesPercentages,

        pub shop_template: Option<T>,
        pub game_system_version: Option<GameSystemVersionEnum>,
    }
}
