use crate::models::creature::creature_metadata::alignment_enum::AlignmentEnum;
use crate::models::creature::creature_metadata::creature_role::CreatureRoleEnum;
use crate::models::creature::creature_metadata::type_enum::CreatureTypeEnum;
use crate::models::pf_version_enum::PathfinderVersionEnum;
use crate::models::shared::rarity_enum::RarityEnum;
use crate::models::shared::size_enum::SizeEnum;
use std::collections::HashSet;

pub struct FilterStruct {
    pub families: Option<Vec<String>>,
    pub traits: Option<Vec<String>>,
    pub rarities: Option<Vec<RarityEnum>>,
    pub sizes: Option<Vec<SizeEnum>>,
    pub alignments: Option<Vec<AlignmentEnum>>,
    pub creature_types: Option<Vec<CreatureTypeEnum>>,
    pub creature_roles: Option<Vec<CreatureRoleEnum>>,
    pub lvl_combinations: HashSet<String>,
    pub pathfinder_version: PathfinderVersionEnum,
}
