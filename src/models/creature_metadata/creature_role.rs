use crate::models::creature_component::creature_combat::CreatureCombatData;
use crate::models::creature_component::creature_core::CreatureCoreData;
use crate::models::creature_component::creature_extra::CreatureExtraData;
use crate::models::creature_component::creature_spell_caster::CreatureSpellCasterData;
use crate::models::scales_struct::creature_scales::CreatureScales;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt;
use utoipa::ToSchema;
use validator::HasLen;

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq)]
pub enum CreatureRole {
    Brute,
    MagicalStriker,
    SkillParagon,
    Skirmisher,
    Sniper,
    Soldier,
    SpellCaster,
}

impl CreatureRole {
    pub fn from_creature_with_given_scales(
        cr_core: &CreatureCoreData,
        cr_extra: &CreatureExtraData,
        cr_combat: &CreatureCombatData,
        cr_spells: &CreatureSpellCasterData,
        scales: &CreatureScales,
        dmg_scales_regex: &Regex,
    ) -> Vec<CreatureRole> {
        let mut roles = Vec::new();
        if is_brute(cr_core, cr_extra, cr_combat, scales, dmg_scales_regex).is_some_and(|x| x) {
            roles.push(Self::Brute);
        }
        if is_magical_striker(cr_core, cr_spells, cr_combat, scales, dmg_scales_regex)
            .is_some_and(|x| x)
        {
            roles.push(Self::MagicalStriker)
        }
        if is_skill_paragon(cr_core, cr_extra, cr_combat, scales).is_some_and(|x| x) {
            roles.push(Self::SkillParagon)
        }
        if is_skirmisher(cr_core, cr_extra, cr_combat, scales).is_some_and(|x| x) {
            roles.push(Self::Skirmisher);
        }
        if is_sniper(cr_core, cr_extra, cr_combat, scales, dmg_scales_regex).is_some_and(|x| x) {
            roles.push(Self::Sniper)
        }
        if is_soldier(cr_core, cr_extra, cr_combat, scales, dmg_scales_regex).is_some_and(|x| x) {
            roles.push(Self::Soldier);
        }
        if is_spellcaster(
            cr_core,
            cr_spells,
            cr_combat,
            cr_extra,
            scales,
            dmg_scales_regex,
        )
        .is_some_and(|x| x)
        {
            roles.push(Self::SpellCaster)
        }

        roles
    }
}
// Brute
fn is_brute(
    cr_core: &CreatureCoreData,
    cr_extra: &CreatureExtraData,
    cr_combat: &CreatureCombatData,
    scales: &CreatureScales,
    re: &Regex,
) -> Option<bool> {
    let lvl = cr_core.base_level;
    let per_scales = scales.perception_scales.get(&lvl)?;
    // low Perception;
    if !(per_scales.low..per_scales.moderate).contains(&(cr_extra.perception as i64)) {
        return Some(false);
    }
    let ability_scales = scales.ability_scales.get(&lvl)?;
    // high or extreme Str modifier,
    if cr_extra.ability_scores.strength < ability_scales.high as i8 {
        return Some(false);
    }
    // high to moderate Con modifier, TODO check with the shortcut of extreme
    if !(ability_scales.moderate..ability_scales.extreme?)
        .contains(&(cr_extra.ability_scores.constitution as i64))
    {
        return Some(false);
    }
    // low or lower mental modifiers;
    if cr_extra.ability_scores.intelligence >= ability_scales.moderate as i8
        && cr_extra.ability_scores.wisdom >= ability_scales.moderate as i8
        && cr_extra.ability_scores.charisma >= ability_scales.moderate as i8
    {
        return Some(false);
    }
    let saving_scales = scales.saving_throw_scales.get(&lvl)?;
    // Low or lower Reflex
    if cr_combat.saving_throws.reflex >= saving_scales.moderate as i8 {
        return Some(false);
    }
    // high Fortitude,
    if !(saving_scales.high..saving_scales.extreme)
        .contains(&(cr_combat.saving_throws.fortitude as i64))
    {
        return Some(false);
    }
    // Low will,
    if !(saving_scales.low..saving_scales.moderate).contains(&(cr_combat.saving_throws.will as i64))
    {
        return Some(false);
    }
    let ac_scales = scales.ac_scales.get(&lvl)?;
    // moderate or low AC;
    if cr_combat.ac >= ac_scales.high as i8 {
        return Some(false);
    }
    // high HP;
    let hp_scales = scales.hp_scales.get(&lvl)?;
    if cr_core.hp < hp_scales.high_lb as i16 || cr_core.hp > hp_scales.high_ub as i16 {
        return Some(false);
    }
    let atk_bonus_scales = scales.strike_bonus_scales.get(&lvl)?;
    let dmg_scales = scales.strike_dmg_scales.get(&lvl)?;

    let scales_extreme_avg = re
        .captures(dmg_scales.extreme.as_str())?
        .get(1)?
        .as_str()
        .parse::<i64>()
        .ok()?;
    let scales_high_avg = re
        .captures(dmg_scales.high.as_str())?
        .get(1)?
        .as_str()
        .parse::<i64>()
        .ok()?;
    // high attack bonus and high damage OR moderate attack bonus and extreme damage
    if cr_combat.weapons.iter().any(|curr_wp| {
        !((atk_bonus_scales.high..atk_bonus_scales.extreme).contains(&curr_wp.to_hit_bonus)
            && curr_wp
                .get_avg_dmg()
                .is_some_and(|x| (scales_high_avg..scales_extreme_avg).contains(&x))
            || (atk_bonus_scales.moderate..atk_bonus_scales.high).contains(&curr_wp.to_hit_bonus)
                && curr_wp
                    .get_avg_dmg()
                    .is_some_and(|x| x >= scales_extreme_avg))
    }) {
        return Some(false);
    }
    Some(true)
}

