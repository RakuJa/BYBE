use crate::models::scales_struct::ability_scales::AbilityScales;
use crate::models::scales_struct::ac_scales::AcScales;
use crate::models::scales_struct::area_dmg_scales::AreaDmgScales;
use crate::models::scales_struct::hp_scales::HpScales;
use crate::models::scales_struct::item_scales::ItemScales;
use crate::models::scales_struct::perception_scales::PerceptionScales;
use crate::models::scales_struct::res_weak_scales::ResWeakScales;
use crate::models::scales_struct::saving_throw_scales::SavingThrowScales;
use crate::models::scales_struct::skill_scales::SkillScales;
use crate::models::scales_struct::spell_dc_and_atk_scales::SpellDcAndAtkScales;
use crate::models::scales_struct::strike_bonus_scales::StrikeBonusScales;
use crate::models::scales_struct::strike_dmg_scales::StrikeDmgScales;
use std::collections::HashMap;

#[derive(Default, Eq, PartialEq, Clone)]
pub struct CreatureScales {
    pub ability_scales: HashMap<i8, AbilityScales>,
    pub ac_scales: HashMap<i8, AcScales>,
    pub area_dmg_scales: HashMap<i8, AreaDmgScales>,
    pub hp_scales: HashMap<i8, HpScales>,
    pub item_scales: HashMap<String, ItemScales>,
    pub perception_scales: HashMap<i8, PerceptionScales>,
    pub res_weak_scales: HashMap<i8, ResWeakScales>,
    pub saving_throw_scales: HashMap<i8, SavingThrowScales>,
    pub skill_scales: HashMap<i8, SkillScales>,
    pub spell_dc_and_atk_scales: HashMap<i8, SpellDcAndAtkScales>,
    pub strike_bonus_scales: HashMap<i8, StrikeBonusScales>,
    pub strike_dmg_scales: HashMap<i8, StrikeDmgScales>,
}
