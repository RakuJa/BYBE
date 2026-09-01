use crate::traits::template_enum::GenericPercentage;
use anyhow::bail;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct RequestedEquippablePercentages {
    #[schema(minimum = 0, maximum = 100, example = 14)]
    pub equipment_percentage: Option<u8>,
    #[schema(minimum = 0, maximum = 100, example = 14)]
    pub weapon_percentage: Option<u8>,
    #[schema(minimum = 0, maximum = 100, example = 14)]
    pub armor_percentage: Option<u8>,
    #[schema(minimum = 0, maximum = 100, example = 14)]
    pub shield_percentage: Option<u8>,
    #[schema(minimum = 0, maximum = 100, example = 14)]
    pub treasure_percentage: Option<u8>,
    #[schema(minimum = 0, maximum = 100, example = 14)]
    pub backpack_percentage: Option<u8>,
}

impl RequestedEquippablePercentages {
    pub const fn is_empty(&self) -> bool {
        self.equipment_percentage.is_none()
            && self.weapon_percentage.is_none()
            && self.armor_percentage.is_none()
            && self.shield_percentage.is_none()
            && self.treasure_percentage.is_none()
            && self.backpack_percentage.is_none()
    }
}
#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct RequestedConsumablePercentages {
    #[schema(minimum = 0, maximum = 100, example = 14)]
    pub generic_percentage: Option<u8>,
    #[schema(minimum = 0, maximum = 100, example = 14)]
    pub ammunition_percentage: Option<u8>,
}

impl RequestedConsumablePercentages {
    pub const fn is_empty(&self) -> bool {
        self.generic_percentage.is_none() && self.ammunition_percentage.is_none()
    }
}
#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct ItemTypesPercentages {
    pub equippable_percentages: RequestedEquippablePercentages,
    pub consumable_percentages: RequestedConsumablePercentages,
}

#[derive(
    Default, Serialize, Deserialize, ToSchema, Hash, Ord, PartialOrd, Eq, PartialEq, Clone,
)]
pub struct EquippablePercentages {
    pub equipment_percentage: u8,
    pub weapon_percentage: u8,
    pub armor_percentage: u8,
    pub shield_percentage: u8,
    pub treasure_percentage: u8,
    pub backpack_percentage: u8,
}

#[derive(
    Default, Serialize, Deserialize, ToSchema, Hash, Ord, PartialOrd, Eq, PartialEq, Clone,
)]
pub struct ConsumablePercentages {
    pub generic_percentage: u8,
    pub ammunition_percentage: u8,
}

impl GenericPercentage for ConsumablePercentages {
    fn to_vec(self) -> Vec<u8> {
        vec![self.generic_percentage, self.ammunition_percentage]
    }
}

impl TryFrom<Vec<u8>> for ConsumablePercentages {
    type Error = anyhow::Error;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        if value.len() != 2 {
            bail!(
                "Given vector length should be exactly the number of fields present in the struct"
            );
        } else {
            Ok(Self {
                generic_percentage: value[0],
                ammunition_percentage: value[1],
            })
        }
    }
}

impl GenericPercentage for EquippablePercentages {
    fn to_vec(self) -> Vec<u8> {
        vec![
            self.equipment_percentage,
            self.weapon_percentage,
            self.armor_percentage,
            self.shield_percentage,
            self.treasure_percentage,
            self.backpack_percentage,
        ]
    }
}

impl TryFrom<Vec<u8>> for EquippablePercentages {
    type Error = anyhow::Error;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        if value.len() != 6 {
            bail!(
                "Given vector length should be exactly the number of fields present in the struct"
            );
        } else {
            Ok(Self {
                equipment_percentage: value[0],
                weapon_percentage: value[1],
                armor_percentage: value[2],
                shield_percentage: value[3],
                treasure_percentage: value[4],
                backpack_percentage: value[5],
            })
        }
    }
}
