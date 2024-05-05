use crate::models::db::raw_creature::RawCreature;
use crate::models::db::raw_language::RawLanguage;
use crate::models::db::raw_sense::RawSense;
use crate::models::db::raw_speed::RawSpeed;
use crate::models::items::action::Action;
use crate::models::items::skill::Skill;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq)]
pub struct AbilityScores {
    pub charisma: i8,
    pub constitution: i8,
    pub dexterity: i8,
    pub intelligence: i8,
    pub strength: i8,
    pub wisdom: i8,
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

impl
    From<(
        RawCreature,
        Vec<Action>,
        Vec<Skill>,
        Vec<RawLanguage>,
        Vec<RawSense>,
        Vec<RawSpeed>,
    )> for CreatureExtraData
{
    fn from(
        tuple: (
            RawCreature,
            Vec<Action>,
            Vec<Skill>,
            Vec<RawLanguage>,
            Vec<RawSense>,
            Vec<RawSpeed>,
        ),
    ) -> Self {
        let raw_cr = tuple.0;
        Self {
            actions: tuple.1,
            skills: tuple.2,
            languages: tuple
                .3
                .into_iter()
                .map(|curr_trait| curr_trait.name)
                .collect(),
            senses: tuple
                .4
                .into_iter()
                .map(|curr_trait| curr_trait.name)
                .collect(),
            speeds: tuple
                .5
                .into_iter()
                .map(|curr_speed| (curr_speed.name, curr_speed.value as i16))
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
        }
    }
}
