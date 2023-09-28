use crate::models::creature_metadata_enums::{AlignmentEnum, RarityEnum, SizeEnum};
use crate::models::creature_sort_enums::{OrderEnum, SortEnum};
use crate::models::encounter_structs::EncounterDifficultyEnum;
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
    pub min_hp_filter: Option<i16>,
    pub max_hp_filter: Option<i16>,
    pub min_level_filter: Option<i8>,
    pub max_level_filter: Option<i8>,
    pub is_melee_filter: Option<bool>,
    pub is_ranged_filter: Option<bool>,
    pub is_spell_caster_filter: Option<bool>,
}

#[derive(Serialize, Deserialize, IntoParams, Validate, Copy, Clone)]
pub struct SortData {
    #[param(inline)]
    pub sort_key: Option<SortEnum>,
    #[param(inline)]
    pub order_by: Option<OrderEnum>,
}

#[derive(Serialize, Deserialize, IntoParams, Validate)]
pub struct RandomEncounterData {
    pub family: Option<String>,
    pub rarity: Option<RarityEnum>,
    pub size: Option<SizeEnum>,
    pub alignment: Option<AlignmentEnum>,
    pub encounter_difficulty: EncounterDifficultyEnum,
}

#[derive(Serialize, Deserialize, IntoParams, Validate)]
pub struct PaginatedRequest {
    pub cursor: u32,
    #[validate(range(min = 1, max = 100))]
    pub page_size: u8,
}

impl Default for PaginatedRequest {
    fn default() -> Self {
        PaginatedRequest {
            cursor: 0,
            page_size: 100,
        }
    }
}
