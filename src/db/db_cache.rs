use crate::models::creature::Creature;
use crate::models::creature_metadata_enums::{AlignmentEnum, RarityEnum, SizeEnum};
use redis::RedisError;
use std::collections::HashMap;

#[derive(Default)]
pub struct DbCache {
    pub lists: SortedVectorCache,
    pub filters: FiltersCache,
}

#[derive(Default)]
pub struct SortedVectorCache {
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

#[derive(Default)]
pub struct FiltersCache {
    pub filtered_by_id: HashMap<i32, Vec<Creature>>,
    pub filtered_by_level: HashMap<i8, Vec<Creature>>,
    pub filtered_by_family: HashMap<Option<String>, Vec<Creature>>,
    pub filtered_by_alignment: HashMap<AlignmentEnum, Vec<Creature>>,
    pub filtered_by_size: HashMap<SizeEnum, Vec<Creature>>,
    pub filtered_by_rarity: HashMap<RarityEnum, Vec<Creature>>,
    pub filtered_by_melee: HashMap<bool, Vec<Creature>>,
    pub filtered_by_ranged: HashMap<bool, Vec<Creature>>,
    pub filtered_by_spell_caster: HashMap<bool, Vec<Creature>>,
}

pub fn from_db_data_to_filter_cache(data: &Result<Vec<Creature>, RedisError>) -> FiltersCache {
    let mut filters_cache = FiltersCache::default();
    // This is inefficient AF, I need to study a way to share
    // those objects between hashmaps, as they are immutable.
    // The problem is that having a reference requires a lifetime, and
    // right now the fields have different lifetime than the struct
    if let Ok(creatures) = data {
        for curr_creature in creatures {
            filters_cache
                .filtered_by_id
                .entry(curr_creature.id)
                .or_insert_with(Vec::new)
                .push(curr_creature.clone());

            filters_cache
                .filtered_by_level
                .entry(curr_creature.level)
                .or_insert_with(Vec::new)
                .push(curr_creature.clone());

            filters_cache
                .filtered_by_family
                .entry(curr_creature.family.clone())
                .or_insert_with(Vec::new)
                .push(curr_creature.clone());

            filters_cache
                .filtered_by_alignment
                .entry(curr_creature.alignment.clone())
                .or_insert_with(Vec::new)
                .push(curr_creature.clone());

            filters_cache
                .filtered_by_size
                .entry(curr_creature.size.clone())
                .or_insert_with(Vec::new)
                .push(curr_creature.clone());

            filters_cache
                .filtered_by_rarity
                .entry(curr_creature.rarity.clone())
                .or_insert_with(Vec::new)
                .push(curr_creature.clone());

            filters_cache
                .filtered_by_melee
                .entry(curr_creature.is_melee)
                .or_insert_with(Vec::new)
                .push(curr_creature.clone());

            filters_cache
                .filtered_by_ranged
                .entry(curr_creature.is_ranged)
                .or_insert_with(Vec::new)
                .push(curr_creature.clone());

            filters_cache
                .filtered_by_spell_caster
                .entry(curr_creature.is_spell_caster)
                .or_insert_with(Vec::new)
                .push(curr_creature.clone());
        }
    }
    filters_cache
}

pub fn from_db_data_to_sorted_vectors(
    data: &Result<Vec<Creature>, RedisError>,
) -> SortedVectorCache {
    let mut sorted_cache = SortedVectorCache::default();
    if let Ok(unordered_creatures) = data {
        // NEEDS TO BE OPTIMIZED, I'M DYING LOOKING AT THIS
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
    }
    sorted_cache
}
