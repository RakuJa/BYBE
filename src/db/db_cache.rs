use crate::models::creature::Creature;
use cached::proc_macro::cached;

#[derive(Default, Hash, Eq, PartialEq, Clone)]
pub struct SortedVectorsByField {
    pub unordered_creatures: Vec<Creature>,

    pub order_by_id_ascending: Vec<Creature>,
    pub order_by_id_descending: Vec<Creature>,

    pub order_by_name_ascending: Vec<Creature>,
    pub order_by_name_descending: Vec<Creature>,

    pub order_by_hp_ascending: Vec<Creature>,
    pub order_by_hp_descending: Vec<Creature>,

    pub order_by_level_ascending: Vec<Creature>,
    pub order_by_level_descending: Vec<Creature>,

    pub order_by_family_ascending: Vec<Creature>,
    pub order_by_family_descending: Vec<Creature>,

    pub order_by_alignment_ascending: Vec<Creature>,
    pub order_by_alignment_descending: Vec<Creature>,

    pub order_by_size_ascending: Vec<Creature>,
    pub order_by_size_descending: Vec<Creature>,

    pub order_by_rarity_ascending: Vec<Creature>,
    pub order_by_rarity_descending: Vec<Creature>,
}

#[derive(Default, Hash, Eq, PartialEq, Clone)]
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

#[cached(time = 604800, sync_writes = true)]
pub fn from_db_data_to_filter_cache(data: Vec<Creature>) -> RuntimeFieldsValues {
    let mut fields_values_cache = RuntimeFieldsValues::default();
    // The right structure would be an hashset, but it does not implement hash..
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
    fields_values_cache
}

#[cached(time = 604800, sync_writes = true)]
pub fn from_db_data_to_sorted_vectors(unordered_creatures: Vec<Creature>) -> SortedVectorsByField {
    let mut sorted_cache = SortedVectorsByField::default();

    let mut sort_stage = unordered_creatures.clone();

    sorted_cache.unordered_creatures = unordered_creatures.clone();

    sort_stage.sort_by_key(|cr| cr.id);
    sorted_cache.order_by_id_ascending = sort_stage.clone();
    sort_stage.reverse();
    sorted_cache.order_by_id_descending = sort_stage.clone();

    sort_stage.sort_by_key(|cr| cr.name.clone());
    sorted_cache.order_by_name_ascending = sort_stage.clone();
    sort_stage.reverse();
    sorted_cache.order_by_name_descending = sort_stage.clone();

    sort_stage.sort_by_key(|cr| cr.hp);
    sorted_cache.order_by_hp_ascending = sort_stage.clone();
    sort_stage.reverse();
    sorted_cache.order_by_hp_descending = sort_stage.clone();

    sort_stage.sort_by_key(|cr| cr.level);
    sorted_cache.order_by_level_ascending = sort_stage.clone();
    sort_stage.reverse();
    sorted_cache.order_by_level_descending = sort_stage.clone();

    sort_stage.sort_by_key(|cr| cr.family.clone());
    sorted_cache.order_by_family_ascending = sort_stage.clone();
    sort_stage.reverse();
    sorted_cache.order_by_family_descending = sort_stage.clone();

    sort_stage.sort_by_key(|cr| cr.alignment.clone());
    sorted_cache.order_by_alignment_ascending = sort_stage.clone();
    sort_stage.reverse();
    sorted_cache.order_by_alignment_descending = sort_stage.clone();

    sort_stage.sort_by_key(|cr| cr.size.clone());
    sorted_cache.order_by_size_ascending = sort_stage.clone();
    sort_stage.reverse();
    sorted_cache.order_by_size_descending = sort_stage.clone();

    sort_stage.sort_by_key(|cr| cr.rarity.clone());
    sorted_cache.order_by_rarity_ascending = sort_stage.clone();
    sort_stage.reverse();
    sorted_cache.order_by_rarity_descending = sort_stage.clone();

    sorted_cache
}
