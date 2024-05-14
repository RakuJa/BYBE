use crate::models::creature_metadata::alignment_enum::AlignmentEnum;
use crate::models::creature_metadata::creature_role::CreatureRoleEnum;
use crate::models::creature_metadata::rarity_enum::RarityEnum;
use crate::models::creature_metadata::size_enum::SizeEnum;
use crate::models::creature_metadata::type_enum::CreatureTypeEnum;
use crate::models::creature_metadata::variant_enum::CreatureVariant;
use crate::models::response_data::OptionalData;
use crate::models::response_data::ResponseCreature;

use crate::models::creature_component::creature_combat::CreatureCombatData;
use crate::models::creature_component::creature_combat::SavingThrows;
use crate::models::creature_component::creature_core::CreatureCoreData;
use crate::models::creature_component::creature_extra::AbilityScores;
use crate::models::creature_component::creature_extra::CreatureExtraData;
use crate::models::creature_component::creature_spell_caster::CreatureSpellCasterData;
use crate::models::creature_component::creature_variant::CreatureVariantData;

use crate::models::items::action::Action;
use crate::models::items::skill::Skill;
use crate::models::items::spell::Spell;
use crate::models::items::spell_caster_entry::SpellCasterEntry;
use crate::models::items::weapon::Weapon;

use crate::models::routers_validator_structs::{FieldFilters, PaginatedRequest};
use crate::services::bestiary_service;
use crate::services::bestiary_service::BestiaryResponse;
use crate::AppState;
use actix_web::{error, get, web, Responder, Result};
use actix_web_validator::Query;
use utoipa::OpenApi;

pub fn init_endpoints(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/bestiary")
            .service(get_bestiary)
            .service(get_elite_creature)
            .service(get_weak_creature)
            .service(get_creature)
            .service(get_families_list)
            .service(get_traits_list)
            .service(get_sources_list)
            .service(get_rarities_list)
            .service(get_creature_types_list)
            .service(get_creature_roles_list)
            .service(get_sizes_list)
            .service(get_alignments_list),
    );
}

pub fn init_docs(doc: &mut utoipa::openapi::OpenApi) {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            get_bestiary,
            get_families_list,
            get_traits_list,
            get_sources_list,
            get_rarities_list,
            get_sizes_list,
            get_alignments_list,
            get_creature_types_list,
            get_creature_roles_list,
            get_creature,
            get_elite_creature,
            get_weak_creature,
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
            CreatureVariantData,
            CreatureExtraData,
            CreatureCombatData,
            CreatureSpellCasterData,
            Spell,
            Weapon,
            SavingThrows,
            AbilityScores,
            Action,
            Skill,
            CreatureRoleEnum,
            SpellCasterEntry
        ))
    )]
    struct ApiDoc;

    doc.merge(ApiDoc::openapi());
}

#[utoipa::path(
    get,
    path = "/bestiary/list",
    tag = "bestiary",
    params(
        FieldFilters, PaginatedRequest
    ),
    responses(
        (status=200, description = "Successful Response", body = BestiaryResponse),
        (status=400, description = "Bad request.")
    ),
)]
#[get("/list")]
pub async fn get_bestiary(
    data: web::Data<AppState>,
    filters: Query<FieldFilters>,
    pagination: Query<PaginatedRequest>,
) -> Result<impl Responder> {
    Ok(web::Json(
        bestiary_service::get_bestiary(&data, &filters.0, &pagination.0).await,
    ))
}

#[utoipa::path(
    get,
    path = "/bestiary/families",
    tag = "bestiary",
    params(

    ),
    responses(
        (status=200, description = "Successful Response", body = [String]),
        (status=400, description = "Bad request.")
    ),
)]
#[get("/families")]
pub async fn get_families_list(data: web::Data<AppState>) -> Result<impl Responder> {
    Ok(web::Json(bestiary_service::get_families_list(&data).await))
}

#[utoipa::path(
    get,
    path = "/bestiary/traits",
    tag = "bestiary",
    params(

    ),
    responses(
        (status=200, description = "Successful Response", body = [String]),
        (status=400, description = "Bad request.")
    ),
)]
#[get("/traits")]
pub async fn get_traits_list(data: web::Data<AppState>) -> Result<impl Responder> {
    Ok(web::Json(bestiary_service::get_traits_list(&data).await))
}

#[utoipa::path(
    get,
    path = "/bestiary/sources",
    tag = "bestiary",
    params(

    ),
    responses(
        (status=200, description = "Successful Response", body = [String]),
        (status=400, description = "Bad request.")
    ),
)]
#[get("/sources")]
pub async fn get_sources_list(data: web::Data<AppState>) -> Result<impl Responder> {
    Ok(web::Json(bestiary_service::get_sources_list(&data).await))
}

