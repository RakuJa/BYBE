use crate::AppState;
use crate::models::npc::request_npc_struct::{AncestryData, RandomNameData, RandomNpcData};
use anyhow::bail;
use itertools::Itertools;
use nanorand::Rng;
use nanorand::WyRand;
use nomina::capitalize_each_substring;
use std::collections::HashMap;
use std::str::FromStr;
use strum::IntoEnumIterator;

use crate::db::json_fetcher::{get_names_from_json, get_nickname_data_from_json};
use crate::models::npc::ancestry_enum::{PfAncestry, SfAncestry};
use crate::models::npc::class_enum::{ClassFilter, PfClass, SfClass};
use crate::models::npc::culture_enum::{PfCulture, SfCulture};
use crate::models::npc::gender_enum::Gender;
use crate::models::npc::job_enum::{JobFilter, PfJob, SfJob};
use crate::models::npc::name_loader_struct::{NamesByAncestryRarity, NamesByCulture};
use crate::models::npc::name_origin_enum::{
    NameSystemOrigin, NameSystemOriginFilter, PfNameOrigin, PfNameOriginFilter, SfNameOrigin,
    SfNameOriginFilter,
};
use crate::models::response_data::ResponseNpc;
use crate::models::routers_validator_structs::LevelData;
use crate::models::shared::game_system_enum::GameSystem;
use crate::services::pf::npc_service as pf_npc_service;
use crate::services::sf::npc_service as sf_npc_service;
use crate::traits::ancestry::average_name_length::AverageNameLength;
use crate::traits::ancestry::context_size::ContextSize;
use crate::traits::ancestry::has_valid_genders::HasValidGenders;
use crate::traits::random_enum::RandomEnum;
use cached::proc_macro::once;

fn process_ancestry<T>(
    ancestry: T,
    gender_filter: Option<&Vec<Gender>>,
    origin_constructor: impl FnOnce(T) -> NameSystemOrigin,
) -> anyhow::Result<(Gender, NameSystemOrigin)>
where
    T: Clone + HasValidGenders,
{
    let valid_genders = ancestry.get_valid_genders();
    let name_origin = origin_constructor(ancestry);

    let gender = if let Some(g_filter) = gender_filter {
        let filtered: Vec<_> = valid_genders
            .into_iter()
            .filter(|g| g_filter.contains(g))
            .collect();
        get_random_gender(Some(filtered))?
    } else {
        Gender::filtered_random(&valid_genders)
    };

    Ok((gender, name_origin))
}

pub fn generate_random_npc(
    app_state: &AppState,
    npc_req_data: RandomNpcData,
) -> anyhow::Result<ResponseNpc> {
    let game_system = match &npc_req_data.name_origin_filter {
        NameSystemOriginFilter::FromPf(_) => GameSystem::Pathfinder,
        NameSystemOriginFilter::FromSf(_) => GameSystem::Starfinder,
    };
    let (gender, name_origin) = match npc_req_data.name_origin_filter {
        NameSystemOriginFilter::FromPf(pf) => match pf.unwrap_or_default() {
            PfNameOriginFilter::FromAncestry(ancestries) => process_ancestry(
                pf_npc_service::get_random_ancestry(ancestries),
                npc_req_data.gender_filter.as_ref(),
                |a| NameSystemOrigin::FromPf(Some(PfNameOrigin::FromAncestry(Some(a)))),
            )?,
            PfNameOriginFilter::FromCulture(pf_locations) => {
                let culture = pf_npc_service::get_random_culture(pf_locations);
                (
                    Gender::random(),
                    NameSystemOrigin::FromPf(Some(PfNameOrigin::FromCulture(Some(culture)))),
                )
            }
        },
        NameSystemOriginFilter::FromSf(sf) => match sf.unwrap_or_default() {
            SfNameOriginFilter::FromAncestry(ancestries) => process_ancestry(
                sf_npc_service::get_random_ancestry(ancestries),
                npc_req_data.gender_filter.as_ref(),
                |a| NameSystemOrigin::FromSf(Some(SfNameOrigin::FromAncestry(Some(a)))),
            )?,
        },
    };

    let (ancestry, culture) = match &name_origin {
        NameSystemOrigin::FromPf(pf) => match pf.clone().unwrap_or_default() {
            PfNameOrigin::FromAncestry(a) => (
                a.unwrap_or_default().to_string(),
                PfCulture::random().to_string(),
            ),
            PfNameOrigin::FromCulture(c) => (
                PfAncestry::random().to_string(),
                c.unwrap_or_default().to_string(),
            ),
        },
        NameSystemOrigin::FromSf(sf) => match sf.clone().unwrap_or_default() {
            SfNameOrigin::FromAncestry(a) => (
                a.unwrap_or_default().to_string(),
                SfCulture::random().to_string(),
            ),
        },
    };

    Ok(ResponseNpc {
        name: generate_random_names(
            RandomNameData {
                name_max_length: npc_req_data.name_max_length,
                max_n_of_names: Some(1),
                gender: Some(gender.clone()),
                origin: name_origin,
            },
            &app_state.name_json_path,
        )
        .first()
        .unwrap()
        .clone(),
        gender: gender.to_string(),
        level: get_random_level(npc_req_data.level_filter),
        ancestry,
        culture,
        nickname: if npc_req_data.generate_nickname.unwrap_or(false) {
            generate_random_nickname(&app_state.nick_json_path)
        } else {
            None
        },
        job: get_random_job(npc_req_data.job_filter.unwrap_or(match &game_system {
            GameSystem::Pathfinder => JobFilter::FromPf(None),
            GameSystem::Starfinder => JobFilter::FromSf(None),
        })),
        class: get_random_class(npc_req_data.class_filter.unwrap_or(match &game_system {
            GameSystem::Pathfinder => ClassFilter::FromPf(None),
            GameSystem::Starfinder => ClassFilter::FromSf(None),
        })),
        game: game_system,
    })
}

