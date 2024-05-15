use crate::models::creature_component::creature_core::CreatureCoreData;
use crate::services::url_calculator::add_boolean_query;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum::Display;
use utoipa::ToSchema;

#[derive(
    Serialize, Deserialize, ToSchema, Display, Eq, Hash, PartialEq, Ord, PartialOrd, Default,
)]
pub enum CreatureVariant {
    Weak,
    Elite,
    #[default]
    Base,
}

impl Clone for CreatureVariant {
    fn clone(&self) -> CreatureVariant {
        match self {
            CreatureVariant::Elite => CreatureVariant::Elite,
            CreatureVariant::Weak => CreatureVariant::Weak,
            CreatureVariant::Base => CreatureVariant::Base,
        }
    }
}

impl CreatureVariant {
    pub fn to_level_delta(&self) -> i64 {
        match self {
            CreatureVariant::Weak => -1,
            CreatureVariant::Elite => 1,
            CreatureVariant::Base => 0,
        }
    }

    pub fn get_variant_hp_and_link(&self, core: &CreatureCoreData) -> (i64, Option<String>) {
        let mut core_hp = core.essential.hp;
        match self {
            CreatureVariant::Base => (core_hp, core.derived.archive_link.clone()),
            _ => {
                let level = core.essential.level;
                let level_delta = self.to_level_delta();
                let archive_link = core.derived.archive_link.clone();
                let variant_archive_link = match self {
                    CreatureVariant::Base => archive_link,
                    _ => add_boolean_query(&archive_link, &self.to_string(), true),
                };

                let hp_increase = hp_increase_by_level();
                let desired_key = hp_increase
                    .keys()
                    .filter(|lvl| level >= **lvl)
                    .max()
                    .unwrap_or(hp_increase.keys().next().unwrap_or(&0));
                core_hp += *hp_increase.get(desired_key).unwrap_or(&0) * level_delta;
                core_hp = core_hp.max(1);

                (core_hp, variant_archive_link)
            }
        }
    }
}

fn hp_increase_by_level() -> HashMap<i64, i64> {
    hashmap! { 1 => 10, 2=> 15, 5=> 20, 20=> 30 }
}
