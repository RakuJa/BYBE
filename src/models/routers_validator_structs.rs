use crate::models::creature_metadata::alignment_enum::AlignmentEnum;
use crate::models::creature_metadata::creature_role::CreatureRoleEnum;
use crate::models::creature_metadata::rarity_enum::RarityEnum;
use crate::models::creature_metadata::size_enum::SizeEnum;
use crate::models::creature_metadata::type_enum::CreatureTypeEnum;
use crate::models::pf_version_enum::PathfinderVersionEnum;
use serde::{Deserialize, Serialize};
use utoipa::IntoParams;
use validator::Validate;

#[derive(Serialize, Deserialize, IntoParams, Validate)]
pub struct FieldFilters {
    pub name_filter: Option<String>,
    pub family_filter: Option<String>,
    pub rarity_filter: Option<RarityEnum>,
    pub size_filter: Option<SizeEnum>,
    pub alignment_filter: Option<AlignmentEnum>,
    pub role_filter: Option<CreatureRoleEnum>,
    pub type_filter: Option<CreatureTypeEnum>,
    pub role_threshold: Option<i64>,
    pub min_hp_filter: Option<i64>,
    pub max_hp_filter: Option<i64>,
    pub min_level_filter: Option<i64>,
    pub max_level_filter: Option<i64>,
    pub is_melee_filter: Option<bool>,
    pub is_ranged_filter: Option<bool>,
    pub is_spell_caster_filter: Option<bool>,
    pub pathfinder_version: Option<PathfinderVersionEnum>,
}

#[derive(Serialize, Deserialize, IntoParams, Validate, Eq, PartialEq, Hash)]
pub struct PaginatedRequest {
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
