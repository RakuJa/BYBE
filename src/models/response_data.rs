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
    pub essential_data: bool,
    pub variant_data: bool,
    pub extra_data: bool,
    pub combat_data: bool,
    pub spell_casting_data: bool,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq)]
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
            core_data: if rd.essential_data {
                Some(cr.core_data)
            } else {
                None
            },
            variant_data: if rd.variant_data {
                Some(cr.variant_data)
            } else {
                None
            },
            extra_data: if rd.extra_data {
                Some(cr.extra_data)
            } else {
                None
            },
            info: if rd.extra_data { Some(cr.info) } else { None },
            spell_caster_data: if rd.spell_casting_data {
                Some(cr.spell_caster_data)
            } else {
                None
            },
            combat_data: if rd.combat_data {
                Some(cr.combat_data)
            } else {
                None
            },
        }
    }
}
