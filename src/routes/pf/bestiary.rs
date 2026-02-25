use crate::models::creature::creature_metadata::alignment_enum::AlignmentEnum;
use crate::models::creature::creature_metadata::creature_role::CreatureRoleEnum;
use crate::models::creature::creature_metadata::type_enum::CreatureTypeEnum;
use crate::models::creature::creature_metadata::variant_enum::CreatureVariant;
use crate::models::item::shield_struct::Shield;
use crate::models::response_data::CreatureResponseDataModifiers;
use crate::models::response_data::ResponseCreature;
use crate::models::routers_validator_structs::OrderEnum;
use crate::models::shared::rarity_enum::RarityEnum;
use crate::models::shared::size_enum::SizeEnum;

use crate::models::creature::creature_component::creature_combat::CreatureCombatData;
use crate::models::creature::creature_component::creature_combat::SavingThrows;
use crate::models::creature::creature_component::creature_core::CreatureCoreData;
use crate::models::creature::creature_component::creature_core::DerivedData;
use crate::models::creature::creature_component::creature_core::EssentialData;
use crate::models::creature::creature_component::creature_extra::AbilityScores;
use crate::models::creature::creature_component::creature_extra::CreatureExtraData;
use crate::models::creature::creature_component::creature_spellcaster::CreatureSpellcasterData;
use crate::models::creature::creature_component::creature_variant::CreatureVariantData;
use crate::models::pf_version_enum::GameSystemVersionEnum;

use crate::models::creature::items::action::Action;
use crate::models::creature::items::skill::Skill;
use crate::models::creature::items::spell::Spell;
use crate::models::creature::items::spellcaster_entry::SpellcasterEntry;
use crate::models::item::armor_struct::Armor;
use crate::models::item::weapon_struct::Weapon;

use crate::AppState;
use crate::models::bestiary_structs::CreatureSortEnum;
use crate::models::bestiary_structs::{BestiaryPaginatedRequest, BestiarySortData};
use crate::models::creature::creature_field_filter::CreatureFieldFilters;
use crate::models::db::sense::Sense;
use crate::models::routers_validator_structs::PaginatedRequest;
use crate::models::shared::game_system_enum::GameSystem;
use crate::services::bestiary_service;
use crate::services::bestiary_service::BestiaryResponse;
use crate::services::sanitizer::sanitize_id;
use actix_web::web::Query;
use actix_web::{Responder, get, post, web};
use utoipa::OpenApi;

pub fn init_endpoints(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/bestiary")
            .service(pf_get_bestiary_listing)
            .service(pf_get_elite_creature)
            .service(pf_get_weak_creature)
            .service(pf_get_creature)
            .service(pf_get_families_list)
            .service(pf_get_traits_list)
            .service(pf_get_sources_list)
            .service(pf_get_rarities_list)
            .service(pf_get_creature_types_list)
            .service(pf_get_creature_roles_list)
            .service(pf_get_sizes_list)
            .service(pf_get_alignments_list),
    );
}

pub fn init_docs() -> utoipa::openapi::OpenApi {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            pf_get_bestiary_listing,
            pf_get_families_list,
            pf_get_traits_list,
            pf_get_sources_list,
            pf_get_rarities_list,
            pf_get_sizes_list,
            pf_get_alignments_list,
            pf_get_creature_types_list,
            pf_get_creature_roles_list,
            pf_get_creature,
            pf_get_elite_creature,
            pf_get_weak_creature,
        ),
        components(schemas(
            BestiaryResponse,
            ResponseCreature,
            AlignmentEnum,
            RarityEnum,
            SizeEnum,
            CreatureTypeEnum,
            CreatureVariant,
            CreatureCoreData,
            EssentialData,
            DerivedData,
            CreatureVariantData,
            CreatureExtraData,
            CreatureCombatData,
            CreatureSpellcasterData,
            Sense,
            Spell,
            Shield,
            Weapon,
            Armor,
            SavingThrows,
            AbilityScores,
            Action,
            Skill,
            CreatureRoleEnum,
            SpellcasterEntry,
            GameSystemVersionEnum,
            OrderEnum,
            CreatureSortEnum
        ))
    )]
    struct ApiDoc;
    ApiDoc::openapi()
}

