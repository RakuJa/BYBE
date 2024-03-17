use crate::models::creature_component::creature_combat::CreatureCombatData;
use crate::models::creature_component::creature_core::CreatureCoreData;
use crate::models::creature_component::creature_extra::CreatureExtraData;
use crate::models::creature_component::creature_info::CreatureInfo;
use crate::models::creature_component::creature_spell_caster::CreatureSpellCasterData;
use crate::models::creature_component::creature_variant::CreatureVariantData;
use crate::models::db::raw_creature::RawCreature;
use crate::models::db::raw_immunity::RawImmunity;
use crate::models::db::raw_language::RawLanguage;
use crate::models::db::raw_resistance::RawResistance;
use crate::models::db::raw_sense::RawSense;
use crate::models::db::raw_speed::RawSpeed;
use crate::models::db::raw_trait::RawTrait;
use crate::models::db::raw_weakness::RawWeakness;
use crate::models::items::action::Action;
use crate::models::items::skill::Skill;
use crate::models::items::spell::Spell;
use crate::models::items::weapon::Weapon;
use crate::models::routers_validator_structs::FieldFilters;
use crate::models::scales_struct::creature_scales::CreatureScales;
use crate::services::url_calculator::generate_archive_link;
use regex::Regex;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq)]
pub struct PublicationInfo {
    pub license: String,
    pub remaster: bool,
    pub source: String,
}

#[derive(Serialize, Deserialize, Clone, Eq, Hash, PartialEq)]
pub struct Creature {
    pub core_data: CreatureCoreData,
    pub variant_data: CreatureVariantData,
    pub extra_data: CreatureExtraData,
    pub combat_data: CreatureCombatData,
    pub spell_caster_data: CreatureSpellCasterData,
    pub info: CreatureInfo,
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

impl
    From<(
        RawCreature,
        Vec<RawTrait>,
        Vec<Weapon>,
        Vec<Spell>,
        Vec<Action>,
        Vec<Skill>,
        Vec<RawImmunity>,
        Vec<RawLanguage>,
        Vec<RawResistance>,
        Vec<RawSense>,
        Vec<RawSpeed>,
        Vec<RawWeakness>,
        &CreatureScales,
        &Regex,
    )> for Creature
{
    fn from(
        tuple: (
            RawCreature,
            Vec<RawTrait>,
            Vec<Weapon>,
            Vec<Spell>,
            Vec<Action>,
            Vec<Skill>,
            Vec<RawImmunity>,
            Vec<RawLanguage>,
            Vec<RawResistance>,
            Vec<RawSense>,
            Vec<RawSpeed>,
            Vec<RawWeakness>,
            &CreatureScales,
            &Regex,
        ),
    ) -> Self {
        let raw_creature = tuple.0;
        let traits = tuple.1;
        let weapons = tuple.2;
        let spells = tuple.3;
        let actions = tuple.4;
        let skills = tuple.5;
        let immunities = tuple.6;
        let languages = tuple.7;
        let resistances = tuple.8;
        let senses = tuple.9;
        let speeds = tuple.10;
        let weaknesses = tuple.11;

        let archive_link = generate_archive_link(raw_creature.aon_id, &raw_creature.cr_type);
        let is_ranged = is_ranged(&weapons);
        let is_melee = is_melee(&weapons);
        let core_data = CreatureCoreData::from((
            raw_creature.clone(),
            traits,
            is_ranged,
            is_melee,
            archive_link.clone(),
        ));
        let extra_data = CreatureExtraData::from((
            raw_creature.clone(),
            actions,
            skills,
            languages,
            senses,
            speeds,
        ));

        let combat_data = CreatureCombatData::from((
            raw_creature.clone(),
            weapons,
            immunities,
            resistances,
            weaknesses,
        ));
        let scales = tuple.12;
        let spell_caster_data = CreatureSpellCasterData::from((raw_creature.clone(), spells));
        let info = CreatureInfo::from((
            &core_data,
            &extra_data,
            &combat_data,
            &spell_caster_data,
            scales,
            tuple.13,
        ));

        Self {
            variant_data: CreatureVariantData::from((raw_creature.level, archive_link)),
            core_data,
            extra_data,
            spell_caster_data,
            combat_data,
            info,
        }
    }
}

fn is_melee(weapons: &[Weapon]) -> bool {
    weapons
        .iter()
        .any(|el| el.wp_type.to_uppercase() == "MELEE")
}

fn is_ranged(weapons: &[Weapon]) -> bool {
    weapons
        .iter()
        .any(|el| el.wp_type.to_uppercase() == "RANGED")
}
