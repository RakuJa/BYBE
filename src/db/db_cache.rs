use crate::AppState;
use crate::models::creature::Creature;

#[derive(Default, Eq, PartialEq, Clone)]
pub struct RuntimeFieldsValues {
    pub list_of_ids: Vec<String>,
    pub list_of_levels: Vec<String>,
    pub list_of_families: Vec<String>,
    pub list_of_traits: Vec<String>,
    pub list_of_alignments: Vec<String>,
    pub list_of_sizes: Vec<String>,
    pub list_of_rarities: Vec<String>,
    pub list_of_creature_types: Vec<String>,
}

pub fn from_db_data_to_filter_cache(app_state: &AppState, data: Vec<Creature>) -> RuntimeFieldsValues {
    let mut fields_values_cache = RuntimeFieldsValues::default();
    let cache = &app_state.runtime_fields_cache.clone();
    if let Some(runtime_fields) = cache.get(&0) {
        return RuntimeFieldsValues{
            list_of_ids: runtime_fields.list_of_ids.clone(),
            list_of_levels: runtime_fields.list_of_levels.clone(),
            list_of_families: runtime_fields.list_of_families.clone(),
            list_of_traits: runtime_fields.list_of_traits.clone(),
            list_of_alignments: runtime_fields.list_of_alignments.clone(),
            list_of_sizes: runtime_fields.list_of_sizes.clone(),
            list_of_rarities: runtime_fields.list_of_rarities.clone(),
            list_of_creature_types: runtime_fields.list_of_creature_types.clone(),
        }
    }
    for curr_creature in data {
        let id = curr_creature.id.to_string();
        let lvl = curr_creature.level.to_string();
        let family = if curr_creature.family.is_some() {
            curr_creature.family.unwrap()
        } else {
            "-".to_string()
        };
        let alignment = curr_creature.alignment.to_string();
        let size = curr_creature.size.to_string();
        let rarity = curr_creature.rarity.to_string();
        let creature_type = curr_creature.creature_type.to_string();

        if !fields_values_cache.list_of_ids.contains(&id) {
            fields_values_cache.list_of_ids.push(id);
        }
        if !fields_values_cache.list_of_levels.contains(&lvl) {
            fields_values_cache.list_of_levels.push(lvl);
        }
        if !fields_values_cache.list_of_families.contains(&family) {
            fields_values_cache.list_of_families.push(family);
        }

        curr_creature.traits.iter().for_each(|single_trait| {
            if !fields_values_cache.list_of_traits.contains(single_trait) {
                fields_values_cache
                    .list_of_traits
                    .push(single_trait.to_string())
            }
        });

        if !fields_values_cache.list_of_alignments.contains(&alignment) {
            fields_values_cache.list_of_alignments.push(alignment);
        }
        if !fields_values_cache.list_of_sizes.contains(&size) {
            fields_values_cache.list_of_sizes.push(size);
        }
        if !fields_values_cache.list_of_rarities.contains(&rarity) {
            fields_values_cache.list_of_rarities.push(rarity);
        }
        if !fields_values_cache
            .list_of_creature_types
            .contains(&creature_type)
        {
            fields_values_cache
                .list_of_creature_types
                .push(creature_type);
        }
    }
    cache.insert(0, fields_values_cache.clone());

    fields_values_cache
}
