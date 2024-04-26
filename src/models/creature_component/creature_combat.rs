use crate::models::db::raw_creature::RawCreature;
use crate::models::db::raw_immunity::RawImmunity;
use crate::models::db::raw_resistance::RawResistance;
use crate::models::db::raw_weakness::RawWeakness;
use crate::models::items::weapon::Weapon;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq)]
pub struct SavingThrows {
    pub fortitude: i8,
    pub reflex: i8,
    pub will: i8,
    pub fortitude_detail: Option<String>,
    pub reflex_detail: Option<String>,
    pub will_detail: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq)]
pub struct CreatureCombatData {
    pub weapons: Vec<Weapon>,
    pub resistances: BTreeMap<String, i16>,
    pub immunities: Vec<String>,
    pub weaknesses: BTreeMap<String, i16>,
    pub saving_throws: SavingThrows,
    pub ac: i8,
}

impl
    From<(
        RawCreature,
        Vec<Weapon>,
        Vec<RawImmunity>,
        Vec<RawResistance>,
        Vec<RawWeakness>,
    )> for CreatureCombatData
{
    fn from(
        tuple: (
            RawCreature,
            Vec<Weapon>,
            Vec<RawImmunity>,
            Vec<RawResistance>,
            Vec<RawWeakness>,
        ),
    ) -> Self {
        let raw_cr = tuple.0;
        Self {
            weapons: tuple.1,
            immunities: tuple
                .2
                .into_iter()
                .map(|curr_trait| curr_trait.name)
                .collect(),
            resistances: tuple
                .3
                .into_iter()
                .map(|curr_res| (curr_res.name, curr_res.value as i16))
                .collect(),
            weaknesses: tuple
                .4
                .into_iter()
                .map(|curr_weak| (curr_weak.name, curr_weak.value as i16))
                .collect(),
            ac: raw_cr.ac as i8,
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
