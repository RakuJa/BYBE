use crate::models::creature::items::action::Action;
use crate::models::creature::items::skill::Skill;
use crate::models::item::item_struct::Item;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq)]
pub struct AbilityScores {
    pub charisma: i64,
    pub constitution: i64,
    pub dexterity: i64,
    pub intelligence: i64,
    pub strength: i64,
    pub wisdom: i64,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq)]
pub struct CreatureExtraData {
    pub actions: Vec<Action>,
    pub skills: Vec<Skill>,
    pub items: Vec<Item>,
    pub languages: Vec<String>,
    pub senses: Vec<String>,
    pub speeds: BTreeMap<String, i16>,
    pub ability_scores: AbilityScores,
    pub hp_detail: Option<String>,
    pub ac_detail: Option<String>,
    pub language_detail: Option<String>,
    pub perception: i8,
    pub perception_detail: Option<String>,
}

impl CreatureExtraData {
    fn add_mod_to_perception_and_skill_mods(self, modifier: i64) -> CreatureExtraData {
        let mut ex_data = self;
        // we should never have a pwl much greater than perception (pwl=lvl)
        ex_data.perception = (ex_data.perception as i64 + modifier) as i8;

        ex_data.skills = ex_data
            .skills
            .into_iter()
            .map(|mut skill| {
                skill.modifier += modifier;
                skill
            })
            .collect();

        ex_data
    }
    /// Lowers skill and perception by the given pwl_mod
    pub fn convert_from_base_to_pwl(self, pwl_mod: u64) -> CreatureExtraData {
        self.add_mod_to_perception_and_skill_mods(-(pwl_mod as i64))
    }
}
