use crate::models::creature::creature_component::creature_combat::CreatureCombatData;
use crate::models::creature::creature_component::creature_core::CreatureCoreData;
use crate::models::creature::creature_component::creature_extra::CreatureExtraData;
use crate::models::creature::creature_component::creature_spell_caster::CreatureSpellCasterData;
use crate::models::creature::creature_component::creature_variant::CreatureVariantData;
use crate::models::creature::creature_struct::Creature;
use crate::models::item::item_struct::Item;
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

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct ResponseItem {
    pub core_item: Item,
}

impl From<Item> for ResponseItem {
    fn from(value: Item) -> Self {
        Self { core_item: value }
    }
}
