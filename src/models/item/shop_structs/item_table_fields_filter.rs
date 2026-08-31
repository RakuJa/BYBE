use crate::models::item::item_metadata::type_enum::ItemTypeEnum;
use crate::models::shared::rarity_enum::RarityEnum;
use crate::models::shared::size_enum::SizeEnum;

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
