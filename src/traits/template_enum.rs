use crate::models::item::item_metadata::type_enum::ItemTypeEnum;
use crate::models::item::shop_structs::item_type_percentages::{
    ConsumablePercentages, EquippablePercentages,
};
use crate::models::shared::rarity_enum::RarityEnum;
use serde::Serialize;
use std::fmt::Debug;
use std::hash::Hash;
use utoipa::ToSchema;

pub trait GenericPercentage:
    TryFrom<Vec<u8>, Error = anyhow::Error>
    + Default
    + Serialize
    + ToSchema
    + Eq
    + PartialEq
    + Hash
    + Ord
    + PartialOrd
    + Clone
{
    fn to_vec(self) -> Vec<u8>;
}

pub trait GenericTemplate:
    Serialize + ToSchema + Default + Eq + PartialEq + Hash + Ord + PartialOrd + Clone + Debug
{
    /// Returns percentage of equipment, weapons, armor, shield, treasure, backpack, ammunition for the given template
    fn get_equippable_percentages(&self) -> EquippablePercentages;

    fn get_consumable_percentages(&self) -> ConsumablePercentages;

    fn get_allowed_rarities(&self) -> Vec<RarityEnum>;

    fn get_traits_whitelist(&self) -> Vec<String>;

    fn get_traits_blacklist(&self) -> Vec<String>;

    fn get_description(&self) -> String;
}

pub trait ItemTemplate {
    fn get_allowed_item_types(&self) -> Vec<ItemTypeEnum>;
}