pub fn get_cultures_list() -> Vec<PfCulture> {
    PfCulture::iter().collect()
}

pub fn get_genders_list() -> Vec<Gender> {
    Gender::iter().collect()
}

pub fn get_jobs_list(game_system: &GameSystem) -> Vec<String> {
    match game_system {
        GameSystem::Pathfinder => PfJob::iter().map(|x| x.to_string()).collect(),
        GameSystem::Starfinder => SfJob::iter().map(|x| x.to_string()).collect(),
    }
}

pub fn get_classes_list(game_system: &GameSystem) -> Vec<String> {
    match game_system {
        GameSystem::Pathfinder => PfClass::iter().map(|x| x.to_string()).collect(),
        GameSystem::Starfinder => SfClass::iter().map(|x| x.to_string()).collect(),
    }
}

pub fn get_random_job(filter: JobFilter) -> String {
    match filter {
        JobFilter::FromPf(pj) => PfJob::filtered_random(&pj.unwrap_or_default()).to_string(),
        JobFilter::FromSf(sj) => SfJob::filtered_random(&sj.unwrap_or_default()).to_string(),
    }
}

pub fn get_ancestries_list(game_system: GameSystem) -> Vec<AncestryData> {
    match game_system {
        GameSystem::Pathfinder => PfAncestry::iter()
            .map(|a| AncestryData {
                ancestry: a.to_string(),
                valid_genders: PfAncestry::get_valid_genders(&a),
            })
            .collect(),

        GameSystem::Starfinder => SfAncestry::iter()
            .map(|a| AncestryData {
                ancestry: a.to_string(),
                valid_genders: SfAncestry::get_valid_genders(&a),
            })
            .collect(),
    }
}

pub fn get_random_level(lvl_data: Option<LevelData>) -> i64 {
    let (min, max) = lvl_data.map_or((None, None), |lvls| {
        if lvls.is_data_valid() {
            (lvls.min_level, lvls.max_level)
        } else {
            (None, None)
        }
    });
    WyRand::new().generate_range(min.unwrap_or(-1)..=max.unwrap_or(25))
}

pub fn get_random_gender(filter: Option<Vec<Gender>>) -> anyhow::Result<Gender> {
    if let Some(whitelist) = filter {
        if whitelist.is_empty() {
            bail!(
                "Whitelist is empty, as such there are 0 possible genders to choose from. Don't pass a whitelist or populate it."
            )
        }
        Ok(Gender::filtered_random(&whitelist))
    } else {
        Ok(Gender::filtered_random(&filter.unwrap_or_default()))
    }
}

pub fn get_random_class(filter: ClassFilter) -> String {
    match filter {
        ClassFilter::FromPf(pc) => PfClass::filtered_random(&pc.unwrap_or_default()).to_string(),
        ClassFilter::FromSf(sc) => SfClass::filtered_random(&sc.unwrap_or_default()).to_string(),
    }
}

pub fn generate_random_names(data: RandomNameData, name_path: &str) -> Vec<String> {
    let (chain, token_size, max_length) = match data.origin {
        NameSystemOrigin::FromPf(pf) => match pf.unwrap_or_default() {
            PfNameOrigin::FromAncestry(ancestry) => {
                let a = ancestry.unwrap_or_else(PfAncestry::random);
                let gender = data
                    .gender
                    .unwrap_or_else(|| Gender::filtered_random(&a.get_valid_genders()));
                (prepare_pf_ancestry_names_builder(name_path).get(&(a.to_string(), gender.clone())).unwrap_or_else(|| {
                        panic!(
                            "Could not fetch the initializer for the given Ancestry {a} and Gender {gender}"
                        )
                    }).clone(),
                     a.context_size(), a.get_average_name_length())
            }
            PfNameOrigin::FromCulture(location) => {
                let l = location.unwrap_or_else(PfCulture::random);

                let gender = data.gender.unwrap_or_else(Gender::random);
                (prepare_pf_culture_names_builder(name_path).get(&(l.to_string(), gender.clone())).unwrap_or_else(|| {
                        panic!(
                            "Could not fetch the initializer for the given Location {l} and Gender {gender}"
                        )
                    }).clone(),
                     l.context_size(), l.get_average_name_length())
            }
        },
        NameSystemOrigin::FromSf(sf) => match sf.unwrap_or_default() {
            SfNameOrigin::FromAncestry(ancestry) => {
                let a = ancestry.unwrap_or_else(SfAncestry::random);
                let gender = data
                    .gender
                    .unwrap_or_else(|| Gender::filtered_random(&a.get_valid_genders()));
                (prepare_sf_ancestry_names_builder(name_path).get(&(a.to_string(), gender.clone())).unwrap_or_else(|| {
                        panic!(
                            "Could not fetch the initializer for the given Ancestry {a} and Gender {gender}"
                        )
                    }).clone(),
                     a.context_size(), a.get_average_name_length())
            }
        },
    };

    (0..data.max_n_of_names.unwrap_or(10))
        .map(|_| {
            nomina::generate_name(
                &chain,
                token_size,
                data.name_max_length.unwrap_or(max_length),
            )
        })
        .unique()
        .map(|n| capitalize_each_substring(n.as_str(), " "))
        .collect()
}