#[utoipa::path(
    post,
    path = "/bestiary/list",
    tags = ["pf", "bestiary"],
    request_body(
        content = CreatureFieldFilters,
        content_type = "application/json"
    ),
    params(
        PaginatedRequest, BestiarySortData
    ),
    responses(
        (status=200, description = "Successful Response", body = BestiaryResponse),
        (status=400, description = "Bad request.")
    ),
)]
#[post("/list")]
pub async fn pf_get_bestiary_listing(
    data: web::Data<AppState>,
    web::Json(body): web::Json<CreatureFieldFilters>,
    pagination: Query<PaginatedRequest>,
    sort_data: Query<BestiarySortData>,
) -> actix_web::Result<impl Responder> {
    Ok(web::Json(
        bestiary_service::get_bestiary_listing(
            &data,
            &body,
            &BestiaryPaginatedRequest {
                paginated_request: pagination.0,
                bestiary_sort_data: sort_data.0,
            },
            &GameSystem::Pathfinder,
        )
        .await,
    ))
}

#[utoipa::path(
    get,
    path = "/bestiary/families",
    tags = ["pf", "bestiary"],
    params(

    ),
    responses(
        (status=200, description = "Successful Response", body = [String]),
        (status=400, description = "Bad request.")
    ),
)]
#[get("/families")]
pub async fn pf_get_families_list(data: web::Data<AppState>) -> actix_web::Result<impl Responder> {
    Ok(web::Json(
        bestiary_service::get_families_list(&data, &GameSystem::Pathfinder).await,
    ))
}

#[utoipa::path(
    get,
    path = "/bestiary/traits",
    tags = ["pf", "bestiary"],
    params(

    ),
    responses(
        (status=200, description = "Successful Response", body = [String]),
        (status=400, description = "Bad request.")
    ),
)]
#[get("/traits")]
pub async fn pf_get_traits_list(data: web::Data<AppState>) -> actix_web::Result<impl Responder> {
    Ok(web::Json(
        bestiary_service::get_traits_list(&data, &GameSystem::Pathfinder).await,
    ))
}

#[utoipa::path(
    get,
    path = "/bestiary/sources",
    tags = ["pf", "bestiary"],
    params(

    ),
    responses(
        (status=200, description = "Successful Response", body = [String]),
        (status=400, description = "Bad request.")
    ),
)]
#[get("/sources")]
pub async fn pf_get_sources_list(data: web::Data<AppState>) -> actix_web::Result<impl Responder> {
    Ok(web::Json(
        bestiary_service::get_sources_list(&data, &GameSystem::Pathfinder).await,
    ))
}

#[utoipa::path(
    get,
    path = "/bestiary/rarities",
    tags = ["pf", "bestiary"],
    params(

    ),
    responses(
        (status=200, description = "Successful Response", body = [String]),
        (status=400, description = "Bad request.")
    ),
)]
#[get("/rarities")]
pub async fn pf_get_rarities_list(data: web::Data<AppState>) -> actix_web::Result<impl Responder> {
    Ok(web::Json(
        bestiary_service::get_rarities_list(&data, &GameSystem::Pathfinder).await,
    ))
}

#[utoipa::path(
    get,
    path = "/bestiary/sizes",
    tags = ["pf", "bestiary"],
    params(

    ),
    responses(
        (status=200, description = "Successful Response", body = [String]),
        (status=400, description = "Bad request.")
    ),
)]
#[get("/sizes")]
pub async fn pf_get_sizes_list(data: web::Data<AppState>) -> actix_web::Result<impl Responder> {
    Ok(web::Json(
        bestiary_service::get_sizes_list(&data, &GameSystem::Pathfinder).await,
    ))
}

