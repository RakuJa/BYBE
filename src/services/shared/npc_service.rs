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
use crate::models::npc::class_enum::{PfClass, SfClass};
use crate::models::npc::culture_enum::PfCulture;
use crate::models::npc::gender_enum::Gender;
use crate::models::npc::job_enum::{PfJob, SfJob};
use crate::models::npc::name_loader_struct::{NamesByAncestryRarity, NamesByCulture};
use crate::models::response_data::ResponseNpc;
use crate::models::routers_validator_structs::LevelData;
use crate::models::shared::game_system_enum::GameSystem;
use crate::traits::class_enum::ClassEnum;
use crate::traits::job_enum::JobEnum;
use crate::traits::name_system::{NameOrigin, NameOriginFilter};
use crate::traits::origin::ancestry::Ancestry;
use crate::traits::origin::context_size::ContextSize;
use crate::traits::origin::culture::Culture;
use crate::traits::origin::has_valid_genders::HasValidGenders;
use crate::traits::random_enum::RandomEnum;
use cached::proc_macro::once;

pub fn generate_random_npc<C, NF, J>(
    app_state: &AppState,
    npc_req_data: RandomNpcData<C, NF, J>,
) -> anyhow::Result<ResponseNpc>
where
    C: ClassEnum,
    NF: NameOriginFilter,
    J: JobEnum,
{
    let game_system = &npc_req_data.name_origin_filter.clone().into();
    let (gender, ancestry, culture, name_origin) =
        if let Some(ancestries) = npc_req_data.name_origin_filter.get_ancestries() {
            let random_ancestry = get_random_ancestry(Some(ancestries));
            let valid_genders = random_ancestry.get_valid_genders();

            let gender = if let Some(g_filter) = &npc_req_data.gender_filter.as_ref() {
                let filtered: Vec<_> = valid_genders
                    .into_iter()
                    .filter(|g| g_filter.contains(g))
                    .collect();
                get_random_gender(Some(filtered))?
            } else {
                Gender::filtered_random(&valid_genders)
            };

            (
                gender,
                random_ancestry.clone(),
                get_random_culture(None),
                npc_req_data
                    .name_origin_filter
                    .to_name_origin(None, Some(random_ancestry))?,
            )
        } else if let Some(cultures) = &npc_req_data.name_origin_filter.get_cultures() {
            let random_culture = get_random_culture(Some(cultures.to_vec()));
            (
                Gender::random(),
                get_random_ancestry(None),
                random_culture.clone(),
                npc_req_data
                    .name_origin_filter
                    .to_name_origin(Some(random_culture), None)?,
            )
        } else {
            bail!("Illegal state found")
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
        ancestry: ancestry.to_string(),
        culture: culture.to_string(),
        nickname: if npc_req_data.generate_nickname.unwrap_or(false) {
            generate_random_nickname(&app_state.nick_json_path)
        } else {
            None
        },
        job: get_random_job(npc_req_data.job_filter.unwrap_or_default()),
        class: get_random_class(npc_req_data.class_filter.unwrap_or_default()),
        game: *game_system,
    })
}

pub fn get_cultures_list() -> Vec<PfCulture> {
    PfCulture::iter().collect()
}

pub fn get_genders_list() -> Vec<Gender> {
    Gender::iter().collect()
}

pub fn get_random_ancestry<T: Ancestry>(filter: Option<Vec<T>>) -> T {
    T::filtered_random(&filter.unwrap_or_default())
}

pub fn get_random_culture<T: Culture>(filter: Option<Vec<T>>) -> T {
    T::filtered_random(&filter.unwrap_or_default())
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

pub fn get_random_class<C>(filter: Vec<C>) -> String
where
    C: ClassEnum,
{
    C::filtered_random(&filter).to_string()
}

pub fn get_random_job<J>(filter: Vec<J>) -> String
where
    J: JobEnum,
{
    J::filtered_random(&filter).to_string()
}

pub fn generate_random_names<N>(data: RandomNameData<N>, name_path: &str) -> Vec<String>
where
    N: NameOrigin,
{
    let name_origin = data.origin;
    let key = name_origin
        .get_ancestry()
        .map(|x| x.to_string())
        .unwrap_or_else(|| name_origin.get_culture().unwrap_or_default().to_string());
    let token_size = name_origin.context_size();
    let max_length = name_origin.get_average_name_length();
    let gender = data
        .gender
        .unwrap_or_else(|| name_origin.get_random_gender());
    let chain = name_origin.get_name_builder(name_path).get(&(key.clone(), gender.clone())).unwrap_or_else(|| {
        panic!(
            "Could not fetch the initializer for the given Key (Location/Ancestry) {key} and Gender {gender}"
        )
    }).clone();

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

pub fn get_ancestry_name_builder(
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

pub fn get_culture_name_builder(
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
