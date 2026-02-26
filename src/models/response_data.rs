use crate::models::creature::creature_component::creature_combat::CreatureCombatData;
use crate::models::creature::creature_component::creature_core::CreatureCoreData;
use crate::models::creature::creature_component::creature_extra::CreatureExtraData;
use crate::models::creature::creature_component::creature_spellcaster::CreatureSpellcasterData;
use crate::models::creature::creature_component::creature_variant::CreatureVariantData;
use crate::models::creature::creature_struct::Creature;
use crate::models::encounter_structs::EncounterChallengeEnum;
use crate::models::hazard::hazard_struct::Hazard;
use crate::models::item::armor_struct::ArmorData;
use crate::models::item::item_struct::Item;
use crate::models::item::shield_struct::ShieldData;
use crate::models::item::weapon_struct::WeaponData;
use crate::models::shared::game_system_enum::GameSystem;
use crate::services::url_calculator::next_url;
use crate::traits::response::listing_response::ListingResponse;
use crate::traits::url::paginated_request_ext::PaginatedRequestExt;
use serde::{Deserialize, Serialize};
#[allow(unused_imports)] // it's used for Schema
use serde_json::json;
use std::collections::BTreeMap;
use utoipa::{IntoParams, ToSchema};

#[derive(Serialize, Deserialize, IntoParams, Default, Eq, PartialEq, Hash, Clone)]
pub struct CreatureResponseDataModifiers {
    pub is_pwl_on: Option<bool>,
    pub extra_data: Option<bool>,
    pub combat_data: Option<bool>,
    pub spellcasting_data: Option<bool>,
}

#[derive(Serialize, Deserialize, ToSchema, Default)]
pub struct HazardListingResponse {
    results: Option<Vec<ResponseHazard>>,
    count: usize,
    total: usize,
    next: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, PartialEq, Eq, Debug)]
pub struct ResponseHazard {
    pub core_hazard: Hazard,
    pub game: GameSystem,
}

impl From<(Hazard, GameSystem)> for ResponseHazard {
    fn from(value: (Hazard, GameSystem)) -> Self {
        let item = value.0;
        let game_system = value.1;
        Self {
            core_hazard: item,
            game: game_system,
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema, Default)]
pub struct BestiaryResponse {
    results: Option<Vec<ResponseCreature>>,
    count: usize,
    total: usize,
    next: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Hash, PartialEq, Eq, Debug)]
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

impl ListingResponse for ShopListingResponse {
    type Item = ResponseItem;

    fn from_results(
        results: Vec<Self::Item>,
        count: usize,
        next: Option<String>,
        total: usize,
    ) -> Self {
        Self {
            results: Some(results),
            count,
            next,
            total,
            game: GameSystem::Starfinder,
        }
    }
}

impl ListingResponse for BestiaryResponse {
    type Item = Creature;

    fn from_results(
        results: Vec<Self::Item>,
        count: usize,
        next: Option<String>,
        total: usize,
    ) -> Self {
        Self {
            results: Some(results.into_iter().map(ResponseCreature::from).collect()),
            count,
            next,
            total,
        }
    }
}

impl ListingResponse for HazardListingResponse {
    type Item = ResponseHazard;

    fn from_results(
        results: Vec<Self::Item>,
        count: usize,
        next: Option<String>,
        total: usize,
    ) -> Self {
        Self {
            results: Some(results),
            count,
            next,
            total,
        }
    }
}

pub fn convert_result_to_response<P, R>(
    pagination: &P,
    result: anyhow::Result<(u32, Vec<R::Item>)>,
) -> R
where
    P: PaginatedRequestExt,
    R: ListingResponse,
{
    match result {
        Ok((total, items)) => {
            let count = items.len();
            let next = (count >= pagination.paginated_request().page_size.unsigned_abs() as usize)
                .then(|| next_url(pagination, count as u32));
            R::from_results(items, count, next, total as usize)
        }
        Err(_) => R::default(),
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct EncounterInfoResponse {
    #[schema(minimum = 0, example = 40)]
    pub(crate) experience: i64,
    pub(crate) challenge: EncounterChallengeEnum,
    #[schema(example = json!({EncounterChallengeEnum::Trivial: 40, EncounterChallengeEnum::Low: 60, EncounterChallengeEnum::Moderate: 80, EncounterChallengeEnum::Severe: 120, EncounterChallengeEnum::Extreme: 160, EncounterChallengeEnum::Impossible: 320}))]
    pub(crate) encounter_exp_levels: BTreeMap<EncounterChallengeEnum, i64>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct EncounterContent {
    pub(crate) creatures: Option<Vec<ResponseCreature>>,
    pub(crate) hazards: Option<Vec<ResponseHazard>>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct RandomEncounterGeneratorResponse {
    pub(crate) results: EncounterContent,
    pub(crate) count: usize,
    pub(crate) encounter_info: EncounterInfoResponse,
    pub(crate) game: GameSystem,
}
