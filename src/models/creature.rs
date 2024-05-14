use crate::models::creature_component::creature_combat::CreatureCombatData;
use crate::models::creature_component::creature_core::CreatureCoreData;
use crate::models::creature_component::creature_extra::CreatureExtraData;
use crate::models::creature_component::creature_spell_caster::CreatureSpellCasterData;
use crate::models::creature_component::creature_variant::CreatureVariantData;
use crate::models::creature_metadata::creature_role::CreatureRoleEnum;
use crate::models::creature_metadata::variant_enum::CreatureVariant;
use crate::models::routers_validator_structs::FieldFilters;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Eq, Hash, PartialEq)]
pub struct Creature {
    pub core_data: CreatureCoreData,
    pub variant_data: CreatureVariantData,
    pub extra_data: Option<CreatureExtraData>,
    pub combat_data: Option<CreatureCombatData>,
    pub spell_caster_data: Option<CreatureSpellCasterData>,
}

impl Creature {
    pub fn from_core(core: CreatureCoreData) -> Creature {
        let level = core.essential.level;
        let archive_link = core.derived.archive_link.clone();
        Self {
            core_data: core,
            variant_data: CreatureVariantData {
                variant: CreatureVariant::Base,
                level,
                archive_link,
            },
            extra_data: None,
            combat_data: None,
            spell_caster_data: None,
        }
    }
    pub fn is_passing_filters(&self, filters: &FieldFilters) -> bool {
        Creature::check_creature_pass_equality_filters(self, filters)
            && Creature::check_creature_pass_greater_filters(self, filters)
            && Creature::check_creature_pass_lesser_filters(self, filters)
            && Creature::check_creature_pass_string_filters(self, filters)
    }

    fn check_creature_pass_greater_filters(&self, filters: &FieldFilters) -> bool {
        let hp_pass = filters
            .max_hp_filter
            .map_or(true, |max_hp| self.core_data.essential.hp <= max_hp);

        let level_pass = filters
            .max_level_filter
            .map_or(true, |max_lvl| self.variant_data.level <= max_lvl);

        hp_pass && level_pass
    }

    fn check_creature_pass_lesser_filters(&self, filters: &FieldFilters) -> bool {
        let hp_pass = filters
            .min_hp_filter
            .map_or(true, |min_hp| self.core_data.essential.hp >= min_hp);

        let level_pass = filters
            .min_level_filter
            .map_or(true, |min_lvl| self.variant_data.level >= min_lvl);

        hp_pass && level_pass
    }

    fn check_creature_pass_equality_filters(&self, filters: &FieldFilters) -> bool {
        let rarity_pass = filters
            .rarity_filter
            .as_ref()
            .map_or(true, |rarity| self.core_data.essential.rarity == *rarity);
        let size_pass = filters
            .size_filter
            .as_ref()
            .map_or(true, |size| self.core_data.essential.size == *size);
        let alignment_pass = filters
            .alignment_filter
            .as_ref()
            .map_or(true, |alignment| self.core_data.alignment == *alignment);
        let is_melee_pass = filters
            .is_melee_filter
            .map_or(true, |is_melee| self.core_data.derived.is_melee == is_melee);
        let is_ranged_pass = filters.is_ranged_filter.map_or(true, |is_ranged| {
            self.core_data.derived.is_ranged == is_ranged
        });
        let is_spell_caster_pass = filters
            .is_spell_caster_filter
            .map_or(true, |is_spell_caster| {
                self.core_data.derived.is_spell_caster == is_spell_caster
            });
        let type_pass = filters
            .type_filter
            .as_ref()
            .map_or(true, |cr_type| self.core_data.essential.cr_type == *cr_type);

        let role_pass = filters.role_threshold.is_none()
            || filters.role_filter.as_ref().map_or(true, |cr_role| {
                let t = filters.role_threshold.unwrap_or(0);
                match cr_role {
                    CreatureRoleEnum::Brute => self.core_data.derived.brute_percentage >= t,
                    CreatureRoleEnum::MagicalStriker => {
                        self.core_data.derived.magical_striker_percentage >= t
                    }
                    CreatureRoleEnum::SkillParagon => {
                        self.core_data.derived.skill_paragon_percentage >= t
                    }
                    CreatureRoleEnum::Skirmisher => {
                        self.core_data.derived.skirmisher_percentage >= t
                    }
                    CreatureRoleEnum::Sniper => self.core_data.derived.sniper_percentage >= t,
                    CreatureRoleEnum::Soldier => self.core_data.derived.soldier_percentage >= t,
                    CreatureRoleEnum::SpellCaster => {
                        self.core_data.derived.spell_caster_percentage >= t
                    }
                }
            });

        rarity_pass
            && size_pass
            && alignment_pass
            && is_melee_pass
            && is_ranged_pass
            && is_spell_caster_pass
            && type_pass
            && role_pass
    }

    fn check_creature_pass_string_filters(&self, filters: &FieldFilters) -> bool {
        let name_pass: bool = filters.name_filter.as_ref().map_or(true, |name| {
            self.core_data.essential.name.to_lowercase().contains(name)
        });

        let family_pass: bool = filters.family_filter.as_ref().map_or(true, |name| {
            self.core_data
                .essential
                .family
                .to_lowercase()
                .contains(name)
        });
        name_pass && family_pass
    }
}
