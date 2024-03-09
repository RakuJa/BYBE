use crate::models::creature_metadata::alignment_enum::AlignmentEnum;
use crate::models::creature_metadata::rarity_enum::RarityEnum;
use crate::models::creature_metadata::size_enum::SizeEnum;
use crate::models::creature_metadata::type_enum::CreatureTypeEnum;
use crate::models::creature_metadata::variant_enum::CreatureVariant;
use crate::models::db::raw_creature::RawCreature;
use crate::models::db::raw_immunity::RawImmunity;
use crate::models::db::raw_language::RawLanguage;
use crate::models::db::raw_resistance::RawResistance;
use crate::models::db::raw_sense::RawSense;
use crate::models::db::raw_speed::RawSpeed;
use crate::models::db::raw_trait::RawTrait;
use crate::models::db::raw_weakness::RawWeakness;
use crate::models::items::spell::Spell;
use crate::models::items::weapon::Weapon;
use crate::models::routers_validator_structs::FieldFilters;
use crate::services::url_calculator::generate_archive_link;
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
    pub publication_info: PublicationInfo,
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
    pub immunities: Vec<String>,
    pub languages: Vec<String>,
    pub resistances: Vec<(String, i16)>,
    pub senses: Vec<String>,
    pub speeds: Vec<(String, i16)>,
    pub weaknesses: Vec<(String, i16)>,
    pub ability_scores: AbilityScores,
    pub hp_detail: Option<String>,
    pub ac_detail: Option<String>,
    pub language_detail: Option<String>,
    pub perception: i8,
    pub perception_detail: Option<String>,
    pub saving_throws: SavingThrows,
}

#[derive(Serialize, Deserialize, Clone, Eq, Hash, PartialEq)]
pub struct AbilityScores {
    pub charisma: i8,
    pub constitution: i8,
    pub dexterity: i8,
    pub intelligence: i8,
    pub strength: i8,
    pub wisdom: i8,
}

#[derive(Serialize, Deserialize, Clone, Eq, Hash, PartialEq)]
pub struct PublicationInfo {
    pub license: String,
    pub remaster: bool,
    pub source: String,
}

