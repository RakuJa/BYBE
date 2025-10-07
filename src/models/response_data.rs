use crate::models::creature::creature_component::creature_combat::CreatureCombatData;
use crate::models::creature::creature_component::creature_core::CreatureCoreData;
use crate::models::creature::creature_component::creature_extra::CreatureExtraData;
use crate::models::creature::creature_component::creature_spellcaster::CreatureSpellcasterData;
use crate::models::creature::creature_component::creature_variant::CreatureVariantData;
use crate::models::creature::creature_struct::Creature;
use crate::models::item::armor_struct::ArmorData;
use crate::models::item::item_struct::Item;
use crate::models::item::shield_struct::ShieldData;
use crate::models::item::weapon_struct::WeaponData;
use crate::models::shared::game_system_enum::GameSystem;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Serialize, Deserialize, IntoParams, Default, Eq, PartialEq, Hash, Clone)]
pub struct ResponseDataModifiers {
    pub is_pwl_on: Option<bool>,
    pub extra_data: Option<bool>,
    pub combat_data: Option<bool>,
    pub spellcasting_data: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Hash, PartialEq, Eq)]
pub struct ResponseCreature {
    pub core_data: CreatureCoreData,
    pub variant_data: CreatureVariantData,
    pub extra_data: Option<CreatureExtraData>,
    pub combat_data: Option<CreatureCombatData>,
    pub spellcaster_data: Option<CreatureSpellcasterData>,
    pub game: GameSystem,
}

impl From<Creature> for ResponseCreature {
    fn from(cr: Creature) -> Self {
        Self {
            core_data: cr.core_data,
            variant_data: cr.variant_data,
            extra_data: cr.extra_data,
            spellcaster_data: cr.spellcaster_data,
            combat_data: cr.combat_data,
            game: cr.game_system,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, ToSchema, PartialEq, Eq, Debug)]
pub struct ResponseItem {
    pub core_item: Item,
    pub weapon_data: Option<WeaponData>,
    pub armor_data: Option<ArmorData>,
    pub shield_data: Option<ShieldData>,
    pub game: GameSystem,
}

impl From<(Item, GameSystem)> for ResponseItem {
    fn from(value: (Item, GameSystem)) -> Self {
        let item = value.0;
        let game_system = value.1;
        Self {
            core_item: item,
            weapon_data: None,
            armor_data: None,
            shield_data: None,
            game: game_system,
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct ResponseNpc {
    pub name: String,
    pub nickname: Option<String>,
    pub gender: String,
    pub ancestry: String,
    pub job: String,
    pub level: i64,
    pub culture: String,
    pub class: String,
    pub game: GameSystem,
}

#[derive(Serialize, Deserialize, ToSchema, Default)]
pub struct ShopListingResponse {
    pub(crate) results: Option<Vec<ResponseItem>>,
    pub(crate) count: usize,
    pub(crate) total: usize,
    pub(crate) game: GameSystem,
    pub(crate) next: Option<String>,
}

impl ShopListingResponse {
    pub const fn default_with_system(game_system: GameSystem) -> Self {
        Self {
            results: None,
            count: 0,
            total: 0,
            game: game_system,
            next: None,
        }
    }
}
