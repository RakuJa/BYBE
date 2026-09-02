use crate::models::item::item_metadata::type_enum::ItemTypeEnum;
use crate::models::item::shop_structs::item_type_percentages::{
    ConsumablePercentages, EquippablePercentages,
};
use crate::models::shared::rarity_enum::RarityEnum;
use crate::traits::template_enum::{GenericTemplate, ItemTemplate};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};
use utoipa::ToSchema;

#[derive(
    Serialize, Deserialize, ToSchema, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Clone,
)]
pub struct ShopTemplateData {
    pub name: String,
    pub description: String,
    pub consumable_percentages: ConsumablePercentages,
    pub equippable_percentages: EquippablePercentages,
    pub item_types: Vec<ItemTypeEnum>,
    pub item_rarities: Vec<RarityEnum>,
    pub item_traits_whitelist: Vec<String>,
    pub item_traits_blacklist: Vec<String>,
}

impl<T> From<T> for ShopTemplateData
where
    T: GenericTemplate + ToString + ItemTemplate,
{
    fn from(template_enum: T) -> Self {
        Self {
            name: template_enum.to_string(),
            description: template_enum.get_description(),
            equippable_percentages: template_enum.get_equippable_percentages(),
            consumable_percentages: template_enum.get_consumable_percentages(),
            item_types: template_enum.get_allowed_item_types(),
            item_rarities: template_enum.get_allowed_rarities(),
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
    Debug,
)]
pub enum SfShopTemplateEnum {
    Fabricator,
    Biochemist,
    #[default]
    General,
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
    Debug,
)]
pub enum PfShopTemplateEnum {
    Blacksmith,
    Alchemist,
    #[default]
    General,
}

impl GenericTemplate for SfShopTemplateEnum {
    fn get_equippable_percentages(&self) -> EquippablePercentages {
        match self {
            Self::Fabricator => EquippablePercentages {
                equipment_percentage: 10,
                weapon_percentage: 45,
                armor_percentage: 20,
                shield_percentage: 20,
                treasure_percentage: 0,
                backpack_percentage: 5,
            },
            Self::Biochemist => EquippablePercentages {
                equipment_percentage: 90,
                weapon_percentage: 0,
                armor_percentage: 0,
                shield_percentage: 0,
                treasure_percentage: 0,
                backpack_percentage: 10,
            },
            Self::General => EquippablePercentages {
                equipment_percentage: 65,
                weapon_percentage: 10,
                armor_percentage: 10,
                shield_percentage: 10,
                treasure_percentage: 0,
                backpack_percentage: 5,
            },
        }
    }

    fn get_consumable_percentages(&self) -> ConsumablePercentages {
        match self {
            Self::Fabricator => ConsumablePercentages {
                generic_percentage: 60,
                ammunition_percentage: 40,
            },
            Self::Biochemist => ConsumablePercentages {
                generic_percentage: 80,
                ammunition_percentage: 20,
            },
            Self::General => ConsumablePercentages {
                generic_percentage: 90,
                ammunition_percentage: 10,
            },
        }
    }

    fn get_allowed_rarities(&self) -> Vec<RarityEnum> {
        match self {
            Self::Fabricator | Self::Biochemist | Self::General => {
                vec![RarityEnum::Common, RarityEnum::Uncommon, RarityEnum::Rare]
            }
        }
    }

    fn get_traits_whitelist(&self) -> Vec<String> {
        // For future-proof, right now contains 0 logic
        match self {
            Self::Fabricator | Self::General => {
                vec![]
            }
            Self::Biochemist => {
                vec![
                    "Alchemical".to_string(),
                    "Bomb".to_string(),
                    "Splash".to_string(),
                    "Potion".to_string(),
                ]
            }
        }
    }

    fn get_traits_blacklist(&self) -> Vec<String> {
        match self {
            Self::Fabricator | Self::Biochemist | Self::General => {
                vec![]
            }
        }
    }

    fn get_description(&self) -> String {
        String::from(match self {
            Self::Fabricator => {
                "Mainly weapons, armors and shields, sometimes equipment and consumables"
            }
            Self::Biochemist => "Only equipment and consumables, no weapons, armors or shields",
            Self::General => "All kinds of items",
        })
    }
}

impl ItemTemplate for SfShopTemplateEnum {
    fn get_allowed_item_types(&self) -> Vec<ItemTypeEnum> {
        match self {
            Self::Fabricator | Self::General => {
                vec![
                    ItemTypeEnum::Armor,
                    ItemTypeEnum::Shield,
                    ItemTypeEnum::Weapon,
                    ItemTypeEnum::Consumable,
                    ItemTypeEnum::Equipment,
                    ItemTypeEnum::Ammunition,
                    ItemTypeEnum::Backpack,
                ]
            }
            Self::Biochemist => {
                vec![
                    ItemTypeEnum::Consumable,
                    ItemTypeEnum::Equipment,
                    ItemTypeEnum::Ammunition,
                    ItemTypeEnum::Backpack,
                ]
            }
        }
    }
}

impl GenericTemplate for PfShopTemplateEnum {
    fn get_equippable_percentages(&self) -> EquippablePercentages {
        match self {
            Self::Blacksmith => EquippablePercentages {
                equipment_percentage: 10,
                weapon_percentage: 45,
                armor_percentage: 20,
                shield_percentage: 20,
                treasure_percentage: 0,
                backpack_percentage: 5,
            },
            Self::Alchemist => EquippablePercentages {
                equipment_percentage: 90,
                weapon_percentage: 0,
                armor_percentage: 0,
                shield_percentage: 0,
                treasure_percentage: 0,
                backpack_percentage: 10,
            },
            Self::General => EquippablePercentages {
                equipment_percentage: 65,
                weapon_percentage: 10,
                armor_percentage: 10,
                shield_percentage: 10,
                treasure_percentage: 0,
                backpack_percentage: 5,
            },
        }
    }

    fn get_consumable_percentages(&self) -> ConsumablePercentages {
        match self {
            Self::Blacksmith => ConsumablePercentages {
                generic_percentage: 60,
                ammunition_percentage: 40,
            },
            Self::Alchemist => ConsumablePercentages {
                generic_percentage: 80,
                ammunition_percentage: 20,
            },
            Self::General => ConsumablePercentages {
                generic_percentage: 90,
                ammunition_percentage: 10,
            },
        }
    }

    fn get_allowed_rarities(&self) -> Vec<RarityEnum> {
        match self {
            Self::Blacksmith | Self::Alchemist | Self::General => {
                vec![RarityEnum::Common, RarityEnum::Uncommon, RarityEnum::Rare]
            }
        }
    }

    fn get_traits_whitelist(&self) -> Vec<String> {
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

    fn get_traits_blacklist(&self) -> Vec<String> {
        match self {
            Self::Blacksmith | Self::Alchemist | Self::General => {
                vec![]
            }
        }
    }

    fn get_description(&self) -> String {
        String::from(match self {
            Self::Blacksmith => {
                "Mainly weapons, armors and shields, sometimes equipment and consumables"
            }
            Self::Alchemist => "Only equipment and consumables, no weapons, armors or shields",
            Self::General => "All kinds of items",
        })
    }
}

impl ItemTemplate for PfShopTemplateEnum {
    fn get_allowed_item_types(&self) -> Vec<ItemTypeEnum> {
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
}
