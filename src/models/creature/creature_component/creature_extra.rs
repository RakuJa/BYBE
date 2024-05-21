use crate::models::creature::items::action::Action;
use crate::models::creature::items::skill::Skill;
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
