use crate::models::npc::name_loader_struct::{Names, NickNameData};
use cached::proc_macro::cached;
use std::fs;

pub fn get_nickname_data_from_json(path: &str) -> anyhow::Result<NickNameData> {
    Ok(serde_json::from_str(
        read_file_as_str(path.to_owned()).as_str(),
    )?)
}

pub fn get_names_from_json(path: &str) -> anyhow::Result<Names> {
    Ok(serde_json::from_str(
        read_file_as_str(path.to_owned()).as_str(),
    )?)
}

#[cached]
fn read_file_as_str(path: String) -> String {
    fs::read_to_string(path).expect("Unable to read file")
}
