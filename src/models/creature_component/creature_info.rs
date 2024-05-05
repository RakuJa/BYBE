use crate::models::creature_component::creature_combat::CreatureCombatData;
use crate::models::creature_component::creature_core::CreatureCoreData;
use crate::models::creature_component::creature_extra::CreatureExtraData;
use crate::models::creature_component::creature_spell_caster::CreatureSpellCasterData;
use crate::models::creature_metadata::creature_role::CreatureRoleEnum;
use crate::models::scales_struct::creature_scales::CreatureScales;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema, Hash, PartialEq, Eq)]
pub struct CreatureInfo {
    pub roles: BTreeMap<CreatureRoleEnum, i64>,
    pub locale: Vec<String>,
}

impl
    From<(
        &CreatureCoreData,
        &CreatureExtraData,
        &CreatureCombatData,
        &CreatureSpellCasterData,
        &CreatureScales,
        &Regex,
    )> for CreatureInfo
{
    fn from(
        tuple: (
            &CreatureCoreData,
            &CreatureExtraData,
            &CreatureCombatData,
            &CreatureSpellCasterData,
            &CreatureScales,
            &Regex,
        ),
    ) -> Self {
        Self {
            roles: CreatureRoleEnum::from_creature_with_given_scales(
                tuple.0, tuple.1, tuple.2, tuple.3, tuple.4, tuple.5,
            ),
            locale: vec![],
        }
    }
}
