use crate::models::creature::creature_metadata::alignment_enum::AlignmentEnum;
use crate::models::creature::creature_metadata::creature_role::CreatureRoleEnum;
use crate::models::creature::creature_metadata::type_enum::CreatureTypeEnum;
use crate::models::item::item_metadata::type_enum::ItemTypeEnum;
use crate::models::pf_version_enum::GameSystemVersionEnum;
use crate::models::shared::rarity_enum::RarityEnum;
use crate::models::shared::size_enum::SizeEnum;
use nanorand::{Rng, WyRand};
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::json;
use std::collections::BTreeMap;
use strum::Display;
use utoipa::{IntoParams, ToSchema};
#[derive(Serialize, Deserialize, IntoParams, ToSchema)]
pub struct CreatureFieldFilters {
    pub name_filter: Option<String>,
    pub source_filter: Option<Vec<String>>,
    pub family_filter: Option<Vec<String>>,
    pub rarity_filter: Option<Vec<RarityEnum>>,
    pub size_filter: Option<Vec<SizeEnum>>,
    pub alignment_filter: Option<Vec<AlignmentEnum>>,
    pub trait_whitelist_filter: Option<Vec<String>>,
    pub trait_blacklist_filter: Option<Vec<String>>,
    pub role_filter: Option<Vec<CreatureRoleEnum>>,
    pub type_filter: Option<Vec<CreatureTypeEnum>>,
    #[schema(minimum = 0, maximum = 100, example = 50)]
    pub role_threshold: Option<i64>,
    #[schema(minimum = 0, example = 0)]
    pub min_hp_filter: Option<i64>,
    #[schema(minimum = 0, example = 100)]
    pub max_hp_filter: Option<i64>,
    #[schema(minimum = -1, example = -1)]
    pub min_level_filter: Option<i64>,
    #[schema(minimum = -1, example = 5)]
    pub max_level_filter: Option<i64>,

    #[schema(example = json!({"melee": true, "ranged": false, "spellcaster": true}))]
    pub attack_data_filter: Option<BTreeMap<String, Option<bool>>>,
    pub game_system_version: Option<GameSystemVersionEnum>,
}

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

#[derive(Serialize, Deserialize, ToSchema, Default, Eq, PartialEq, Hash, Clone, Display)]
pub enum OrderEnum {
    #[default]
    #[serde(alias = "ascending", alias = "ASCENDING")]
    Ascending,
    #[serde(alias = "descending", alias = "DESCENDING")]
    Descending,
}

#[derive(Serialize, Deserialize, IntoParams, Eq, PartialEq, Hash, ToSchema)]
pub struct PaginatedRequest {
    #[schema(minimum = 0, example = 0)]
    pub cursor: u32,
    #[schema(minimum = -1, maximum = 100, example = 100)]
    pub page_size: i16,
}

impl Default for PaginatedRequest {
    fn default() -> Self {
        Self {
            cursor: 0,
            page_size: 100,
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema, Eq, PartialEq, Hash, Clone)]
pub struct Dice {
    #[schema(minimum = 0, maximum = 255, example = 1)]
    pub n_of_dices: u8,
    // 1 needs to be an option, to allow 100d1 => 100
    #[schema(minimum = 0, maximum = 255, example = 20)]
    pub dice_size: u8,
}

impl Dice {
    /// Dice roll will roll n dices with each roll in the range of 1<=result<=`dice_size`.
    /// It returns the sum of `n_of_dices` rolls.
    /// IT SHOULD NEVER BE <1, OTHERWISE WE BREAK THE CONTRACT OF THE METHOD.
    pub fn roll(&self) -> u16 {
        let mut roll_result = 0_u16;
        let n_of_dices = u16::from(self.n_of_dices);
        let dice_size = u16::from(self.dice_size);
        for _ in 0..n_of_dices {
            roll_result += WyRand::new().generate_range(1..=dice_size);
        }
        roll_result
    }

    pub fn get_avg_dmg(&self, bonus_dmg: f64) -> i64 {
        // avg dice value is
        // AVG = (((M+1)/2)∗N)+B
        //
        // M = max value of the dice
        // N = number of dices
        // B = bonus dmg
        let m = f64::from(self.dice_size);
        let n = f64::from(self.n_of_dices);
        let b = bonus_dmg;
        let avg: f64 = f64::midpoint(m, 1.).mul_add(n, b);
        avg.floor() as i64
    }

    pub const fn from_optional_dice_number_and_size(
        n_of_dices: Option<u8>,
        dice_size: Option<u8>,
    ) -> Option<Self> {
        match (n_of_dices, dice_size) {
            (Some(n), Some(s)) => Some(Self {
                n_of_dices: n,
                dice_size: s,
            }),
            (None, Some(s)) => Some(Self {
                n_of_dices: 1,
                dice_size: s,
            }),
            (Some(n), None) => Some(Self {
                n_of_dices: n,
                dice_size: 1,
            }),
            (_, _) => None,
        }
    }
}

#[derive(Serialize, Deserialize, IntoParams, ToSchema, Default, Clone, Debug)]
pub struct LevelData {
    pub min_level: Option<i64>,
    pub max_level: Option<i64>,
}

impl LevelData {
    pub fn is_data_valid(&self) -> bool {
        self.min_level.unwrap_or(-1) <= self.max_level.unwrap_or(25)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(4, 1, 4)]
    #[case(100, 1, 100)]
    #[case(200, 1, 200)]
    #[case(17, 1, 17)]
    #[case(255, 1, 255)]
    #[case(0, 1, 0)]
    #[case(1, 1, 1)]
    fn roll_with_lots_of_d1(#[case] n_of_dices: u8, #[case] dice_size: u8, #[case] expected: u16) {
        let dice = Dice {
            n_of_dices,
            dice_size,
        };
        for _ in 0..1000 {
            let res = dice.roll();
            assert_eq!(res, expected);
        }
    }

    #[rstest]
    #[case(4, 3)]
    #[case(100, 100)]
    #[case(200, 20)]
    #[case(17, 3)]
    #[case(255, 255)]
    #[case(12, 12)]
    #[case(6, 6)]
    #[case(4, 4)]
    #[case(4, 8)]
    #[case(3, 20)]
    fn roll_with_different_dices_and_sizes(#[case] n_of_dices: u8, #[case] dice_size: u8) {
        let dice = Dice {
            n_of_dices,
            dice_size,
        };
        let max_value = u16::from(n_of_dices) * u16::from(dice_size);
        for _ in 0..1000 {
            let res = dice.roll();
            assert!(res > 0 && res <= max_value);
        }
    }
}
