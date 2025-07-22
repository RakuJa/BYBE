use crate::env;
use crate::models::npc::name_loader_struct::{Names, NickNameData};
use crate::models::npc::request_npc_struct::{
    AncestryData, NameOriginFilter, RandomNameData, RandomNpcData,
};
use anyhow::bail;
use itertools::Itertools;
use nanorand::Rng;
use nanorand::WyRand;
use nomina::capitalize_each_substring;
use std::collections::HashMap;
use std::fs;
use strum::IntoEnumIterator;

use crate::models::npc::ancestry_enum::Ancestry;
use crate::models::npc::class_enum::Class;
use crate::models::npc::culture_enum::Culture;
use crate::models::npc::gender_enum::Gender;
use crate::models::npc::job_enum::Job;
use crate::models::npc::request_npc_struct::NameOrigin;
use crate::models::response_data::ResponseNpc;
use crate::traits::random_enum::RandomEnum;
use cached::proc_macro::cached;
use cached::proc_macro::once;

pub fn generate_random_npc(npc_req_data: RandomNpcData) -> anyhow::Result<ResponseNpc> {
    let origin = npc_req_data.name_origin_filter;
    let (gender, name_origin) = match origin {
        NameOriginFilter::FromAncestry(ancestries) => {
            let ancestry = get_random_ancestry(ancestries);
            let valid_genders = ancestry.get_valid_genders();
            let n_og = NameOrigin::FromAncestry(Some(ancestry));
            if let Some(g_filter) = npc_req_data.gender_filter {
                {
                    (
                        get_random_gender(Some(
                            valid_genders
                                .into_iter()
                                .filter(|g| g_filter.contains(g))
                                .collect::<Vec<_>>(),
                        ))?,
                        n_og,
                    )
                }
            } else {
                (Gender::filtered_random(&valid_genders), n_og)
            }
        }
        NameOriginFilter::FromCulture(locations) => (
            Gender::random(),
            NameOrigin::FromCulture(Some(get_random_culture(locations))),
        ),
    };

    let (ancestry, culture) = match &name_origin {
        NameOrigin::FromAncestry(a) => (a.clone().unwrap_or_default(), Culture::random()),
        NameOrigin::FromCulture(c) => (Ancestry::random(), c.clone().unwrap_or_default()),
    };
    Ok(ResponseNpc {
        name: generate_random_names(RandomNameData {
            name_max_length: npc_req_data.name_max_length,
            max_n_of_names: Some(1),
            gender: Some(gender.clone()),
            origin: name_origin,
        })
        .first()
        .unwrap()
        .clone(),
        gender,
        ancestry,
        culture,
        nickname: if npc_req_data.generate_nickname.unwrap_or(false) {
            generate_random_nickname()
        } else {
            None
        },
        job: get_random_job(npc_req_data.job_filter),
        class: get_random_class(npc_req_data.class_filter),
    })
}

pub fn get_ancestries_list() -> Vec<AncestryData> {
    Ancestry::iter()
        .map(|a| AncestryData {
            ancestry: a.clone(),
            valid_genders: Ancestry::get_valid_genders(&a),
        })
        .collect()
}

pub fn get_cultures_list() -> Vec<Culture> {
    Culture::iter().collect()
}

pub fn get_genders_list() -> Vec<Gender> {
    Gender::iter().collect()
}

pub fn get_jobs_list() -> Vec<Job> {
    Job::iter().collect()
}

pub fn get_classes_list() -> Vec<Class> {
    Class::iter().collect()
}

pub fn get_random_job(filter: Option<Vec<Job>>) -> Job {
    Job::filtered_random(&filter.unwrap_or_default())
}

pub fn get_random_ancestry(filter: Option<Vec<Ancestry>>) -> Ancestry {
    Ancestry::filtered_random(&filter.unwrap_or_default())
}

pub fn get_random_culture(filter: Option<Vec<Culture>>) -> Culture {
    Culture::filtered_random(&filter.unwrap_or_default())
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

pub fn get_random_class(filter: Option<Vec<Class>>) -> Class {
    Class::filtered_random(&filter.unwrap_or_default())
}

pub fn generate_random_names(data: RandomNameData) -> Vec<String> {
    let ancestry_chain = prepare_ancestry_names_builder();
    let location_chain = prepare_culture_names_builder();
    let (chain, token_size, max_length) = match data.origin {
        NameOrigin::FromAncestry(ancestry) => {
            let a = ancestry.unwrap_or_else(Ancestry::random);
            let gender = data
                .gender
                .unwrap_or_else(|| Gender::filtered_random(&a.get_valid_genders()));
            (ancestry_chain.get(&(a.clone(), gender.clone())).unwrap_or_else(|| {
                panic!(
                    "Could not fetch the initializer for the given Ancestry {a} and Gender {gender}"
                )
            }),
            a.get_default_order_size(), a.get_default_name_length())
        }
        NameOrigin::FromCulture(location) => {
            let l = location.unwrap_or_else(Culture::random);

            let gender = data.gender.unwrap_or_else(Gender::random);
            (location_chain.get(&(l.clone(), gender.clone())).unwrap_or_else(|| {
                panic!(
                    "Could not fetch the initializer for the given Location {l} and Gender {gender}"
                )
            }),
            l.get_default_order_size(), l.get_default_name_length())
        }
    };

    (0..data.max_n_of_names.unwrap_or(10))
        .map(|_| {
            nomina::generate_name(
                chain,
                token_size,
                data.name_max_length.unwrap_or(max_length),
            )
        })
        .unique()
        .map(|n| capitalize_each_substring(n.as_str(), " "))
        .collect()
}

pub fn generate_random_nickname() -> Option<String> {
    if let Ok(data) = get_nickname_data_from_json() {
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
fn prepare_ancestry_names_builder() -> HashMap<(Ancestry, Gender), HashMap<String, Vec<char>>> {
    let names = get_names_from_json().unwrap();
    let mut chains = HashMap::new();
    for ancestry_struct in names.by_ancestry {
        let ancestry = ancestry_struct.ancestry;
        for el in ancestry_struct.names {
            let gender = el.gender;
            let curr_names: Vec<_> = el.list.iter().map(String::as_str).collect();
            chains.insert(
                (ancestry.clone(), gender),
                nomina::build_chain(
                    curr_names.as_slice(),
                    Ancestry::get_default_order_size(&ancestry),
                ),
            );
        }
    }
    chains
}

#[once(sync_writes = true)]
fn prepare_culture_names_builder() -> HashMap<(Culture, Gender), HashMap<String, Vec<char>>> {
    let names = get_names_from_json().unwrap();
    let mut chains = HashMap::new();
    for culture_struct in names.by_culture {
        let culture = culture_struct.culture;
        for el in culture_struct.names {
            let gender = el.gender;
            let curr_names: Vec<_> = el.list.iter().map(String::as_str).collect();
            chains.insert(
                (culture.clone(), gender),
                nomina::build_chain(
                    curr_names.as_slice(),
                    Culture::get_default_order_size(&culture),
                ),
            );
        }
    }
    chains
}

fn get_nickname_data_from_json() -> anyhow::Result<NickNameData> {
    Ok(serde_json::from_str(
        read_file_as_str(env::var("NICKNAMES_PATH").expect("Error fetching nick json")).as_str(),
    )?)
}

#[cached]
fn read_file_as_str(path: String) -> String {
    fs::read_to_string(path).expect("Unable to read file")
}

fn get_names_from_json() -> anyhow::Result<Names> {
    Ok(serde_json::from_str(
        read_file_as_str(env::var("NAMES_PATH").expect("Error fetching names json")).as_str(),
    )?)
}
