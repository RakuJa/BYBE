use crate::models::creature::creature_component::creature_combat::CreatureCombatData;
use crate::models::creature::creature_component::creature_core::CreatureCoreData;
use crate::models::creature::creature_component::creature_extra::CreatureExtraData;
use crate::models::creature::creature_component::creature_spell_caster::CreatureSpellcasterData;
use crate::models::creature::creature_component::creature_variant::CreatureVariantData;
use crate::models::creature::creature_struct::Creature;
use crate::models::item::armor_struct::ArmorData;
use crate::models::item::item_struct::Item;
use crate::models::item::shield_struct::ShieldData;
use crate::models::item::weapon_struct::WeaponData;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Serialize, Deserialize, IntoParams, Default, Eq, PartialEq, Hash, Clone)]
pub struct ResponseDataModifiers {
    pub is_pwl_on: Option<bool>,
    pub extra_data: Option<bool>,
    pub combat_data: Option<bool>,
    pub spell_casting_data: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Hash, PartialEq, Eq)]
pub struct ResponseCreature {
    pub core_data: CreatureCoreData,
    pub variant_data: CreatureVariantData,
    pub extra_data: Option<CreatureExtraData>,
    pub combat_data: Option<CreatureCombatData>,
    pub spell_caster_data: Option<CreatureSpellcasterData>,
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
    pub weapon_data: Option<WeaponData>,
    pub armor_data: Option<ArmorData>,
    pub shield_data: Option<ShieldData>,
}

impl From<Item> for ResponseItem {
    fn from(value: Item) -> Self {
        Self {
            core_item: value,
            weapon_data: None,
            armor_data: None,
            shield_data: None,
        }
    }
}
