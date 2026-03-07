use crate::models::creature::creature_metadata::creature_role::CreatureRoleEnum;
use crate::models::creature::creature_metadata::type_enum::CreatureTypeEnum;
use crate::models::shared::alignment_enum::AlignmentEnum;
use crate::models::shared::pf_version_enum::GameSystemVersionEnum;
use crate::models::shared::rarity_enum::RarityEnum;
use crate::models::shared::size_enum::SizeEnum;
use serde::{Deserialize, Serialize};
#[allow(unused_imports)] // it's actually used in the example schema
use serde_json::json;
use std::collections::BTreeMap;
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
