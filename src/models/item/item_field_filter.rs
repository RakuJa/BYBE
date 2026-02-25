use crate::models::item::item_metadata::type_enum::ItemTypeEnum;
use crate::models::pf_version_enum::GameSystemVersionEnum;
use crate::models::shared::rarity_enum::RarityEnum;
use crate::models::shared::size_enum::SizeEnum;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Serialize, Deserialize, IntoParams, ToSchema)]
pub struct ItemFieldFilters {
    pub name_filter: Option<String>,
    pub category_filter: Option<Vec<String>>,
    pub source_filter: Option<Vec<String>>,
    pub trait_whitelist_filter: Option<Vec<String>>,
    pub trait_blacklist_filter: Option<Vec<String>>,

    #[schema(minimum = 0., example = 0.)]
    pub min_bulk_filter: Option<f64>,
    #[schema(minimum = 0., example = 5.)]
    pub max_bulk_filter: Option<f64>,
    #[schema(minimum = 0, example = 0)]
    pub min_hardness_filter: Option<i64>,
    #[schema(minimum = 0, example = 2)]
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
    pub min_price_filter: Option<i64>,
    #[schema(minimum = 0, example = 100)]
    pub max_price_filter: Option<i64>,
    #[schema(minimum = 0, example = 0)]
    pub min_n_of_uses_filter: Option<i64>,
    #[schema(minimum = 0, example = 5)]
    pub max_n_of_uses_filter: Option<i64>,

    pub type_filter: Option<Vec<ItemTypeEnum>>,
    pub rarity_filter: Option<Vec<RarityEnum>>,
    pub size_filter: Option<Vec<SizeEnum>>,
    pub game_system_version: Option<GameSystemVersionEnum>,
}