// Sniper
fn is_sniper(
    cr_core: &CreatureCoreData,
    cr_extra: &CreatureExtraData,
    cr_combat: &CreatureCombatData,
    scales: &CreatureScales,
    re: &Regex,
) -> Option<bool> {
    let lvl = cr_core.base_level;
    let per_scales = scales.perception_scales.get(&lvl)?;
    // high Perception;
    if !(per_scales.high..per_scales.extreme).contains(&(cr_extra.perception as i64)) {
        return Some(false);
    }
    let ability_scales = scales.ability_scales.get(&lvl)?;
    // high Dex modifier;
    if !(ability_scales.high..ability_scales.extreme?)
        .contains(&(cr_extra.ability_scores.dexterity as i64))
    {
        return Some(false);
    }
    let saving_scales = scales.saving_throw_scales.get(&lvl)?;
    // low Fortitude
    if !(saving_scales.low..saving_scales.moderate)
        .contains(&(cr_combat.saving_throws.fortitude as i64))
    {
        return Some(false);
    }
    // high Reflex;
    if !(saving_scales.high..saving_scales.extreme)
        .contains(&(cr_combat.saving_throws.reflex as i64))
    {
        return Some(false);
    }

    let hp_scales = scales.hp_scales.get(&lvl)?;
    // moderate to low HP;
    if !(hp_scales.low_lb..hp_scales.moderate_ub + 1).contains(&(cr_core.hp as i64)) {
        return Some(false);
    }
    let atk_bonus_scales = scales.strike_bonus_scales.get(&lvl)?;
    let dmg_scales = scales.strike_dmg_scales.get(&lvl)?;
    let scales_extreme_avg = re
        .captures(dmg_scales.extreme.as_str())?
        .get(1)?
        .as_str()
        .parse::<i64>()
        .ok()?;
    let scales_high_avg = re
        .captures(dmg_scales.high.as_str())?
        .get(1)?
        .as_str()
        .parse::<i64>()
        .ok()?;
    // ranged Strikes have high attack bonus and damage or
    // moderate attack bonus and extreme damage (melee Strikes are weaker)
    if !cr_combat.weapons.iter().any(|curr_wp| {
        curr_wp.wp_type.to_uppercase() == *"RANGED"
            && ((atk_bonus_scales.high..atk_bonus_scales.extreme).contains(&curr_wp.to_hit_bonus)
                && curr_wp
                    .get_avg_dmg()
                    .is_some_and(|x| (scales_high_avg..scales_extreme_avg).contains(&x))
                || (atk_bonus_scales.moderate..atk_bonus_scales.high)
                    .contains(&curr_wp.to_hit_bonus)
                    && curr_wp
                        .get_avg_dmg()
                        .is_some_and(|x| x >= scales_extreme_avg))
    }) {
        return Some(false);
    }
    Some(true)
}
// Skirmisher
fn is_skirmisher(
    cr_core: &CreatureCoreData,
    cr_extra: &CreatureExtraData,
    cr_combat: &CreatureCombatData,
    scales: &CreatureScales,
) -> Option<bool> {
    let lvl = cr_core.base_level;
    let ability_scales = scales.ability_scales.get(&lvl)?;
    // high Dex modifier;
    if !(ability_scales.high..ability_scales.extreme?)
        .contains(&(cr_extra.ability_scores.dexterity as i64))
    {
        return Some(false);
    }
    let saving_scales = scales.saving_throw_scales.get(&lvl)?;
    // low Fortitude
    if !(saving_scales.low..saving_scales.moderate)
        .contains(&(cr_combat.saving_throws.fortitude as i64))
    {
        return Some(false);
    }
    // high Reflex;
    if !(saving_scales.high..saving_scales.extreme)
        .contains(&(cr_combat.saving_throws.reflex as i64))
    {
        return Some(false);
    }
    // TODO: higher Speed than typical

    Some(true)
}
// Soldier
pub fn is_soldier(
    cr_core: &CreatureCoreData,
    cr_extra: &CreatureExtraData,
    cr_combat: &CreatureCombatData,
    scales: &CreatureScales,
    re: &Regex,
) -> Option<bool> {
    let lvl = cr_core.base_level;
    let ability_scales = scales.ability_scales.get(&lvl)?;
    // high Str modifier;
    if !(ability_scales.high..ability_scales.extreme?)
        .contains(&(cr_extra.ability_scores.strength as i64))
    {
        return Some(false);
    }
    let ac_scales = scales.ac_scales.get(&lvl)?;
    // high to extreme AC;
    if !(ac_scales.high..ac_scales.extreme).contains(&(cr_combat.ac as i64)) {
        return Some(false);
    }
    let saving_scales = scales.saving_throw_scales.get(&lvl)?;
    // high Fortitude;
    if !(saving_scales.high..saving_scales.extreme)
        .contains(&(cr_combat.saving_throws.fortitude as i64))
    {
        return Some(false);
    }
    let atk_bonus_scales = scales.strike_bonus_scales.get(&lvl)?;
    let dmg_scales = scales.strike_dmg_scales.get(&lvl)?;
    let scales_extreme_avg = re
        .captures(dmg_scales.extreme.as_str())?
        .get(1)?
        .as_str()
        .parse::<i64>()
        .ok()?;
    let scales_high_avg = re
        .captures(dmg_scales.high.as_str())?
        .get(1)?
        .as_str()
        .parse::<i64>()
        .ok()?;
    // high attack bonus and high damage;
    if !cr_combat.weapons.iter().any(|curr_wp| {
        (atk_bonus_scales.high..atk_bonus_scales.extreme).contains(&curr_wp.to_hit_bonus)
            && curr_wp
                .get_avg_dmg()
                .is_some_and(|x| (scales_high_avg..scales_extreme_avg).contains(&x))
    }) {
        return Some(false);
    }
    if !cr_extra.actions.iter().any(|curr_act| {
        curr_act.name.to_uppercase() == "ATTACK OF OPPORTUNITY"
            || (curr_act.slug.is_none()
                || curr_act.slug.clone().unwrap().to_uppercase() == "ATTACK-OF-OPPORTUNITY")
    }) {
        return Some(false);
    }
    // TODO: Attack of Opportunity **OR** other tactical abilities
    Some(true)
}

