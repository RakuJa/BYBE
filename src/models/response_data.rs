use crate::models::creature::Creature;
use crate::models::creature_component::creature_combat::CreatureCombatData;
use crate::models::creature_component::creature_core::CreatureCoreData;
use crate::models::creature_component::creature_extra::CreatureExtraData;
use crate::models::creature_component::creature_info::CreatureInfo;
use crate::models::creature_component::creature_spell_caster::CreatureSpellCasterData;
use crate::models::creature_component::creature_variant::CreatureVariantData;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Serialize, Deserialize, IntoParams, Default, Eq, PartialEq, Hash, Clone, Validate)]
pub struct ResponseData {
    pub core_data: Option<bool>,
    pub variant_data: Option<bool>,
    pub extra_data: Option<bool>,
    pub combat_data: Option<bool>,
    pub spell_casting_data: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Hash, PartialEq)]
pub struct ResponseCreature {
    pub core_data: Option<CreatureCoreData>,
    pub variant_data: Option<CreatureVariantData>,
    pub extra_data: Option<CreatureExtraData>,
    pub combat_data: Option<CreatureCombatData>,
    pub spell_caster_data: Option<CreatureSpellCasterData>,
    pub info: Option<CreatureInfo>,
}

impl From<(Creature, &ResponseData)> for ResponseCreature {
    fn from(value: (Creature, &ResponseData)) -> Self {
        let cr = value.0;
        let rd = value.1;
        Self {
            core_data: if rd.core_data.is_none() || !rd.core_data.unwrap() {
                None
            } else {
                Some(cr.core_data)
            },
            variant_data: if rd.variant_data.is_none() || !rd.variant_data.unwrap() {
                None
            } else {
                Some(cr.variant_data)
            },
            extra_data: if rd.extra_data.is_none() || !rd.extra_data.unwrap() {
                None
            } else {
                Some(cr.extra_data)
            },
            info: if rd.extra_data.is_none() || !rd.extra_data.unwrap() {
                None
            } else {
                Some(cr.info)
            },
            spell_caster_data: if rd.spell_casting_data.is_none() || !rd.spell_casting_data.unwrap()
            {
                None
            } else {
                Some(cr.spell_caster_data)
            },
            combat_data: if rd.combat_data.is_none() || !rd.combat_data.unwrap() {
                None
            } else {
                Some(cr.combat_data)
            },
        }
    }
}
