use crate::models::npc::ancestry_enum::PfAncestry;
use crate::models::npc::culture_enum::PfCulture;
use crate::traits::random_enum::RandomEnum;

pub fn get_random_ancestry(filter: Option<Vec<PfAncestry>>) -> PfAncestry {
    PfAncestry::filtered_random(&filter.unwrap_or_default())
}

pub fn get_random_culture(filter: Option<Vec<PfCulture>>) -> PfCulture {
    PfCulture::filtered_random(&filter.unwrap_or_default())
}
