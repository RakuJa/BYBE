use crate::models::db::resistance::Resistance;
use crate::models::db::weakness::Weakness;
use crate::models::hazard::hazard_component::hazard_core::HazardEssentialData;
use crate::models::hazard::hazard_field_filter::{HazardComplexityEnum, HazardFieldFilters};
use crate::models::item::weapon_struct::Weapon;
use crate::models::shared::action::Action;
use crate::models::shared::game_system_enum::GameSystem;
use crate::models::shared::pf_version_enum::GameSystemVersionEnum;
use crate::models::shared::trait_data::TraitData;
use crate::traits::filterable::Filterable;
use crate::traits::has_complexity::HasComplexity;
use crate::traits::has_level::HasLevel;
use crate::traits::traits_enrichable::TraitsEnrichable;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::{Error, FromRow};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Eq, PartialEq, Hash, Clone, Copy)]
pub struct HazardRanges {
    pub min_ac: i64,
    pub max_ac: i64,
    pub min_hardness: i64,
    pub max_hardness: i64,
    pub min_hp: i64,
    pub max_hp: i64,
    pub min_stealth: i64,
    pub max_stealth: i64,
    pub min_level: i64,
    pub max_level: i64,

    pub min_will: i64,
    pub max_will: i64,
    pub min_reflex: i64,
    pub max_reflex: i64,
    pub min_fortitude: i64,
    pub max_fortitude: i64,
}

impl Default for HazardRanges {
    fn default() -> Self {
        Self {
            min_ac: i64::MAX,
            max_ac: i64::MIN,
            min_hardness: i64::MAX,
            max_hardness: i64::MIN,
            min_hp: i64::MAX,
            max_hp: i64::MIN,
            min_stealth: i64::MAX,
            max_stealth: i64::MIN,
            min_level: i64::MAX,
            max_level: i64::MIN,
            min_will: i64::MAX,
            max_will: i64::MIN,
            min_reflex: i64::MAX,
            max_reflex: i64::MIN,
            min_fortitude: i64::MAX,
            max_fortitude: i64::MIN,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Eq, Hash, PartialEq, Debug, ToSchema)]
pub struct Hazard {
    pub essential: HazardEssentialData,
    pub traits: Vec<TraitData>,
    pub actions: Vec<Action>,
    pub immunities: Vec<String>,
    pub resistances: Vec<Resistance>,
    pub weaknesses: Vec<Weakness>,
    pub weapons: Vec<Weapon>,
    pub game_system: GameSystem,
}

impl From<(Self, GameSystem)> for Hazard {
    fn from(value: (Self, GameSystem)) -> Self {
        let hazard = value.0;
        Self {
            essential: hazard.essential,
            traits: hazard.traits,
            game_system: value.1,
            actions: hazard.actions,
            immunities: hazard.immunities,
            resistances: hazard.resistances,
            weaknesses: hazard.weaknesses,
            weapons: hazard.weapons,
        }
    }
}

impl HasLevel for Hazard {
    fn level(&self) -> i64 {
        self.essential.level
    }
}
impl HasComplexity for Hazard {
    fn complexity(&self) -> HazardComplexityEnum {
        self.essential.complexity
    }
}
impl Filterable for Hazard {
    type FilterImpl = HazardFieldFilters;
    fn does_it_pass_ub_filters(&self, filters: &Self::FilterImpl) -> bool {
        filters
            .max_ac_filter
            .is_none_or(|m| self.essential.ac.is_none_or(|x| x <= m))
            && filters
                .max_hardness_filter
                .is_none_or(|m| self.essential.hardness <= m)
            && filters
                .max_hp_filter
                .is_none_or(|m| self.essential.hp.is_none_or(|x| x <= m))
            && filters
                .max_level_filter
                .is_none_or(|m| self.essential.level <= m)
            && filters
                .max_fortitude_filter
                .is_none_or(|m| self.essential.fortitude.is_none_or(|x| x <= m))
            && filters
                .max_reflex_filter
                .is_none_or(|m| self.essential.reflex.is_none_or(|x| x <= m))
            && filters
                .max_will_filter
                .is_none_or(|m| self.essential.will.is_none_or(|x| x <= m))
            && filters
                .max_stealth_filter
                .is_none_or(|m| self.essential.stealth <= m)
    }