// Magical Striker
pub fn is_magical_striker(
    cr_core: &CreatureCoreData,
    cr_spell: &CreatureSpellCasterData,
    cr_combat: &CreatureCombatData,
    scales: &CreatureScales,
    re: &Regex,
) -> Option<bool> {
    let lvl = cr_core.base_level;
    let atk_bonus_scales = scales.strike_bonus_scales.get(&lvl)?;
    let dmg_scales = scales.strike_dmg_scales.get(&lvl)?;
    let scales_extreme_avg = re
        .captures(dmg_scales.extreme.as_str())?
        .get(1)?
        .as_str()
        .parse::<i64>()
        .ok()?;
    let scales_high_avg = re
        .captures(dmg_scales.high.as_str())?
        .get(1)?
        .as_str()
        .parse::<i64>()
        .ok()?;
    // high attack bonus and high damage;
    if !cr_combat.weapons.iter().any(|curr_wp| {
        (atk_bonus_scales.high..atk_bonus_scales.extreme).contains(&curr_wp.to_hit_bonus)
            && curr_wp
                .get_avg_dmg()
                .is_some_and(|x| (scales_high_avg..scales_extreme_avg).contains(&x))
    }) {
        return Some(false);
    }
    let spell_dc = scales.spell_dc_and_atk_scales.get(&lvl)?;
    // moderate to high spell DCs;
    if !cr_spell
        .spell_caster_entry
        .spell_casting_dc_mod
        .is_some_and(|x| (spell_dc.moderate_dc..spell_dc.high_dc).contains(&(x as i64)))
    {
        return Some(false);
    }
    // either a scattering of innate spells or prepared or spontaneous spells up to half the creature’s level (rounded up) minus 1
    Some(true)
}

