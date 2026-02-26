use crate::models::hazard::hazard_component::hazard_core::HazardEssentialData;
use crate::models::hazard::hazard_field_filter::{HazardComplexityEnum, HazardFieldFilters};
use crate::models::shared::action::Action;
use crate::models::shared::game_system_enum::GameSystem;
use crate::models::shared::pf_version_enum::GameSystemVersionEnum;
use crate::traits::has_level::HasLevel;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;
use sqlx::{Error, FromRow};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, Eq, Hash, PartialEq, Debug, ToSchema)]
pub struct Hazard {
    pub essential: HazardEssentialData,
    pub traits: Vec<String>,
    pub actions: Vec<Action>,
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
        }
    }
}

impl HasLevel for Hazard {
    fn level(&self) -> i64 {
        self.essential.level
    }
}

impl Hazard {
    pub fn is_passing_filters(&self, filters: &HazardFieldFilters) -> bool {
        self.check_creature_pass_equality_filters(filters)
            && self.check_creature_pass_ub_filters(filters)
            && self.check_creature_pass_lb_filters(filters)
            && self.check_creature_pass_string_filters(filters)
    }

    fn check_creature_pass_ub_filters(&self, filters: &HazardFieldFilters) -> bool {
        filters.max_ac_filter.is_none_or(|m| self.essential.ac <= m)
            && filters
                .max_hardness_filter
                .is_none_or(|m| self.essential.hardness <= m)
            && filters.max_hp_filter.is_none_or(|m| self.essential.hp <= m)
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
    }

    fn check_creature_pass_lb_filters(&self, filters: &HazardFieldFilters) -> bool {
        filters.max_ac_filter.is_none_or(|m| self.essential.ac >= m)
            && filters
                .max_hardness_filter
                .is_none_or(|m| self.essential.hardness >= m)
            && filters.max_hp_filter.is_none_or(|m| self.essential.hp >= m)
            && filters
                .max_level_filter
                .is_none_or(|m| self.essential.level >= m)
            && filters
                .max_fortitude_filter
                .is_none_or(|m| self.essential.fortitude.is_some_and(|x| x <= m))
            && filters
                .max_reflex_filter
                .is_none_or(|m| self.essential.reflex.is_some_and(|x| x <= m))
            && filters
                .max_will_filter
                .is_none_or(|m| self.essential.will.is_some_and(|x| x <= m))
    }

    fn check_creature_pass_equality_filters(&self, filters: &HazardFieldFilters) -> bool {
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
                HazardComplexityEnum::Simple => self.essential.kind == HazardComplexityEnum::Simple,
                HazardComplexityEnum::Complex => {
                    self.essential.kind == HazardComplexityEnum::Complex
                }
                HazardComplexityEnum::Any => true,
            }
            && match filters.game_system_version.unwrap_or_default() {
                GameSystemVersionEnum::Legacy => !self.essential.remaster,
                GameSystemVersionEnum::Remaster => self.essential.remaster,
                GameSystemVersionEnum::Any => true,
            }
    }

    fn check_creature_pass_string_filters(&self, filters: &HazardFieldFilters) -> bool {
        filters.name_filter.as_ref().is_none_or(|name| {
            self.essential
                .name
                .to_lowercase()
                .contains(name.to_lowercase().as_str())
        }) && filters.trait_whitelist_filter.as_ref().is_none_or(|x| {
            x.iter().any(|filter_trait| {
                self.traits.iter().any(|cr_trait| {
                    cr_trait
                        .to_lowercase()
                        .contains(filter_trait.to_lowercase().as_str())
                })
            })
        }) && !filters.trait_blacklist_filter.as_ref().is_some_and(|x| {
            x.iter().any(|filter_trait| {
                self.traits.iter().any(|cr_trait| {
                    cr_trait
                        .to_lowercase()
                        .eq(filter_trait.to_lowercase().as_str())
                })
            })
        })
    }
}

impl<'r> FromRow<'r, SqliteRow> for Hazard {
    fn from_row(row: &'r SqliteRow) -> Result<Self, Error> {
        Ok(Self {
            essential: HazardEssentialData::from_row(row)?,
            traits: vec![],
            actions: vec![],
            game_system: Default::default(),
        })
    }
}
