use crate::models::creature::creature_metadata::alignment_enum::AlignmentEnum;
use crate::models::creature::creature_metadata::creature_role::CreatureRoleEnum;
use crate::models::creature::creature_metadata::rarity_enum::RarityEnum;
use crate::models::creature::creature_metadata::size_enum::SizeEnum;
use crate::models::creature::creature_metadata::type_enum::CreatureTypeEnum;
use crate::models::item::item_metadata::type_enum::ItemTypeEnum;
use serde::{Deserialize, Serialize};
use utoipa::IntoParams;
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
}

#[derive(Serialize, Deserialize, IntoParams, Validate, Eq, PartialEq, Hash)]
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
