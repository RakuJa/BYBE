use crate::models::npc::culture_enum::Culture;
use crate::models::{
    npc::{ancestry_enum::Ancestry, gender_enum::Gender},
    shared::rarity_enum::RarityEnum,
};
use serde::Deserialize;

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct Names {
    rarity: RarityEnum,
    pub by_ancestry: Vec<NamesByAncestry>,
    pub by_culture: Vec<NamesByCulture>,
}

#[derive(Deserialize)]
pub struct NamesByAncestry {
    pub ancestry: Ancestry,
    pub names: Vec<NamesByGender>,
}

#[derive(Deserialize)]
pub struct NamesByCulture {
    pub culture: Culture,
    pub names: Vec<NamesByGender>,
}

#[derive(Deserialize)]
pub struct NamesByGender {
    pub gender: Gender,
    pub list: Vec<String>,
}

#[derive(Deserialize, Clone)]
pub struct NickNameData {
    pub terms: TermsData,
}

#[derive(Deserialize, Clone)]
pub struct TermsData {
    pub adjective: Vec<String>,
    pub nouns: Vec<String>,
}