    fn does_it_pass_lb_filters(&self, filters: &Self::FilterImpl) -> bool {
        filters
            .min_ac_filter
            .is_none_or(|m| self.essential.ac.is_none_or(|x| x >= m))
            && filters
                .min_hardness_filter
                .is_none_or(|m| self.essential.hardness >= m)
            && filters
                .min_hp_filter
                .is_none_or(|m| self.essential.hp.is_none_or(|x| x >= m))
            && filters
                .min_level_filter
                .is_none_or(|m| self.essential.level >= m)
            && filters
                .min_fortitude_filter
                .is_none_or(|m| self.essential.fortitude.is_none_or(|x| x >= m))
            && filters
                .min_reflex_filter
                .is_none_or(|m| self.essential.reflex.is_none_or(|x| x >= m))
            && filters
                .min_will_filter
                .is_none_or(|m| self.essential.will.is_none_or(|x| x >= m))
            && filters
                .min_stealth_filter
                .is_none_or(|m| self.essential.stealth >= m)
    }

    fn does_it_pass_string_filters(&self, filters: &Self::FilterImpl) -> bool {
        filters.name_filter.as_ref().is_none_or(|name| {
            self.essential
                .name
                .to_lowercase()
                .contains(name.to_lowercase().as_str())
        }) && filters.trait_whitelist_filter.as_ref().is_none_or(|x| {
            x.iter().any(|filter_trait| {
                self.traits.iter().any(|cr_trait| {
                    cr_trait
                        .name
                        .to_lowercase()
                        .contains(filter_trait.to_lowercase().as_str())
                })
            })
        }) && !filters.trait_blacklist_filter.as_ref().is_some_and(|x| {
            x.iter().any(|filter_trait| {
                self.traits.iter().any(|cr_trait| {
                    cr_trait
                        .name
                        .to_lowercase()
                        .eq(filter_trait.to_lowercase().as_str())
                })
            })
        })
    }

    fn does_it_pass_equality_filters(&self, filters: &Self::FilterImpl) -> bool {
        filters
            .rarity_filter
            .as_ref()
            .is_none_or(|x| x.contains(&self.essential.rarity))
            && filters
                .source_filter
                .as_ref()
                .is_none_or(|x| x.contains(&self.essential.source))
            && filters
                .size_filter
                .as_ref()
                .is_none_or(|x| x.contains(&self.essential.size))
            && match filters.complexity_filter.unwrap_or_default() {
                HazardComplexityEnum::Simple => {
                    self.essential.complexity == HazardComplexityEnum::Simple
                }
                HazardComplexityEnum::Complex => {
                    self.essential.complexity == HazardComplexityEnum::Complex
                }
                HazardComplexityEnum::Any => true,
            }
            && match filters.game_system_version.unwrap_or_default() {
                GameSystemVersionEnum::Legacy => !self.essential.remaster,
                GameSystemVersionEnum::Remaster => self.essential.remaster,
                GameSystemVersionEnum::Any => true,
            }
    }
}

impl<'r> FromRow<'r, PgRow> for Hazard {
    fn from_row(row: &'r PgRow) -> Result<Self, Error> {
        Ok(Self {
            essential: HazardEssentialData::from_row(row)?,
            traits: vec![],
            actions: vec![],
            immunities: vec![],
            resistances: vec![],
            weaknesses: Default::default(),
            weapons: vec![],
            game_system: Default::default(),
        })
    }
}

impl TraitsEnrichable for Hazard {
    fn entity_id(&self) -> i64 {
        self.essential.id
    }
    fn set_traits(&mut self, traits: Vec<TraitData>) {
        self.traits = traits;
    }
    fn entity_name() -> &'static str {
        "hazard"
    }
}