// Skill Paragon
fn is_skill_paragon(
    cr_core: &CreatureCoreData,
    cr_extra: &CreatureExtraData,
    cr_combat: &CreatureCombatData,
    scales: &CreatureScales,
) -> Option<bool> {
    let lvl = cr_core.base_level;
    let ability_scales = scales.ability_scales.get(&lvl)?;
    scales.skill_scales.get(&lvl)?;
    let best_skill = cr_extra.skills.iter().map(|x| x.modifier).max()?;
    // high or extreme attribute modifier matching its best skills;
    if !(ability_scales.high..ability_scales.extreme?).contains(&best_skill) {
        return Some(false);
    };
    let saving_scales = scales.saving_throw_scales.get(&lvl)?;
    // typically high Reflex or Will and low Fortitude;
    if !(((saving_scales.high..saving_scales.extreme)
        .contains(&(cr_combat.saving_throws.reflex as i64))
        || (saving_scales.high..saving_scales.extreme)
            .contains(&(cr_combat.saving_throws.will as i64)))
        && (saving_scales.low..saving_scales.moderate)
            .contains(&(cr_combat.saving_throws.fortitude as i64)))
    {
        return Some(false);
    }
    // many skills at moderate or high and potentially one or two extreme skills;
    // Many is kinda up in the air, I'll set 70%
    let cr_skill_amount = cr_extra.skills.length() * 70 / 100;
    // if there aren't at least 70% of skill in the moderate-high range, exit
    if !cr_extra
        .skills
        .iter()
        .filter(|x| (saving_scales.moderate..saving_scales.high).contains(&x.modifier))
        .count() as u64
        >= cr_skill_amount
    {
        return Some(false);
    }
    // TODO: at least one special ability to use the creature's skills in combat
    Some(true)
}
// Spellcaster
fn is_spellcaster(
    cr_core: &CreatureCoreData,
    cr_spell: &CreatureSpellCasterData,
    cr_combat: &CreatureCombatData,
    cr_extra: &CreatureExtraData,
    scales: &CreatureScales,
    re: &Regex,
) -> Option<bool> {
    let lvl = cr_core.base_level;
    let saving_scales = scales.saving_throw_scales.get(&lvl)?;
    // low Fortitude,
    if !(saving_scales.low..saving_scales.moderate)
        .contains(&(cr_combat.saving_throws.fortitude as i64))
    {
        return Some(false);
    }
    // high Will;
    if !(saving_scales.high..saving_scales.extreme).contains(&(cr_combat.saving_throws.will as i64))
    {
        return Some(false);
    }
    // low HP;
    let hp_scales = scales.hp_scales.get(&lvl)?;
    if cr_core.hp < hp_scales.low_lb as i16 || cr_core.hp > hp_scales.low_ub as i16 {
        return Some(false);
    }
    let atk_bonus_scales = scales.strike_bonus_scales.get(&lvl)?;
    let dmg_scales = scales.strike_dmg_scales.get(&lvl)?;
    let scales_high_avg = re
        .captures(dmg_scales.high.as_str())?
        .get(1)?
        .as_str()
        .parse::<i64>()
        .ok()?;
    let scales_low_avg = re
        .captures(dmg_scales.low.as_str())?
        .get(1)?
        .as_str()
        .parse::<i64>()
        .ok()?;

    // low attack bonus and moderate or low damage;
    if cr_combat.weapons.iter().any(|curr_wp| {
        (atk_bonus_scales.low..atk_bonus_scales.moderate).contains(&curr_wp.to_hit_bonus)
            && curr_wp
                .get_avg_dmg()
                .is_some_and(|x| (scales_low_avg..scales_high_avg).contains(&x))
    }) {
        return Some(false);
    }
    // high or extreme spell DCs;
    let spells_dc_and_atk_scales = scales.spell_dc_and_atk_scales.get(&lvl)?;
    if !(spells_dc_and_atk_scales.high_dc..spells_dc_and_atk_scales.extreme_dc)
        .contains(&(cr_spell.spell_caster_entry.spell_casting_dc_mod? as i64))
    {
        return Some(false);
    }
    // prepared or spontaneous spells up to half the creature’s level (rounded up)
    if !cr_spell.spells.len() as f64 >= (cr_core.base_level as f64 / 2.).ceil() {
        return Some(false);
    }
    let ability_scales = scales.ability_scales.get(&lvl)?;
    // high or extreme modifier for the corresponding mental ability;
    if !(cr_extra.ability_scores.wisdom as i64 > ability_scales.high
        || cr_extra.ability_scores.intelligence as i64 > ability_scales.high
        || cr_extra.ability_scores.charisma as i64 > ability_scales.high)
    {
        return Some(false);
    }
    Some(true)
}

impl fmt::Display for CreatureRole {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CreatureRole::Brute => {
                write!(f, "{:?}", "Brute")
            }
            CreatureRole::MagicalStriker => {
                write!(f, "{:?}", "Magical Striker")
            }
            CreatureRole::SkillParagon => {
                write!(f, "{:?}", "Skill Paragon")
            }
            CreatureRole::Skirmisher => {
                write!(f, "{:?}", "Skirmisher")
            }
            CreatureRole::Sniper => {
                write!(f, "{:?}", "Sniper")
            }
            CreatureRole::Soldier => {
                write!(f, "{:?}", "Soldier")
            }
            CreatureRole::SpellCaster => {
                write!(f, "{:?}", "Spellcaster")
            }
        }
    }
}
