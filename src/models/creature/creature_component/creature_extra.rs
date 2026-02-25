use crate::models::creature::creature_metadata::variant_enum::CreatureVariant;
use crate::models::creature::items::skill::Skill;
use crate::models::db::sense::Sense;
use crate::models::item::item_struct::Item;
use crate::models::shared::action::Action;
use serde::{Deserialize, Serialize};
#[allow(unused_imports)] // it's actually used in the example schema
use serde_json::json;
use std::collections::BTreeMap;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq, Debug)]
pub struct AbilityScores {
    #[schema(example = 0)]
    pub charisma: i64,
    #[schema(example = 0)]
    pub constitution: i64,
    #[schema(example = 0)]
    pub dexterity: i64,
    #[schema(example = 0)]
    pub intelligence: i64,
    #[schema(example = 0)]
    pub strength: i64,
    #[schema(example = 0)]
    pub wisdom: i64,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq, Debug)]
pub struct CreatureExtraData {
    pub actions: Vec<Action>,
    pub skills: Vec<Skill>,
    pub items: Vec<Item>,
    pub languages: Vec<String>,
    pub senses: Vec<Sense>,
    #[schema(example = json!({"fly": 100, "swim": 50, "Base": 25}))]
    pub speeds: BTreeMap<String, i16>,
    pub ability_scores: AbilityScores,
    pub hp_detail: Option<String>,
    pub ac_detail: Option<String>,
    pub language_detail: Option<String>,
    #[schema(example = 0)]
    pub perception: i8,
    pub perception_detail: Option<String>,
    pub has_vision: bool,
}

impl CreatureExtraData {
    fn add_mod_to_perception_and_skill_mods(self, modifier: i64) -> Self {
        let mut ex_data = self;
        // we should never have a pwl much greater than perception (pwl=lvl)
        ex_data.perception = (i64::from(ex_data.perception) + modifier) as i8;

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
    /// Lowers skill and perception by the given `pwl_mod`
    pub fn convert_from_base_to_pwl(self, pwl_mod: u64) -> Self {
        self.add_mod_to_perception_and_skill_mods(-i64::try_from(pwl_mod).unwrap_or(i64::MAX))
    }

    /// Increase/Decrease Perception, and skill modifiers by 2.
    pub fn convert_from_base_to_variant(self, variant: CreatureVariant) -> Self {
        self.add_mod_to_perception_and_skill_mods(variant.to_adjustment_modifier())
    }
}