#[utoipa::path(
    get,
    path = "/bestiary/rarities",
    tag = "bestiary",
    params(

    ),
    responses(
        (status=200, description = "Successful Response", body = [String]),
        (status=400, description = "Bad request.")
    ),
)]
#[get("/rarities")]
pub async fn get_rarities_list(data: web::Data<AppState>) -> Result<impl Responder> {
    Ok(web::Json(bestiary_service::get_rarities_list(&data).await))
}

#[utoipa::path(
    get,
    path = "/bestiary/sizes",
    tag = "bestiary",
    params(

    ),
    responses(
        (status=200, description = "Successful Response", body = [String]),
        (status=400, description = "Bad request.")
    ),
)]
#[get("/sizes")]
pub async fn get_sizes_list(data: web::Data<AppState>) -> Result<impl Responder> {
    Ok(web::Json(bestiary_service::get_sizes_list(&data).await))
}

#[utoipa::path(
    get,
    path = "/bestiary/alignments",
    tag = "bestiary",
    params(

    ),
    responses(
        (status=200, description = "Successful Response", body = [String]),
        (status=400, description = "Bad request.")
    ),
)]
#[get("/alignments")]
pub async fn get_alignments_list(data: web::Data<AppState>) -> Result<impl Responder> {
    Ok(web::Json(
        bestiary_service::get_alignments_list(&data).await,
    ))
}

#[utoipa::path(
    get,
    path = "/bestiary/creature_types",
    tag = "bestiary",
    params(

    ),
    responses(
        (status=200, description = "Successful Response", body = [String]),
        (status=400, description = "Bad request.")
    ),
)]
#[get("/creature_types")]
pub async fn get_creature_types_list(data: web::Data<AppState>) -> Result<impl Responder> {
    Ok(web::Json(
        bestiary_service::get_creature_types_list(&data).await,
    ))
}

#[utoipa::path(
    get,
    path = "/bestiary/creature_roles",
    tag = "bestiary",
    params(

    ),
    responses(
        (status=200, description = "Successful Response", body = [String]),
        (status=400, description = "Bad request.")
    ),
)]
#[get("/creature_roles")]
pub async fn get_creature_roles_list() -> Result<impl Responder> {
    Ok(web::Json(bestiary_service::get_creature_roles_list().await))
}

#[utoipa::path(
    get,
    path = "/bestiary/base/{creature_id}",
    tag = "bestiary",
    params(
        ("creature_id" = String, Path, description = "id of the creature to fetch"),
        OptionalData,
    ),
    responses(
        (status=200, description = "Successful Response", body = ResponseCreature),
        (status=400, description = "Bad request.")
    ),
)]
#[get("/base/{creature_id}")]
pub async fn get_creature(
    data: web::Data<AppState>,
    creature_id: web::Path<String>,
    optional_data: Query<OptionalData>,
) -> Result<impl Responder> {
    Ok(web::Json(
        bestiary_service::get_creature(&data, sanitize_id(&creature_id)?, &optional_data.0).await,
    ))
}

#[utoipa::path(
    get,
    path = "/bestiary/elite/{creature_id}",
    tag = "bestiary",
    params(
        ("creature_id" = String, Path, description = "id of the creature to fetch"),
        OptionalData
    ),
    responses(
        (status=200, description = "Successful Response", body = ResponseCreature),
        (status=400, description = "Bad request.")
    ),
)]
#[get("/elite/{creature_id}")]
pub async fn get_elite_creature(
    data: web::Data<AppState>,
    creature_id: web::Path<String>,
    response_data: Query<OptionalData>,
) -> Result<impl Responder> {
    Ok(web::Json(
        bestiary_service::get_elite_creature(&data, sanitize_id(&creature_id)?, &response_data.0)
            .await,
    ))
}

#[utoipa::path(
    get,
    path = "/bestiary/weak/{creature_id}",
    tag = "bestiary",
    params(
        ("creature_id" = String, Path, description = "id of the creature to fetch"),
        OptionalData,
    ),
    responses(
        (status=200, description = "Successful Response", body = ResponseCreature),
        (status=400, description = "Bad request.")
    ),
)]
#[get("/weak/{creature_id}")]
pub async fn get_weak_creature(
    data: web::Data<AppState>,
    creature_id: web::Path<String>,
    response_data: Query<OptionalData>,
) -> Result<impl Responder> {
    Ok(web::Json(
        bestiary_service::get_weak_creature(&data, sanitize_id(&creature_id)?, &response_data.0)
            .await,
    ))
}

fn sanitize_id(creature_id: &str) -> Result<i64> {
    let id = creature_id.parse::<i64>();
    match id {
        Ok(s) => Ok(s),
        Err(e) => Err(error::ErrorNotFound(e)),
    }
}
