use crate::models::creature::Creature;
use crate::models::creature_component::creature_combat::CreatureCombatData;
use crate::models::creature_component::creature_core::CreatureCoreData;
use crate::models::creature_component::creature_extra::CreatureExtraData;
use crate::models::creature_component::creature_spell_caster::CreatureSpellCasterData;
use crate::models::creature_component::creature_variant::CreatureVariantData;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Serialize, Deserialize, IntoParams, Default, Eq, PartialEq, Hash, Clone, Validate)]
pub struct OptionalData {
    pub extra_data: Option<bool>,
    pub combat_data: Option<bool>,
    pub spell_casting_data: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Hash, PartialEq)]
pub struct ResponseCreature {
    pub core_data: CreatureCoreData,
    pub variant_data: CreatureVariantData,
    pub extra_data: Option<CreatureExtraData>,
    pub combat_data: Option<CreatureCombatData>,
    pub spell_caster_data: Option<CreatureSpellCasterData>,
}

impl From<Creature> for ResponseCreature {
    fn from(value: Creature) -> Self {
        let cr = value;
        Self {
            core_data: cr.core_data,
            variant_data: cr.variant_data,
            extra_data: cr.extra_data,
            spell_caster_data: cr.spell_caster_data,
            combat_data: cr.combat_data,
        }
    }
}
