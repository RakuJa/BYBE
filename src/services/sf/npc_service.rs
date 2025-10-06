use crate::models::npc::ancestry_enum::SfAncestry;
use crate::traits::random_enum::RandomEnum;

pub fn get_random_ancestry(filter: Option<Vec<SfAncestry>>) -> SfAncestry {
    SfAncestry::filtered_random(&filter.unwrap_or_default())
}
