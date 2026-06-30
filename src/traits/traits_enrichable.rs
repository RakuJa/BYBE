use crate::models::shared::trait_data::TraitData;

pub trait TraitsEnrichable {
    fn entity_id(&self) -> i64;
    fn set_traits(&mut self, traits: Vec<TraitData>);
    fn entity_name() -> &'static str
    where
        Self: Sized;
}
