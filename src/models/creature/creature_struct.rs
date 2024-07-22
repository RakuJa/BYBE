use crate::models::creature::creature_component::creature_combat::CreatureCombatData;
use crate::models::creature::creature_component::creature_core::CreatureCoreData;
use crate::models::creature::creature_component::creature_extra::CreatureExtraData;
use crate::models::creature::creature_component::creature_spell_caster::CreatureSpellCasterData;
use crate::models::creature::creature_component::creature_variant::CreatureVariantData;
use crate::models::creature::creature_metadata::creature_role::CreatureRoleEnum;
use crate::models::creature::creature_metadata::variant_enum::CreatureVariant;
use crate::models::pf_version_enum::PathfinderVersionEnum;
use crate::models::routers_validator_structs::CreatureFieldFilters;
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
    /// Decrease the creature’s level by 1; if the creature is level 1, instead decrease its level by 2.
    /// Decrease the creature’s HP based on its starting level.
    /// Decrease the creature’s AC, attack modifiers, DCs, saving throws, Perception, and skill modifiers by 2.
    /// Decrease the damage of its Strikes and other offensive abilities by 2. If the creature has limits on how many times or how often it can use an ability (such as a spellcaster’s spells or a dragon’s breath), decrease the damage by 4 instead.
    pub fn convert_creature_to_variant(self, variant: CreatureVariant) -> Creature {
        let mut cr = Self::from_core_with_variant(self.core_data, variant.clone());
        cr.extra_data = self
            .extra_data
            .map(|x| x.convert_from_base_to_variant(&variant));
        cr.combat_data = self
            .combat_data
            .map(|x| x.convert_from_base_to_variant(&variant));
        cr.spell_caster_data = self
            .spell_caster_data
            .map(|x| x.convert_from_base_to_variant(&variant));
        cr
    }

    pub fn convert_creature_to_pwl(self) -> Creature {
        let pwl_mod = if self.core_data.essential.level >= 0 {
            self.core_data.essential.level as u64
        } else {
            0
        };

        Creature {
            core_data: self.core_data,
            variant_data: self.variant_data,
            extra_data: self.extra_data.map(|x| x.convert_from_base_to_pwl(pwl_mod)),
            combat_data: self
                .combat_data
                .map(|x| x.convert_from_base_to_pwl(pwl_mod)),
            spell_caster_data: self
                .spell_caster_data
                .map(|x| x.convert_from_base_to_pwl(pwl_mod)),
        }
    }
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

    pub fn from_core_with_variant(
        mut core: CreatureCoreData,
        creature_variant: CreatureVariant,
    ) -> Creature {
        let variant_hp = creature_variant.get_variant_hp(core.essential.hp, core.essential.level);
        let variant_archive_link =
            creature_variant.get_variant_archive_link(core.derived.archive_link.clone());
        let variant_level = creature_variant.get_variant_level(core.essential.level);
        core.essential.hp = variant_hp;
        Self {
            core_data: core,
            variant_data: CreatureVariantData {
                variant: creature_variant,
                level: variant_level,
                archive_link: variant_archive_link,
            },
            extra_data: None,
            combat_data: None,
            spell_caster_data: None,
        }
    }
    pub fn is_passing_filters(&self, filters: &CreatureFieldFilters) -> bool {
        self.check_creature_pass_equality_filters(filters)
            && self.check_creature_pass_ub_filters(filters)
            && self.check_creature_pass_lb_filters(filters)
            && self.check_creature_pass_string_filters(filters)
    }

    fn check_creature_pass_ub_filters(&self, filters: &CreatureFieldFilters) -> bool {
        filters
            .max_hp_filter
            .map_or(true, |max_hp| self.core_data.essential.hp <= max_hp)
            && filters
                .max_level_filter
                .map_or(true, |max_lvl| self.variant_data.level <= max_lvl)
    }

    fn check_creature_pass_lb_filters(&self, filters: &CreatureFieldFilters) -> bool {
        filters
            .min_hp_filter
            .map_or(true, |min_hp| self.core_data.essential.hp >= min_hp)
            && filters
                .min_level_filter
                .map_or(true, |min_lvl| self.variant_data.level >= min_lvl)
    }

    fn check_creature_pass_equality_filters(&self, filters: &CreatureFieldFilters) -> bool {
        filters
            .rarity_filter
            .as_ref()
            .map_or(true, |rarity| self.core_data.essential.rarity == *rarity)
            && filters
                .size_filter
                .as_ref()
                .map_or(true, |size| self.core_data.essential.size == *size)
            && filters.alignment_filter.as_ref().map_or(true, |alignment| {
                self.core_data.essential.alignment == *alignment
            })
            && filters
                .is_melee_filter
                .map_or(true, |is_melee| self.core_data.derived.is_melee == is_melee)
            && filters.is_ranged_filter.map_or(true, |is_ranged| {
                self.core_data.derived.is_ranged == is_ranged
            })
            && filters
                .is_spell_caster_filter
                .map_or(true, |is_spell_caster| {
                    self.core_data.derived.is_spell_caster == is_spell_caster
                })
            && filters
                .type_filter
                .as_ref()
                .map_or(true, |cr_type| self.core_data.essential.cr_type == *cr_type)
            && (filters.role_threshold.is_none()
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
                }))
            && match filters.pathfinder_version.clone().unwrap_or_default() {
                PathfinderVersionEnum::Legacy => !self.core_data.essential.remaster,
                PathfinderVersionEnum::Remaster => self.core_data.essential.remaster,
                PathfinderVersionEnum::Any => true,
            }
    }

    fn check_creature_pass_string_filters(&self, filters: &CreatureFieldFilters) -> bool {
        filters.name_filter.as_ref().map_or(true, |name| {
            self.core_data.essential.name.to_lowercase().contains(name)
        }) && filters.family_filter.as_ref().map_or(true, |name| {
            self.core_data
                .essential
                .family
                .to_lowercase()
                .contains(name)
        })
    }
}
