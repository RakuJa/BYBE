use crate::models::item::item_metadata::type_enum::ItemTypeEnum;
use crate::models::pf_version_enum::PathfinderVersionEnum;
use crate::models::routers_validator_structs::{Dice, OrderEnum, PaginatedRequest};
use crate::models::shared::rarity_enum::RarityEnum;
use crate::models::shared::size_enum::SizeEnum;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};
use utoipa::{IntoParams, ToSchema};

#[derive(
    Serialize, Deserialize, ToSchema, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Clone,
)]
pub struct ShopTemplateData {
    pub name: String,
    pub description: String,
    #[schema(minimum = 0, maximum = 100, example = 25)]
    pub equipment_percentage: u8,
    #[schema(minimum = 0, maximum = 100, example = 25)]
    pub weapon_percentage: u8,
    #[schema(minimum = 0, maximum = 100, example = 25)]
    pub armor_percentage: u8,
    #[schema(minimum = 0, maximum = 100, example = 25)]
    pub shield_percentage: u8,
    pub item_types: Vec<ItemTypeEnum>,
    pub item_rarities: Vec<RarityEnum>,
    pub item_traits_whitelist: Vec<String>,
    pub item_traits_blacklist: Vec<String>,
}

impl From<ShopTemplateEnum> for ShopTemplateData {
    fn from(template_enum: ShopTemplateEnum) -> Self {
        let (e_p, w_p, a_p, s_p) = template_enum.get_equippable_percentages();
        Self {
            name: template_enum.to_string(),
            description: template_enum.get_description(),
            equipment_percentage: e_p,
            weapon_percentage: w_p,
            armor_percentage: a_p,
            shield_percentage: s_p,
            item_types: template_enum.get_allowed_item_types(),
            item_rarities: template_enum.get_allowed_item_rarities(),
            item_traits_whitelist: template_enum.get_traits_whitelist(),
            item_traits_blacklist: template_enum.get_traits_blacklist(),
        }
    }
}

#[derive(
    Serialize,
    Deserialize,
    ToSchema,
    Default,
    EnumIter,
    Eq,
    PartialEq,
    Hash,
    Ord,
    PartialOrd,
    Clone,
    Display,
)]
pub enum ShopTemplateEnum {
    Blacksmith,
    Alchemist,
    #[default]
    General,
}

impl ShopTemplateEnum {
    /// Returns percentage of equipment, weapons, armor, shield for the given template
    pub const fn get_equippable_percentages(&self) -> (u8, u8, u8, u8) {
        match self {
            Self::Blacksmith => (10, 40, 25, 25),
            Self::Alchemist => (100, 0, 0, 0),
            Self::General => (70, 10, 10, 10),
        }
    }

    pub fn get_allowed_item_types(&self) -> Vec<ItemTypeEnum> {
        match self {
            Self::Blacksmith | Self::General => {
                vec![
                    ItemTypeEnum::Armor,
                    ItemTypeEnum::Shield,
                    ItemTypeEnum::Weapon,
                    ItemTypeEnum::Consumable,
                    ItemTypeEnum::Equipment,
                ]
            }
            Self::Alchemist => {
                vec![ItemTypeEnum::Consumable, ItemTypeEnum::Equipment]
            }
        }
    }

    pub fn get_allowed_item_rarities(&self) -> Vec<RarityEnum> {
        match self {
            Self::Blacksmith | Self::Alchemist | Self::General => {
                vec![RarityEnum::Common, RarityEnum::Uncommon, RarityEnum::Rare]
            }
        }
    }

    pub fn get_traits_whitelist(&self) -> Vec<String> {
        // For future-proof, right now contains 0 logic
        match self {
            Self::Blacksmith | Self::General => {
                vec![]
            }
            Self::Alchemist => {
                vec![
                    "Alchemical".to_string(),
                    "Bomb".to_string(),
                    "Splash".to_string(),
                    "Potion".to_string(),
                ]
            }
        }
    }

    pub const fn get_traits_blacklist(&self) -> Vec<String> {
        match self {
            Self::Blacksmith | Self::Alchemist | Self::General => {
                vec![]
            }
        }
    }

    pub fn get_description(&self) -> String {
        String::from(match self {
            Self::Blacksmith => {
                "Mainly weapons, armors and shields, sometimes equipment and consumables"
            }
            Self::Alchemist => "Only equipment and consumables, no weapons, armors or shields",
            Self::General => "All kinds of items",
        })
    }
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct RandomShopData {
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

    #[schema(minimum = 0, maximum = 100, example = 25)]
    pub equipment_percentage: Option<u8>,
    #[schema(minimum = 0, maximum = 100, example = 25)]
    pub weapon_percentage: Option<u8>,
    #[schema(minimum = 0, maximum = 100, example = 25)]
    pub armor_percentage: Option<u8>,
    #[schema(minimum = 0, maximum = 100, example = 25)]
    pub shield_percentage: Option<u8>,

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

#[derive(Serialize, Deserialize, IntoParams, ToSchema, Eq, PartialEq, Hash, Default)]
pub struct ShopSortData {
    // Optional here for swagger, kinda bad but w/e
    pub sort_by: Option<ItemSortEnum>,
    pub order_by: Option<OrderEnum>,
}

#[derive(Serialize, Deserialize, IntoParams, Eq, PartialEq, Hash)]
pub struct ShopPaginatedRequest {
    pub paginated_request: PaginatedRequest,
    pub shop_sort_data: ShopSortData,
}
