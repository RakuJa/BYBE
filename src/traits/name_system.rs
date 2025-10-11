use crate::models::npc::gender_enum::Gender;
use crate::models::shared::game_system_enum::GameSystem;
use crate::traits::filter::Filter;
use crate::traits::origin::ancestry::Ancestry;
use crate::traits::origin::average_name_length::AverageNameLength;
use crate::traits::origin::context_size::ContextSize;
use crate::traits::origin::culture::Culture;
use crate::traits::origin::has_valid_genders::HasValidGenders;
use crate::traits::random_enum::RandomEnum;
use std::collections::HashMap;
use std::fmt::{Debug, Display};

pub trait NameOriginFilter: RandomEnum + Filter + Into<GameSystem> + Clone {
    type NameOriginType: NameOrigin; // used for conversion
    type AncestryType: Ancestry + Into<Self> + Display + ToString + Debug;

    type CultureType: Culture + Display + ToString + Debug;

    fn ancestries_have_at_least_one_valid_gender(&self, g_filter: Vec<Gender>) -> bool {
        self.get_ancestries().is_none_or(|a_list| {
            a_list
                .iter()
                .any(|x| x.has_at_least_one_gender_in_common(g_filter.clone()))
        })
    }
    fn get_ancestries(&self) -> Option<Vec<Self::AncestryType>>;

    fn get_cultures(&self) -> Option<Vec<Self::CultureType>>;

    fn to_name_origin(
        &self,
        c: Option<Self::CultureType>,
        a: Option<Self::AncestryType>,
    ) -> anyhow::Result<Self::NameOriginType>;
}

pub trait NameOrigin: RandomEnum + ContextSize + AverageNameLength {
    type AncestryType: Ancestry + Into<Self> + Display + ToString;
    type CultureType: Culture + TryInto<Self> + Display + ToString;

    fn get_random_gender(&self) -> Gender;
    fn get_random_ancestry() -> Self::AncestryType;

    fn get_ancestry(&self) -> Option<Self::AncestryType>;

    fn get_culture(&self) -> Option<Self::CultureType>;

    fn get_name_builder(
        &self,
        json_path: &str,
    ) -> HashMap<(String, Gender), HashMap<String, Vec<char>>>;
}