pub fn generate_random_nickname(nickname_path: &str) -> Option<String> {
    if let Ok(data) = get_nickname_data_from_json(nickname_path) {
        let adj_list = data.terms.adjective;
        let nouns = data.terms.nouns;

        if let Some(adj) = adj_list.get(WyRand::new().generate_range(0..adj_list.len()))
            && let Some(noun) = nouns.get(WyRand::new().generate_range(0..nouns.len()))
        {
            Some(match WyRand::new().generate_range(0..2) {
                0 => format!("{adj} {noun}"),
                _ => format!("The {adj} {noun}"),
            })
        } else {
            log::error!("Cannot generate nickname, empty json?");
            None
        }
    } else {
        log::error!("Cannot generate nickname, missing json?");
        None
    }
}

#[once(sync_writes = true)]
pub fn prepare_pf_ancestry_names_builder(
    json_path: &str,
) -> HashMap<(String, Gender), HashMap<String, Vec<char>>> {
    let names = get_names_from_json(json_path).unwrap();
    get_ancestry_name_builder(names.pf_names.by_ancestry, GameSystem::Pathfinder)
}

#[once(sync_writes = true)]
pub fn prepare_sf_ancestry_names_builder(
    json_path: &str,
) -> HashMap<(String, Gender), HashMap<String, Vec<char>>> {
    let names = get_names_from_json(json_path).unwrap();
    get_ancestry_name_builder(names.sf_names.by_ancestry, GameSystem::Starfinder)
}

fn get_ancestry_name_builder(
    ancestry_struct: NamesByAncestryRarity,
    game_system: GameSystem,
) -> HashMap<(String, Gender), HashMap<String, Vec<char>>> {
    let mut chains = HashMap::new();
    let x = ancestry_struct.rarity;
    let all_ancestries: Vec<_> = x
        .common
        .into_iter()
        .chain(x.uncommon)
        .chain(x.rare)
        .chain(x.unique)
        .collect();
    for names_by_rarity in all_ancestries {
        let ancestry = names_by_rarity.ancestry;
        for el in names_by_rarity.names {
            let gender = el.gender;
            let curr_names: Vec<_> = el.list.iter().map(String::as_str).collect();
            let context_size = match game_system {
                GameSystem::Pathfinder => PfAncestry::from_str(ancestry.as_str())
                    .unwrap_or_default()
                    .context_size(),
                GameSystem::Starfinder => SfAncestry::from_str(ancestry.as_str())
                    .unwrap_or_default()
                    .context_size(),
            };
            chains.insert(
                (ancestry.clone(), gender),
                nomina::build_chain(curr_names.as_slice(), context_size),
            );
        }
    }
    chains
}

fn get_culture_name_builder(
    names_by_culture: Vec<NamesByCulture>,
    game_system: GameSystem,
) -> HashMap<(String, Gender), HashMap<String, Vec<char>>> {
    let mut chains = HashMap::new();
    for culture_struct in names_by_culture {
        let culture = culture_struct.culture;
        for el in culture_struct.names {
            let gender = el.gender;
            let curr_names: Vec<_> = el.list.iter().map(String::as_str).collect();
            let context_size = match game_system {
                GameSystem::Pathfinder => PfCulture::from_str(culture.as_str())
                    .unwrap_or_default()
                    .context_size(),
                GameSystem::Starfinder => panic!("Starfinder culture is not supported!"),
            };
            chains.insert(
                (culture.clone(), gender),
                nomina::build_chain(curr_names.as_slice(), context_size),
            );
        }
    }

    chains
}

#[once(sync_writes = true)]
pub fn prepare_pf_culture_names_builder(
    json_path: &str,
) -> HashMap<(String, Gender), HashMap<String, Vec<char>>> {
    let names = get_names_from_json(json_path).unwrap();
    get_culture_name_builder(names.pf_names.by_culture, GameSystem::Pathfinder)
}
