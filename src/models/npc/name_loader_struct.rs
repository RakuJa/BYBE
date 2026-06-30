use crate::models::npc::gender_enum::Gender;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Names {
    pub pf_names: GameSystemNames,
    pub sf_names: GameSystemNames,
}

#[derive(Deserialize, Clone)]
pub struct GameSystemNames {
    pub by_ancestry: NamesByAncestryRarity,
    pub by_culture: Vec<NamesByCulture>,
}

#[derive(Deserialize, Clone)]
pub struct NamesByAncestryRarity {
    pub rarity: NamesByRarity,
}

#[derive(Deserialize, Clone)]
//#[allow(dead_code)]
pub struct NamesByRarity {
    pub common: Vec<NamesByAncestry>,
    pub uncommon: Vec<NamesByAncestry>,
    pub rare: Vec<NamesByAncestry>,
    pub unique: Vec<NamesByAncestry>,
}

#[derive(Deserialize, Clone)]
pub struct NamesByAncestry {
    pub ancestry: String,
    pub names: Vec<NamesByGender>,
}

#[derive(Deserialize, Clone)]
pub struct NamesByCulture {
    pub culture: String,
    pub names: Vec<NamesByGender>,
}

#[derive(Deserialize, Clone)]
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
