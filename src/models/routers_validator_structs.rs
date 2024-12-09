use crate::models::creature::creature_metadata::alignment_enum::AlignmentEnum;
use crate::models::creature::creature_metadata::creature_role::CreatureRoleEnum;
use crate::models::creature::creature_metadata::type_enum::CreatureTypeEnum;
use crate::models::item::item_metadata::type_enum::ItemTypeEnum;
use crate::models::pf_version_enum::PathfinderVersionEnum;
use crate::models::shared::rarity_enum::RarityEnum;
use crate::models::shared::size_enum::SizeEnum;
use serde::{Deserialize, Serialize};
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
    pub is_melee_filter: Option<bool>,
    pub is_ranged_filter: Option<bool>,
    pub is_spell_caster_filter: Option<bool>,
    pub pathfinder_version: Option<PathfinderVersionEnum>,
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
    pub pathfinder_version: Option<PathfinderVersionEnum>,
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
        PaginatedRequest {
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
    /// Dice roll will roll n dices with each roll in the range of 1<=result<=dice_size.
    /// It returns the sum of n_of_dices rolls.
    /// IT SHOULD NEVER BE <1, OTHERWISE WE BREAK THE CONTRACT OF THE METHOD.
    pub fn roll(&self) -> i64 {
        let mut roll_result = 0;
        for _ in 0..self.n_of_dices {
            // gen_range panics if n<2 (1..1), panic!
            // so we directly return 1 if that's the case
            roll_result += if self.dice_size > 1 {
                fastrand::i64(1..=self.dice_size as i64)
            } else {
                1
            }
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
        let m = self.dice_size as f64;
        let n = self.n_of_dices as f64;
        let b = bonus_dmg;
        let avg: f64 = (((m + 1.) / 2.) * n) + b;
        avg.floor() as i64
    }

    pub fn from_optional_dice_number_and_size(
        n_of_dices: Option<u8>,
        dice_size: Option<u8>,
    ) -> Option<Dice> {
        match (n_of_dices, dice_size) {
            (Some(n), Some(s)) => Some(Dice {
                n_of_dices: n,
                dice_size: s,
            }),
            (None, Some(s)) => Some(Dice {
                n_of_dices: 1,
                dice_size: s,
            }),
            (Some(n), None) => Some(Dice {
                n_of_dices: n,
                dice_size: 1,
            }),
            (_, _) => None,
        }
    }
}