#[derive(Serialize, Deserialize, Clone, Eq, Hash, PartialEq)]
pub struct SavingThrows {
    fortitude: i8,
    reflex: i8,
    will: i8,
    fortitude_detail: Option<String>,
    reflex_detail: Option<String>,
    will_detail: Option<String>,
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

impl From<(RawCreature, Vec<RawTrait>, bool, bool, Option<String>)> for CoreCreatureData {
    fn from(tuple: (RawCreature, Vec<RawTrait>, bool, bool, Option<String>)) -> Self {
        let raw = tuple.0;
        let traits = tuple.1;
        let is_ranged = tuple.2;
        let is_melee = tuple.3;
        let archive_link = tuple.4;

        let alignment_enum = AlignmentEnum::from((&traits, raw.remaster));
        CoreCreatureData {
            id: raw.id as i32,
            aon_id: raw.aon_id.map(|x| x as i32),
            name: raw.name.clone(),
            hp: raw.hp as i16,
            base_level: raw.level as i8,
            alignment: alignment_enum,
            size: raw.size.clone(),
            family: raw.family.clone(),
            rarity: raw.rarity.clone(),
            is_spell_caster: raw.is_spell_caster,
            publication_info: PublicationInfo {
                remaster: raw.remaster,
                source: raw.source,
                license: raw.license,
            },
            traits: traits
                .into_iter()
                .map(|curr_trait| curr_trait.name)
                .collect(),
            creature_type: raw.cr_type.clone(),
            archive_link: archive_link.clone(),
            variant: CreatureVariant::Base,
            is_ranged,
            is_melee,
        }
    }
}

impl From<(i64, Option<String>)> for VariantCreatureData {
    fn from(value: (i64, Option<String>)) -> Self {
        Self {
            level: value.0 as i8,
            archive_link: value.1,
        }
    }
}

impl
    From<(
        RawCreature,
        Vec<Weapon>,
        Vec<Spell>,
        Vec<RawImmunity>,
        Vec<RawLanguage>,
        Vec<RawResistance>,
        Vec<RawSense>,
        Vec<RawSpeed>,
        Vec<RawWeakness>,
    )> for ExtraCreatureData
{
    fn from(
        tuple: (
            RawCreature,
            Vec<Weapon>,
            Vec<Spell>,
            Vec<RawImmunity>,
            Vec<RawLanguage>,
            Vec<RawResistance>,
            Vec<RawSense>,
            Vec<RawSpeed>,
            Vec<RawWeakness>,
        ),
    ) -> Self {
        let raw_cr = tuple.0;
        Self {
            weapons: tuple.1,
            spells: tuple.2,
            immunities: tuple
                .3
                .into_iter()
                .map(|curr_trait| curr_trait.name)
                .collect(),
            languages: tuple
                .4
                .into_iter()
                .map(|curr_trait| curr_trait.name)
                .collect(),
            resistances: tuple
                .5
                .into_iter()
                .map(|curr_res| (curr_res.name, curr_res.value as i16))
                .collect(),
            senses: tuple
                .6
                .into_iter()
                .map(|curr_trait| curr_trait.name)
                .collect(),
            speeds: tuple
                .7
                .into_iter()
                .map(|curr_speed| (curr_speed.name, curr_speed.value as i16))
                .collect(),
            weaknesses: tuple
                .8
                .into_iter()
                .map(|curr_weak| (curr_weak.name, curr_weak.value as i16))
                .collect(),
            ability_scores: AbilityScores {
                charisma: raw_cr.charisma as i8,
                constitution: raw_cr.constitution as i8,
                dexterity: raw_cr.dexterity as i8,
                intelligence: raw_cr.intelligence as i8,
                strength: raw_cr.strength as i8,
                wisdom: raw_cr.wisdom as i8,
            },
            hp_detail: if raw_cr.hp_detail.is_empty() {
                None
            } else {
                Some(raw_cr.hp_detail)
            },
            ac_detail: if raw_cr.ac_detail.is_empty() {
                None
            } else {
                Some(raw_cr.ac_detail)
            },
            language_detail: raw_cr.language_detail,
            perception: raw_cr.perception as i8,
            perception_detail: if raw_cr.perception_detail.is_empty() {
                None
            } else {
                Some(raw_cr.perception_detail)
            },
            saving_throws: SavingThrows {
                fortitude: raw_cr.fortitude as i8,
                reflex: raw_cr.reflex as i8,
                will: raw_cr.will as i8,
                fortitude_detail: if raw_cr.fortitude_detail.is_empty() {
                    None
                } else {
                    Some(raw_cr.fortitude_detail)
                },
                reflex_detail: if raw_cr.reflex_detail.is_empty() {
                    None
                } else {
                    Some(raw_cr.reflex_detail)
                },
                will_detail: if raw_cr.will_detail.is_empty() {
                    None
                } else {
                    Some(raw_cr.will_detail)
                },
            },
        }
    }
}

impl
    From<(
        RawCreature,
        Vec<RawTrait>,
        Vec<Weapon>,
        Vec<Spell>,
        Vec<RawImmunity>,
        Vec<RawLanguage>,
        Vec<RawResistance>,
        Vec<RawSense>,
        Vec<RawSpeed>,
        Vec<RawWeakness>,
    )> for Creature
{
    fn from(
        tuple: (
            RawCreature,
            Vec<RawTrait>,
            Vec<Weapon>,
            Vec<Spell>,
            Vec<RawImmunity>,
            Vec<RawLanguage>,
            Vec<RawResistance>,
            Vec<RawSense>,
            Vec<RawSpeed>,
            Vec<RawWeakness>,
        ),
    ) -> Self {
        let raw_creature = tuple.0;
        let weapons = tuple.2;
        let spells = tuple.3;
        let traits = tuple.1;
        let immunities = tuple.4;
        let languages = tuple.5;
        let resistances = tuple.6;
        let senses = tuple.7;
        let speeds = tuple.8;
        let weaknesses = tuple.9;

        let archive_link = generate_archive_link(raw_creature.aon_id, &raw_creature.cr_type);
        let is_ranged = CoreCreatureData::is_ranged(&weapons);
        let is_melee = CoreCreatureData::is_melee(&weapons);

        Self {
            variant_data: VariantCreatureData::from((raw_creature.level, archive_link.clone())),
            core_data: CoreCreatureData::from((
                raw_creature.clone(),
                traits,
                is_ranged,
                is_melee,
                archive_link,
            )),
            extra_data: ExtraCreatureData::from((
                raw_creature,
                weapons,
                spells,
                immunities,
                languages,
                resistances,
                senses,
                speeds,
                weaknesses,
            )),
        }
    }
}
