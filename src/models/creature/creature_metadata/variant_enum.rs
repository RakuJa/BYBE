use crate::services::url_calculator::add_boolean_query;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum::Display;
use utoipa::ToSchema;

#[derive(
    Copy,
    Clone,
    Serialize,
    Deserialize,
    ToSchema,
    Display,
    Eq,
    Hash,
    PartialEq,
    Ord,
    PartialOrd,
    Default,
    Debug,
)]
pub enum CreatureVariant {
    Weak,
    Elite,
    #[default]
    Base,
}

impl CreatureVariant {
    const fn to_adjustment_modifier(self) -> i64 {
        match self {
            Self::Weak => -2,
            Self::Elite => 2,
            Self::Base => 0,
        }
    }

    pub const fn to_dmg_adjustment_modifier(self, has_limited_usage: bool) -> i64 {
        self.to_adjustment_modifier() * if has_limited_usage { 2 } else { 1 }
    }

    pub const fn get_variant_level(self, base_lvl: i64) -> i64 {
        match self {
            //Decrease the creature’s level by 1; if the creature is level 1,
            // instead decrease its level by 2.
            Self::Weak => {
                if base_lvl == 1 {
                    base_lvl - 2
                } else {
                    base_lvl - 1
                }
            }
            //Increase the creature’s level by 1; if the creature is level –1 or 0,
            // instead increase its level by 2.
            Self::Elite => {
                if base_lvl == -1 || base_lvl == 0 {
                    base_lvl + 2
                } else {
                    base_lvl + 1
                }
            }
            Self::Base => base_lvl,
        }
    }

    pub fn get_variant_hp(self, base_hp: i64, starting_lvl: i64) -> i64 {
        let hp_mod_map = match self {
            Self::Weak => hp_decrease_by_level(),
            Self::Elite => hp_increase_by_level(),
            Self::Base => {
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
            .unwrap_or_else(|| hp_mod_map.keys().next().unwrap_or(&0));
        let hp_mod = *hp_mod_map.get(desired_key).unwrap_or(&0);
        (base_hp + hp_mod).max(1)
    }

    pub fn get_variant_archive_link(self, archive_link: Option<String>) -> Option<String> {
        match self {
            Self::Base => archive_link,
            _ => add_boolean_query(Option::from(&archive_link), &self.to_string(), true),
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

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(CreatureVariant::Weak, -2)]
    #[case(CreatureVariant::Base, 0)]
    #[case(CreatureVariant::Elite, 2)]
    fn to_adjustment_modifier_is_always_two(
        #[case] variant: CreatureVariant,
        #[case] expected: i64,
    ) {
        assert_eq!(expected, variant.to_adjustment_modifier());
    }

    #[rstest]
    #[case(CreatureVariant::Weak, -4)]
    #[case(CreatureVariant::Base, 0)]
    #[case(CreatureVariant::Elite, 4)]
    fn to_limited_dmg_adjustment_modifier_is_always_four(
        #[case] variant: CreatureVariant,
        #[case] expected: i64,
    ) {
        assert_eq!(expected, variant.to_dmg_adjustment_modifier(true));
    }

    #[rstest]
    #[case(CreatureVariant::Elite, true, 4)]
    #[case(CreatureVariant::Elite, false, 2)]
    #[case(CreatureVariant::Weak, true, -4)]
    #[case(CreatureVariant::Weak, false, -2)]
    #[case(CreatureVariant::Base, true, 0)]
    #[case(CreatureVariant::Base, false, 0)]
    fn to_dmg_adjustment_modifier_picks_based_on_limited_use(
        #[case] variant: CreatureVariant,
        #[case] is_limited_use: bool,
        #[case] expected: i64,
    ) {
        assert_eq!(expected, variant.to_dmg_adjustment_modifier(is_limited_use));
    }
}
