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
    pub fn to_adjustment_modifier(&self) -> i64 {
        match self {
            CreatureVariant::Weak => -2,
            CreatureVariant::Elite => 2,
            CreatureVariant::Base => 0,
        }
    }

    pub fn get_variant_level(&self, base_lvl: i64) -> i64 {
        match self {
            //Decrease the creature’s level by 1; if the creature is level 1,
            // instead decrease its level by 2.
            CreatureVariant::Weak => {
                if base_lvl == 1 {
                    base_lvl - 2
                } else {
                    base_lvl - 1
                }
            }
            //Increase the creature’s level by 1; if the creature is level –1 or 0,
            // instead increase its level by 2.
            CreatureVariant::Elite => {
                if base_lvl == -1 || base_lvl == 0 {
                    base_lvl + 2
                } else {
                    base_lvl - 1
                }
            }
            CreatureVariant::Base => base_lvl,
        }
    }

    pub fn get_variant_hp(&self, base_hp: i64, starting_lvl: i64) -> i64 {
        let hp_mod_map = match self {
            CreatureVariant::Weak => hp_decrease_by_level(),
            CreatureVariant::Elite => hp_increase_by_level(),
            CreatureVariant::Base => {
                hashmap! {}
            }
        };
        // get the lowest possible key,
        // it must still be higher than the given starting level
        // ex: {1=>2, 3=>4} w start_lvl = 2 => 3
        let desired_key = hp_mod_map
            .keys()
            .filter(|lvl| starting_lvl >= **lvl)
            .max()
            .unwrap_or(hp_mod_map.keys().next().unwrap_or(&0));
        let hp_mod = *hp_mod_map.get(desired_key).unwrap_or(&0);
        (base_hp + hp_mod).max(1)
    }

    pub fn get_variant_archive_link(&self, archive_link: Option<String>) -> Option<String> {
        match self {
            CreatureVariant::Base => archive_link,
            _ => add_boolean_query(&archive_link, &self.to_string(), true),
        }
    }
}

fn hp_increase_by_level() -> HashMap<i64, i64> {
    hashmap! { 1 => 10, 2=> 15, 5=> 20, 20=> 30 }
}

fn hp_decrease_by_level() -> HashMap<i64, i64> {
    hashmap! {
        1 => -10,
        3 => -15,
        6 => -20,
        21 => -30
    }
}