#[utoipa::path(
    get,
    path = "/bestiary/alignments",
    tags = ["pf", "bestiary"],
    params(

    ),
    responses(
        (status=200, description = "Successful Response", body = [String]),
        (status=400, description = "Bad request.")
    ),
)]
#[get("/alignments")]
pub async fn pf_get_alignments_list(
    data: web::Data<AppState>,
) -> actix_web::Result<impl Responder> {
    Ok(web::Json(
        bestiary_service::get_alignments_list(&data, &GameSystem::Pathfinder).await,
    ))
}

#[utoipa::path(
    get,
    path = "/bestiary/creature_types",
    tags = ["pf", "bestiary"],
    params(

    ),
    responses(
        (status=200, description = "Successful Response", body = [String]),
        (status=400, description = "Bad request.")
    ),
)]
#[get("/creature_types")]
pub async fn pf_get_creature_types_list(
    data: web::Data<AppState>,
) -> actix_web::Result<impl Responder> {
    Ok(web::Json(
        bestiary_service::get_creature_types_list(&data, &GameSystem::Pathfinder).await,
    ))
}

#[utoipa::path(
    get,
    path = "/bestiary/creature_roles",
    tags = ["pf", "bestiary"],
    params(

    ),
    responses(
        (status=200, description = "Successful Response", body = [String]),
        (status=400, description = "Bad request.")
    ),
)]
#[get("/creature_roles")]
pub async fn pf_get_creature_roles_list() -> actix_web::Result<impl Responder> {
    Ok(web::Json(bestiary_service::get_creature_roles_list()))
}

#[utoipa::path(
    get,
    path = "/bestiary/base/{creature_id}",
    tags = ["pf", "bestiary"],
    params(
        ("creature_id" = String, Path, description = "id of the creature to fetch"),
        CreatureResponseDataModifiers,
    ),
    responses(
        (status=200, description = "Successful Response", body = ResponseCreature),
        (status=400, description = "Bad request.")
    ),
)]
#[get("/base/{creature_id}")]
pub async fn pf_get_creature(
    data: web::Data<AppState>,
    creature_id: web::Path<String>,
    response_data_mods: Query<CreatureResponseDataModifiers>,
) -> actix_web::Result<impl Responder> {
    Ok(web::Json(
        bestiary_service::get_creature(
            &data,
            sanitize_id(&creature_id)?,
            &response_data_mods.0,
            &GameSystem::Pathfinder,
        )
        .await,
    ))
}

#[utoipa::path(
    get,
    path = "/bestiary/elite/{creature_id}",
    tags = ["pf", "bestiary"],
    params(
        ("creature_id" = String, Path, description = "id of the creature to fetch"),
        CreatureResponseDataModifiers
    ),
    responses(
        (status=200, description = "Successful Response", body = ResponseCreature),
        (status=400, description = "Bad request.")
    ),
)]
#[get("/elite/{creature_id}")]
pub async fn pf_get_elite_creature(
    data: web::Data<AppState>,
    creature_id: web::Path<String>,
    response_data_mods: Query<CreatureResponseDataModifiers>,
) -> actix_web::Result<impl Responder> {
    Ok(web::Json(
        bestiary_service::get_elite_creature(
            &data,
            sanitize_id(&creature_id)?,
            &response_data_mods.0,
            &GameSystem::Pathfinder,
        )
        .await,
    ))
}

#[utoipa::path(
    get,
    path = "/bestiary/weak/{creature_id}",
    tags = ["pf", "bestiary"],
    params(
        ("creature_id" = String, Path, description = "id of the creature to fetch"),
        CreatureResponseDataModifiers,
    ),
    responses(
        (status=200, description = "Successful Response", body = ResponseCreature),
        (status=400, description = "Bad request.")
    ),
)]
#[get("/weak/{creature_id}")]
pub async fn pf_get_weak_creature(
    data: web::Data<AppState>,
    creature_id: web::Path<String>,
    response_data_mods: Query<CreatureResponseDataModifiers>,
) -> actix_web::Result<impl Responder> {
    Ok(web::Json(
        bestiary_service::get_weak_creature(
            &data,
            sanitize_id(&creature_id)?,
            &response_data_mods.0,
            &GameSystem::Pathfinder,
        )
        .await,
    ))
}
