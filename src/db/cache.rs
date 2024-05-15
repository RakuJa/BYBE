use crate::db::data_providers::fetcher::{
    fetch_traits_associated_with_creatures, fetch_unique_values_of_field,
};
use crate::models::creature_metadata::alignment_enum::AlignmentEnum;
use crate::AppState;
use strum::IntoEnumIterator;

#[derive(Default, Eq, PartialEq, Clone)]
pub struct RuntimeFieldsValues {
    pub list_of_levels: Vec<String>,
    pub list_of_families: Vec<String>,
    pub list_of_traits: Vec<String>,
    pub list_of_sources: Vec<String>,
    pub list_of_alignments: Vec<String>,
    pub list_of_sizes: Vec<String>,
    pub list_of_rarities: Vec<String>,
    pub list_of_creature_types: Vec<String>,
}

pub async fn from_db_data_to_filter_cache(app_state: &AppState) -> RuntimeFieldsValues {
    let cache = &app_state.runtime_fields_cache.clone();
    if let Some(runtime_fields) = cache.get(&0) {
        return RuntimeFieldsValues {
            list_of_levels: runtime_fields.list_of_levels.clone(),
            list_of_families: runtime_fields.list_of_families.clone(),
            list_of_traits: runtime_fields.list_of_traits.clone(),
            list_of_sources: runtime_fields.list_of_sources.clone(),
            list_of_alignments: runtime_fields.list_of_alignments.clone(),
            list_of_sizes: runtime_fields.list_of_sizes.clone(),
            list_of_rarities: runtime_fields.list_of_rarities.clone(),
            list_of_creature_types: runtime_fields.list_of_creature_types.clone(),
        };
    }
    let fields_values_cache = RuntimeFieldsValues {
        list_of_levels: fetch_unique_values_of_field(&app_state.conn, "CREATURE_CORE", "level")
            .await
            .unwrap_or_default(),
        list_of_families: fetch_unique_values_of_field(&app_state.conn, "CREATURE_CORE", "family")
            .await
            .unwrap(),
        list_of_traits: fetch_traits_associated_with_creatures(&app_state.conn)
            .await
            .unwrap_or_default(),
        list_of_sources: fetch_unique_values_of_field(&app_state.conn, "CREATURE_CORE", "source")
            .await
            .unwrap_or_default(),
        list_of_alignments: AlignmentEnum::iter().map(|x| x.to_string()).collect(),
        list_of_sizes: fetch_unique_values_of_field(&app_state.conn, "CREATURE_CORE", "size")
            .await
            .unwrap_or_default(),
        list_of_rarities: fetch_unique_values_of_field(&app_state.conn, "CREATURE_CORE", "rarity")
            .await
            .unwrap_or_default(),
        list_of_creature_types: fetch_unique_values_of_field(
            &app_state.conn,
            "CREATURE_CORE",
            "cr_type",
        )
        .await
        .unwrap_or_default(),
    };
    cache.insert(0, fields_values_cache.clone());
    fields_values_cache
}
