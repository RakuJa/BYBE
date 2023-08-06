use crate::models::creature_metadata_enums::{AlignmentEnum, RarityEnum, SizeEnum};
use crate::models::routers_validator_structs::FieldFilters;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Creature {
    pub id: i32,
    pub name: String,
    pub hp: i16,
    pub level: i8,
    pub alignment: AlignmentEnum,
    pub size: SizeEnum,
    pub family: Option<String>,
    pub rarity: RarityEnum,
    pub is_melee: bool,
    pub is_ranged: bool,
    pub is_spell_caster: bool,
    pub source: Vec<String>,
}

pub fn check_creature_pass_filters(creature: &Creature, filters: &FieldFilters) -> bool {
    check_creature_pass_equality_filters(creature, filters)
        && check_creature_pass_greater_filters(creature, filters)
        && check_creature_pass_lesser_filters(creature, filters)
        && check_creature_pass_string_filters(creature, filters)
}

fn check_creature_pass_greater_filters(creature: &Creature, filters: &FieldFilters) -> bool {
    let hp_pass = filters
        .max_hp_filter
        .map_or(true, |max_hp| creature.hp <= max_hp);

    let level_pass = filters
        .max_level_filter
        .map_or(true, |max_lvl| creature.level <= max_lvl);

    hp_pass && level_pass
}

fn check_creature_pass_lesser_filters(creature: &Creature, filters: &FieldFilters) -> bool {
    let hp_pass = filters
        .min_hp_filter
        .map_or(true, |min_hp| creature.hp >= min_hp);

    let level_pass = filters
        .min_level_filter
        .map_or(true, |min_lvl| creature.level >= min_lvl);

    hp_pass && level_pass
}

fn check_creature_pass_equality_filters(creature: &Creature, filters: &FieldFilters) -> bool {
    let rarity_pass = filters
        .rarity_filter
        .as_ref()
        .map_or(true, |rarity| creature.rarity == *rarity);
    let size_pass = filters
        .size_filter
        .as_ref()
        .map_or(true, |size| creature.size == *size);
    let alignment_pass = filters
        .alignment_filter
        .as_ref()
        .map_or(true, |alignment| creature.alignment == *alignment);
    let is_melee_pass = filters
        .is_melee_filter
        .map_or(true, |is_melee| creature.is_melee == is_melee);
    let is_ranged_pass = filters
        .is_ranged_filter
        .map_or(true, |is_ranged| creature.is_ranged == is_ranged);
    let is_spell_caster_pass = filters
        .is_spell_caster_filter
        .map_or(true, |is_spell_caster| {
            creature.is_spell_caster == is_spell_caster
        });

    rarity_pass
        && size_pass
        && alignment_pass
        && is_melee_pass
        && is_ranged_pass
        && is_spell_caster_pass
}

fn check_creature_pass_string_filters(creature: &Creature, filters: &FieldFilters) -> bool {
    let name_pass: bool = filters
        .name_filter
        .as_ref()
        .map_or(true, |name| creature.name.to_lowercase().contains(name));

    let family_pass: bool = filters.family_filter.as_ref().map_or(true, |name| {
        creature
            .family
            .as_ref()
            .unwrap_or(&"".to_string())
            .to_lowercase()
            .contains(name)
    });
    name_pass && family_pass
}
