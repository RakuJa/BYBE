use crate::models::creature::creature_metadata::alignment_enum::AlignmentEnum;
use crate::models::creature::creature_metadata::creature_role::CreatureRoleEnum;
use crate::models::creature::creature_metadata::type_enum::CreatureTypeEnum;
use crate::models::item::item_metadata::type_enum::ItemTypeEnum;
use crate::models::pf_version_enum::PathfinderVersionEnum;
use crate::models::shared::rarity_enum::RarityEnum;
use crate::models::shared::size_enum::SizeEnum;
use rand::Rng;
use serde::{Deserialize, Serialize};
use strum::Display;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Serialize, Deserialize, IntoParams, Validate)]
pub struct CreatureFieldFilters {
    pub name_filter: Option<String>,
    pub family_filter: Option<String>,
    pub rarity_filter: Option<RarityEnum>,
    pub size_filter: Option<SizeEnum>,
    pub alignment_filter: Option<AlignmentEnum>,
    pub role_filter: Option<CreatureRoleEnum>,
    pub type_filter: Option<CreatureTypeEnum>,
    #[validate(range(min = 0, max = 100))]
    pub role_threshold: Option<i64>,
    #[validate(range(min = 0))]
    pub min_hp_filter: Option<i64>,
    #[validate(range(min = 0))]
    pub max_hp_filter: Option<i64>,
    #[validate(range(min = -1))]
    pub min_level_filter: Option<i64>,
    #[validate(range(min = -1))]
    pub max_level_filter: Option<i64>,
    pub is_melee_filter: Option<bool>,
    pub is_ranged_filter: Option<bool>,
    pub is_spell_caster_filter: Option<bool>,
    pub pathfinder_version: Option<PathfinderVersionEnum>,
}

#[derive(Serialize, Deserialize, IntoParams, Validate)]
pub struct ItemFieldFilters {
    pub name_filter: Option<String>,
    pub category_filter: Option<String>,

    #[validate(range(min = 0.))]
    pub min_bulk_filter: Option<f64>,
    #[validate(range(min = 0.))]
    pub max_bulk_filter: Option<f64>,
    #[validate(range(min = 0))]
    pub min_hardness_filter: Option<i64>,
    #[validate(range(min = 0))]
    pub max_hardness_filter: Option<i64>,
    #[validate(range(min = 0))]
    pub min_hp_filter: Option<i64>,
    #[validate(range(min = 0))]
    pub max_hp_filter: Option<i64>,
    #[validate(range(min = -1))]
    pub min_level_filter: Option<i64>,
    #[validate(range(min = -1))]
    pub max_level_filter: Option<i64>,
    #[validate(range(min = -1))]
    pub min_price_filter: Option<i64>,
    #[validate(range(min = 0))]
    pub max_price_filter: Option<i64>,
    #[validate(range(min = 0))]
    pub min_n_of_uses_filter: Option<i64>,
    #[validate(range(min = 0))]
    pub max_n_of_uses_filter: Option<i64>,

    pub type_filter: Option<ItemTypeEnum>,
    pub rarity_filter: Option<RarityEnum>,
    pub size_filter: Option<SizeEnum>,
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

#[derive(Serialize, Deserialize, IntoParams, Validate, Eq, PartialEq, Hash, ToSchema)]
pub struct PaginatedRequest {
    #[validate(range(min = 0))]
    pub cursor: u32,
    #[validate(range(min = -1, max = 100))]
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

#[derive(Serialize, Deserialize, ToSchema, Validate, Eq, PartialEq, Hash, Clone)]
pub struct Dice {
    #[validate(range(min = 1, max = 255))]
    pub n_of_dices: u8,
    // 1 needs to be an option, to allow 100d1 => 100
    #[validate(range(min = 1, max = 255))]
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
                rand::thread_rng().gen_range(1..=self.dice_size) as i64
            } else {
                1
            }
        }
        roll_result
    }
}
