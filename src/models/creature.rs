use crate::models::creature_metadata::alignment_enum::AlignmentEnum;
use crate::models::creature_metadata::rarity_enum::RarityEnum;
use crate::models::creature_metadata::size_enum::SizeEnum;
use crate::models::creature_metadata::type_enum::CreatureTypeEnum;
use crate::models::creature_metadata::variant_enum::CreatureVariant;
use crate::models::items::spell::Spell;
use crate::models::items::weapon::Weapon;
use crate::models::routers_validator_structs::FieldFilters;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq)]
pub struct CoreCreatureData {
    pub id: i32,
    pub aon_id: Option<i32>,
    pub name: String,
    pub hp: i16,
    // constant value, it will never change
    pub base_level: i8,
    pub alignment: AlignmentEnum,
    pub size: SizeEnum,
    pub family: Option<String>,
    pub rarity: RarityEnum,
    pub is_spell_caster: bool,
    pub is_melee: bool,
    pub is_ranged: bool,
    pub source: String,
    pub traits: Vec<String>,
    pub archive_link: Option<String>,
    pub creature_type: CreatureTypeEnum,
    pub variant: CreatureVariant,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq)]
pub struct VariantCreatureData {
    pub level: i8,
    pub archive_link: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq)]
pub struct ExtraCreatureData {
    pub weapons: Vec<Weapon>,
    pub spells: Vec<Spell>,
}

#[derive(Serialize, Deserialize, Clone, Eq, Hash, PartialEq)]
pub struct Creature {
    pub core_data: CoreCreatureData,
    pub variant_data: VariantCreatureData,
    pub extra_data: ExtraCreatureData,
}

impl CoreCreatureData {
    pub fn is_melee(weapons: &[Weapon]) -> bool {
        weapons
            .iter()
            .any(|el| el.wp_type.to_uppercase() == "MELEE")
    }

    pub fn is_ranged(weapons: &[Weapon]) -> bool {
        weapons
            .iter()
            .any(|el| el.wp_type.to_uppercase() == "RANGED")
    }
}

impl Creature {
    pub fn is_passing_filters(&self, filters: &FieldFilters) -> bool {
        Creature::check_creature_pass_equality_filters(self, filters)
            && Creature::check_creature_pass_greater_filters(self, filters)
            && Creature::check_creature_pass_lesser_filters(self, filters)
            && Creature::check_creature_pass_string_filters(self, filters)
    }

    fn check_creature_pass_greater_filters(&self, filters: &FieldFilters) -> bool {
        let hp_pass = filters
            .max_hp_filter
            .map_or(true, |max_hp| self.core_data.hp <= max_hp);

        let level_pass = filters
            .max_level_filter
            .map_or(true, |max_lvl| self.variant_data.level <= max_lvl);

        hp_pass && level_pass
    }

    fn check_creature_pass_lesser_filters(&self, filters: &FieldFilters) -> bool {
        let hp_pass = filters
            .min_hp_filter
            .map_or(true, |min_hp| self.core_data.hp >= min_hp);

        let level_pass = filters
            .min_level_filter
            .map_or(true, |min_lvl| self.variant_data.level >= min_lvl);

        hp_pass && level_pass
    }

    fn check_creature_pass_equality_filters(&self, filters: &FieldFilters) -> bool {
        let rarity_pass = filters
            .rarity_filter
            .as_ref()
            .map_or(true, |rarity| self.core_data.rarity == *rarity);
        let size_pass = filters
            .size_filter
            .as_ref()
            .map_or(true, |size| self.core_data.size == *size);
        let alignment_pass = filters
            .alignment_filter
            .as_ref()
            .map_or(true, |alignment| self.core_data.alignment == *alignment);
        let is_melee_pass = filters
            .is_melee_filter
            .map_or(true, |is_melee| self.core_data.is_melee == is_melee);
        let is_ranged_pass = filters
            .is_ranged_filter
            .map_or(true, |is_ranged| self.core_data.is_ranged == is_ranged);
        let is_spell_caster_pass = filters
            .is_spell_caster_filter
            .map_or(true, |is_spell_caster| {
                self.core_data.is_spell_caster == is_spell_caster
            });

        rarity_pass
            && size_pass
            && alignment_pass
            && is_melee_pass
            && is_ranged_pass
            && is_spell_caster_pass
    }

    fn check_creature_pass_string_filters(&self, filters: &FieldFilters) -> bool {
        let name_pass: bool = filters.name_filter.as_ref().map_or(true, |name| {
            self.core_data.name.to_lowercase().contains(name)
        });

        let family_pass: bool = filters.family_filter.as_ref().map_or(true, |name| {
            self.core_data
                .family
                .as_ref()
                .unwrap_or(&"".to_string())
                .to_lowercase()
                .contains(name)
        });
        name_pass && family_pass
    }
}
