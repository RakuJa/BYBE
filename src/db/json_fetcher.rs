use crate::AppState;
use crate::models::npc::name_loader_struct::{Names, NickNameData};
#[cfg(feature = "cache")]
use cached::cached;
use std::fs;

pub fn get_nickname_data_from_json(path: &str) -> anyhow::Result<NickNameData> {
    Ok(serde_json::from_str(read_file_as_str(path).as_str())?)
}

pub fn get_names_from_json(path: &str) -> anyhow::Result<Names> {
    Ok(serde_json::from_str(read_file_as_str(path).as_str())?)
}

/// Returns the parsed names data, caching the result
#[cfg_attr(
    feature = "cache",
    cached(key = "String", convert = r##"{ app_state.name_json_path.clone() }"##)
)]
pub fn get_names(app_state: &AppState) -> Names {
    get_names_from_json(&app_state.name_json_path).expect("Unable to read names JSON")
}

/// Returns the parsed nickname data, caching the result
#[cfg_attr(
    feature = "cache",
    cached(key = "String", convert = r##"{ app_state.nick_json_path.clone() }"##)
)]
pub fn get_nicknames(app_state: &AppState) -> NickNameData {
    get_nickname_data_from_json(&app_state.nick_json_path).expect("Unable to read nicknames JSON")
}

fn read_file_as_str(path: &str) -> String {
    fs::read_to_string(path).expect("Unable to read file")
}
