use std::collections::HashMap;
use crate::models::creature::Creature;

struct DbCache {
    unordered_creatures: Vec<Creature>,

    order_by_id_ascending: Vec<Creature>,
    order_by_id_descending: Vec<Creature>,

    order_by_name_ascending: Vec<Creature>,
    order_by_name_descending: Vec<Creature>,

    order_by_hp_ascending: Vec<Creature>,
    order_by_hp_descending: Vec<Creature>,

    order_by_level_ascending: Vec<Creature>,
    order_by_level_descending: Vec<Creature>,

    order_by_family_ascending: Vec<Creature>,
    order_by_family_descending: Vec<Creature>,

    order_by_alignment_ascending: Vec<Creature>,
    order_by_alignment_descending: Vec<Creature>,

    order_by_size_ascending: Vec<Creature>,
    order_by_size_descending: Vec<Creature>,

    order_by_rarity_ascending: Vec<Creature>,
    order_by_rarity_descending: Vec<Creature>,

    filtered_by_id: HashMap<String, Creature>,
    filtered_by_level: HashMap<String, Vec<Creature>>,
    filtered_by_family: HashMap<String, Vec<Creature>>,
    filtered_by_alignment: HashMap<String, Vec<Creature>>,
    filtered_by_size: HashMap<String, Vec<Creature>>,
    filtered_by_rarity: HashMap<String, Vec<Creature>>,
    filtered_by_melee: HashMap<String, Vec<Creature>>,
    filtered_by_ranged: HashMap<String, Vec<Creature>>,
    filtered_by_spell_caster: HashMap<String, Vec<Creature>>,


}