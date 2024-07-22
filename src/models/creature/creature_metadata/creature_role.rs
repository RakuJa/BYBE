use crate::models::creature::creature_component::creature_combat::CreatureCombatData;
use crate::models::creature::creature_component::creature_core::EssentialData;
use crate::models::creature::creature_component::creature_extra::CreatureExtraData;
use crate::models::creature::creature_component::creature_spell_caster::CreatureSpellCasterData;
use crate::models::item::item_metadata::type_enum::WeaponTypeEnum;
use crate::models::scales_struct::creature_scales::CreatureScales;
use num_traits::float::FloatConst;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt;
use std::str::FromStr;
use strum::{EnumIter, IntoEnumIterator};
use utoipa::ToSchema;

const MISSING_FIELD_DISTANCE: u16 = 20;

#[derive(
    Serialize, Deserialize, EnumIter, Clone, ToSchema, Eq, Hash, PartialEq, Ord, PartialOrd,
)]
pub enum CreatureRoleEnum {
    Brute,
    #[serde(rename = "Magical Striker")]
    MagicalStriker,
    #[serde(rename = "Skill Paragon")]
    SkillParagon,
    Skirmisher,
    Sniper,
    Soldier,
    SpellCaster,
}

fn get_dmg_from_regex(raw_str: &str) -> Option<i64> {
    // It will only initialize it once.
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\((\d+)\)").unwrap());
    RE.captures(raw_str)?.get(1)?.as_str().parse::<i64>().ok()
}

impl CreatureRoleEnum {
    pub fn to_db_column(&self) -> String {
        match self {
            CreatureRoleEnum::Brute => String::from("brute_percentage"),
            CreatureRoleEnum::MagicalStriker => String::from("magical_striker_percentage"),
            CreatureRoleEnum::SkillParagon => String::from("skill_paragon_percentage"),
            CreatureRoleEnum::Skirmisher => String::from("skirmisher_percentage"),
            CreatureRoleEnum::Sniper => String::from("sniper_percentage"),
            CreatureRoleEnum::Soldier => String::from("soldier_percentage"),
            CreatureRoleEnum::SpellCaster => String::from("spell_caster_percentage"),
        }
    }
    pub fn from_creature_with_given_scales(
        cr_core: &EssentialData,
        cr_extra: &CreatureExtraData,
        cr_combat: &CreatureCombatData,
        cr_spells: &CreatureSpellCasterData,
        scales: &CreatureScales,
    ) -> BTreeMap<CreatureRoleEnum, i64> {
        let mut roles = BTreeMap::new();
        roles.insert(
            Self::Brute,
            is_brute(cr_core, cr_extra, cr_combat, scales)
                .map(|x| (x * 100.).round() as i64)
                .unwrap_or(0),
        );
        roles.insert(
            Self::MagicalStriker,
            is_magical_striker(cr_core, cr_spells, cr_combat, scales)
                .map(|x| (x * 100.).round() as i64)
                .unwrap_or(0),
        );
        roles.insert(
            Self::SkillParagon,
            is_skill_paragon(cr_core, cr_extra, cr_combat, scales)
                .map(|x| (x * 100.).round() as i64)
                .unwrap_or(0),
        );
        roles.insert(
            Self::Skirmisher,
            is_skirmisher(cr_core, cr_extra, cr_combat, scales)
                .map(|x| (x * 100.).round() as i64)
                .unwrap_or(0),
        );
        roles.insert(
            Self::Sniper,
            is_sniper(cr_core, cr_extra, cr_combat, scales)
                .map(|x| (x * 100.).round() as i64)
                .unwrap_or(0),
        );
        roles.insert(
            Self::Soldier,
            is_soldier(cr_core, cr_extra, cr_combat, scales)
                .map(|x| (x * 100.).round() as i64)
                .unwrap_or(0),
        );
        roles.insert(
            Self::SpellCaster,
            is_spellcaster(cr_core, cr_spells, cr_combat, cr_extra, scales)
                .map(|x| (x * 100.).round() as i64)
                .unwrap_or(0),
        );
        roles
    }

    pub fn list() -> Vec<String> {
        CreatureRoleEnum::iter().map(|x| x.to_string()).collect()
    }
}
// Brute
fn is_brute(
    cr_core: &EssentialData,
    cr_extra: &CreatureExtraData,
    cr_combat: &CreatureCombatData,
    scales: &CreatureScales,
) -> Option<f64> {
    let mut score: u16 = 0;
    let lvl = cr_core.level;
    let per_scales = scales.perception_scales.get(&lvl)?;
    // low Perception;
    score += calculate_ub_distance(per_scales.moderate, cr_extra.perception as i64 + 1);
    let ability_scales = scales.ability_scales.get(&lvl)?;
    // high or extreme Str modifier,
    score += calculate_lb_distance(ability_scales.high, cr_extra.ability_scores.strength);
    // high to moderate Con modifier,
    let constitution = cr_extra.ability_scores.constitution;
    score += calculate_lb_distance(ability_scales.moderate, constitution);
    // low or lower mental modifiers;
    score += calculate_ub_distance(
        ability_scales.moderate,
        cr_extra.ability_scores.intelligence + 1,
    );
    score += calculate_ub_distance(ability_scales.moderate, cr_extra.ability_scores.wisdom + 1);
    score += calculate_ub_distance(
        ability_scales.moderate,
        cr_extra.ability_scores.charisma + 1,
    );
    let saving_scales = scales.saving_throw_scales.get(&lvl)?;
    // Low or lower Reflex
    score += calculate_ub_distance(saving_scales.moderate, cr_combat.saving_throws.reflex + 1);
    // high Fortitude,
    score += calculate_lb_distance(saving_scales.high, cr_combat.saving_throws.fortitude);
    // Low will,
    score += calculate_ub_distance(saving_scales.moderate, cr_combat.saving_throws.will + 1);
    let ac_scales = scales.ac_scales.get(&lvl)?;
    // moderate or low AC;
    score += calculate_ub_distance(ac_scales.high, cr_combat.ac as i64 + 1);
    // high HP;
    let hp_scales = scales.hp_scales.get(&lvl)?;
    score += calculate_lb_distance(hp_scales.high_lb, cr_core.hp);
    let atk_bonus_scales = scales.strike_bonus_scales.get(&lvl)?;
    let dmg_scales = scales.strike_dmg_scales.get(&lvl)?;

    let scales_extreme_avg = get_dmg_from_regex(dmg_scales.extreme.as_str())?;
    let scales_high_avg = get_dmg_from_regex(dmg_scales.high.as_str())?;
    // high attack bonus and high damage OR moderate attack bonus and extreme damage
    let wp_distance = cr_combat
        .weapons
        .iter()
        .filter(|wp| wp.get_avg_dmg().is_some())
        .map(|wp| {
            let avg_dmg = wp.get_avg_dmg().unwrap();
            let x = calculate_lb_distance(
                atk_bonus_scales.high,
                wp.weapon_data.to_hit_bonus.unwrap_or(0),
            ) + calculate_lb_distance(scales_high_avg, avg_dmg);
            let y = calculate_dist(
                atk_bonus_scales.moderate,
                atk_bonus_scales.high,
                wp.weapon_data.to_hit_bonus.unwrap_or(0),
            ) + calculate_lb_distance(scales_extreme_avg, avg_dmg);
            x.min(y)
        })
        .min();
    score += wp_distance.unwrap_or(MISSING_FIELD_DISTANCE);

    Some(f64::E().powf(-0.2 * (score as f64)))
}

// Sniper
fn is_sniper(
    cr_core: &EssentialData,
    cr_extra: &CreatureExtraData,
    cr_combat: &CreatureCombatData,
    scales: &CreatureScales,
) -> Option<f64> {
    let mut score: u16 = 0;
    let lvl = cr_core.level;
    let per_scales = scales.perception_scales.get(&lvl)?;
    // high Perception (chosen moderate
    // !!!This is a critical stat, upping it will half creature result!!!
    // );
    score += calculate_lb_distance(per_scales.moderate, cr_extra.perception as i64);
    let ability_scales = scales.ability_scales.get(&lvl)?;
    // high Dex modifier (chosen moderate);
    score += calculate_lb_distance(ability_scales.moderate, cr_extra.ability_scores.dexterity);
    let saving_scales = scales.saving_throw_scales.get(&lvl)?;
    // low Fortitude
    // skipped
    // high Reflex (chosen moderate);
    score += calculate_lb_distance(saving_scales.moderate, cr_combat.saving_throws.reflex);

    // moderate to low HP; skipped
    let atk_bonus_scales = scales.strike_bonus_scales.get(&lvl)?;
    let dmg_scales = scales.strike_dmg_scales.get(&lvl)?;
    let scales_mod_avg = get_dmg_from_regex(dmg_scales.moderate.as_str())?;
    // ranged Strikes have high attack bonus and damage or
    // moderate attack bonus and extreme damage (melee Strikes are weaker)
    let wp_distance = cr_combat
        .weapons
        .iter()
        .filter(|wp| {
            wp.get_avg_dmg().is_some() && wp.weapon_data.weapon_type == WeaponTypeEnum::Ranged
        })
        .map(|wp| {
            let avg_dmg = wp.get_avg_dmg().unwrap();
            calculate_lb_distance(
                atk_bonus_scales.high,
                wp.weapon_data.to_hit_bonus.unwrap_or(0),
            ) + calculate_lb_distance(scales_mod_avg, avg_dmg)
        })
        .min();
    score += wp_distance.unwrap_or(MISSING_FIELD_DISTANCE);
    Some(f64::E().powf(-0.2 * (score as f64)))
}
// Skirmisher
fn is_skirmisher(
    cr_core: &EssentialData,
    cr_extra: &CreatureExtraData,
    cr_combat: &CreatureCombatData,
    scales: &CreatureScales,
) -> Option<f64> {
    let mut score: u16 = 0;
    let lvl = cr_core.level;
    let ability_scales = scales.ability_scales.get(&lvl)?;
    // high Dex modifier;
    score += calculate_lb_distance(ability_scales.high, cr_extra.ability_scores.dexterity);
    let saving_scales = scales.saving_throw_scales.get(&lvl)?;
    // low Fortitude
    score += calculate_ub_distance(
        saving_scales.moderate,
        cr_combat.saving_throws.fortitude + 1,
    );
    // high Reflex;
    score += calculate_lb_distance(saving_scales.high, cr_combat.saving_throws.reflex);
    // Higher than avg speed (avg ~= 25)
    score += cr_extra
        .speeds
        .values()
        .map(|speed_value| calculate_lb_distance(30, *speed_value as i64))
        .min()
        .unwrap_or(MISSING_FIELD_DISTANCE);
    Some(f64::E().powf(-0.2 * (score as f64)))
}
// Soldier
pub fn is_soldier(
    cr_core: &EssentialData,
    cr_extra: &CreatureExtraData,
    cr_combat: &CreatureCombatData,
    scales: &CreatureScales,
) -> Option<f64> {
    let mut score: u16 = 0;
    let lvl = cr_core.level;
    let ability_scales = scales.ability_scales.get(&lvl)?;
    // high Str modifier;
    score += calculate_lb_distance(ability_scales.high, cr_extra.ability_scores.strength);
    let ac_scales = scales.ac_scales.get(&lvl)?;
    // high to extreme AC;
    score += calculate_lb_distance(ac_scales.high, cr_combat.ac as i64);
    let saving_scales = scales.saving_throw_scales.get(&lvl)?;
    // high Fortitude;
    score += calculate_lb_distance(saving_scales.high, cr_combat.saving_throws.fortitude);
    let atk_bonus_scales = scales.strike_bonus_scales.get(&lvl)?;
    let dmg_scales = scales.strike_dmg_scales.get(&lvl)?;
    let scales_high_avg = get_dmg_from_regex(dmg_scales.high.as_str())?;
    // high attack bonus and high damage;
    let wp_distance = cr_combat
        .weapons
        .iter()
        .filter(|wp| wp.get_avg_dmg().is_some())
        .map(|wp| {
            calculate_lb_distance(
                atk_bonus_scales.high,
                wp.weapon_data.to_hit_bonus.unwrap_or(0),
            ) + calculate_lb_distance(scales_high_avg, wp.get_avg_dmg().unwrap())
        })
        .min();

    score += wp_distance.unwrap_or(MISSING_FIELD_DISTANCE);
    if !cr_extra.actions.iter().any(|x| {
        x.category.is_some()
            && x.category.clone().unwrap().as_str().to_uppercase() == "OFFENSIVE"
            && x.action_type.as_str().to_uppercase() == "ACTION"
    }) {
        score += MISSING_FIELD_DISTANCE;
    } else if !cr_extra.actions.iter().any(|curr_act| {
        curr_act.name.to_uppercase() == "ATTACK OF OPPORTUNITY"
            || (curr_act.slug.is_none()
                || curr_act.slug.clone().unwrap().to_uppercase() == "ATTACK-OF-OPPORTUNITY")
    }) {
        score += 3;
    }
    Some(f64::E().powf(-0.2 * (score as f64)))
}

// Magical Striker
pub fn is_magical_striker(
    cr_core: &EssentialData,
    cr_spell: &CreatureSpellCasterData,
    cr_combat: &CreatureCombatData,
    scales: &CreatureScales,
) -> Option<f64> {
    let mut score: u16 = 0;
    let lvl = cr_core.level;
    let atk_bonus_scales = scales.strike_bonus_scales.get(&lvl)?;
    let dmg_scales = scales.strike_dmg_scales.get(&lvl)?;
    let scales_high_avg = get_dmg_from_regex(dmg_scales.high.as_str())?;
    // high attack bonus and high damage;
    let wp_distance = cr_combat
        .weapons
        .iter()
        .filter(|wp| wp.get_avg_dmg().is_some())
        .map(|wp| {
            calculate_lb_distance(
                atk_bonus_scales.high,
                wp.weapon_data.to_hit_bonus.unwrap_or(0),
            ) + calculate_lb_distance(scales_high_avg, wp.get_avg_dmg().unwrap())
        })
        .min();
    score += wp_distance.unwrap_or(MISSING_FIELD_DISTANCE);
    let spell_dc = scales.spell_dc_and_atk_scales.get(&lvl)?;
    // moderate to high spell DCs;
    if cr_spell.spell_caster_entry.spell_casting_dc_mod.is_some() {
        score += calculate_lb_distance(
            spell_dc.moderate_dc,
            cr_spell.spell_caster_entry.spell_casting_dc_mod.unwrap(),
        );
    } else {
        score += MISSING_FIELD_DISTANCE;
    }
    if (cr_spell.spells.len() as f64) < (cr_core.level as f64 / 2.).ceil() - 1. {
        score += (((cr_core.level as f64 / 2.).ceil() as i64) - 1 - (cr_spell.spells.len() as i64))
            .unsigned_abs() as u16;
    }
    Some(f64::E().powf(-0.2 * (score as f64)))
}

// Skill Paragon
fn is_skill_paragon(
    cr_core: &EssentialData,
    cr_extra: &CreatureExtraData,
    cr_combat: &CreatureCombatData,
    scales: &CreatureScales,
) -> Option<f64> {
    let mut score: u16 = 0;
    let lvl = cr_core.level;
    let ability_scales = scales.ability_scales.get(&lvl)?;
    scales.skill_scales.get(&lvl)?;
    let best_skill = cr_extra.skills.iter().map(|x| x.modifier).max()?;
    // high or extreme attribute modifier matching its best skills;
    score += calculate_lb_distance(ability_scales.high, best_skill);
    let saving_scales = scales.saving_throw_scales.get(&lvl)?;
    // typically high Reflex or Will and low Fortitude;
    score += calculate_ub_distance(
        saving_scales.moderate,
        cr_combat.saving_throws.fortitude + 1,
    );

    let ref_dist = calculate_lb_distance(saving_scales.high, cr_combat.saving_throws.reflex);
    let will_dist = calculate_lb_distance(saving_scales.high, cr_combat.saving_throws.will);
    score += if ref_dist > will_dist {
        will_dist
    } else {
        ref_dist
    };

    // many skills at moderate or high and potentially one or two extreme skills;
    // Many is kinda up in the air, I'll set 70%
    let cr_skill_amount = cr_extra.skills.len() as i64 * 70 / 100;
    // if there aren't at least 70% of skill in the moderate-high range
    score += (cr_extra
        .skills
        .iter()
        .filter(|x| x.modifier >= saving_scales.moderate)
        .count() as i64
        - cr_skill_amount)
        .unsigned_abs() as u16;
    // at least two special ability to use the creature's skills in combat
    if cr_extra
        .actions
        .iter()
        .filter(|x| {
            x.category.is_some()
                && x.category.clone().unwrap().as_str().to_uppercase() == "OFFENSIVE"
                && x.action_type.as_str().to_uppercase() == "ACTION"
        })
        .count()
        < 2
    {
        score += MISSING_FIELD_DISTANCE;
    }
    Some(f64::E().powf(-0.2 * (score as f64)))
}
// Spellcaster
fn is_spellcaster(
    cr_core: &EssentialData,
    cr_spell: &CreatureSpellCasterData,
    cr_combat: &CreatureCombatData,
    cr_extra: &CreatureExtraData,
    scales: &CreatureScales,
) -> Option<f64> {
    let mut score: u16 = 0;
    let lvl = cr_core.level;
    let saving_scales = scales.saving_throw_scales.get(&lvl)?;
    // low Fortitude,
    score += calculate_ub_distance(
        saving_scales.moderate,
        cr_combat.saving_throws.fortitude + 1,
    );
    // high Will;
    score += calculate_lb_distance(saving_scales.high, cr_combat.saving_throws.will);
    // low HP;
    let hp_scales = scales.hp_scales.get(&lvl)?;
    score += calculate_ub_distance(hp_scales.high_lb, cr_core.hp + 1);
    // low attack bonus and moderate or low damage;
    // skipped
    // high or extreme spell DCs;
    let spells_dc_and_atk_scales = scales.spell_dc_and_atk_scales.get(&lvl)?;
    score += calculate_lb_distance(
        spells_dc_and_atk_scales.high_dc,
        cr_spell.spell_caster_entry.spell_casting_dc_mod?,
    );
    // prepared or spontaneous spells up to half the creature’s level (rounded up)
    if (cr_spell.spells.len() as f64) < (cr_core.level as f64 / 2.).ceil() {
        score += (((cr_core.level as f64 / 2.).ceil() as i64) - (cr_spell.spells.len() as i64))
            .unsigned_abs() as u16;
    }
    let ability_scales = scales.ability_scales.get(&lvl)?;
    // high or extreme modifier for the corresponding mental ability;
    let best_mental_ability = cr_extra
        .ability_scores
        .wisdom
        .max(cr_extra.ability_scores.intelligence)
        .max(cr_extra.ability_scores.charisma);
    score += calculate_lb_distance(ability_scales.high, best_mental_ability);
    Some(f64::E().powf(-0.2 * (score as f64)))
}

impl FromStr for CreatureRoleEnum {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "BRUTE" => Ok(CreatureRoleEnum::Brute),
            "MAGICAL STRIKER" | "MAGICALSTRIKER" => Ok(CreatureRoleEnum::MagicalStriker),
            "SKILL PARAGON" | "SKILLPARAGON" => Ok(CreatureRoleEnum::SkillParagon),
            "SKIRMISHER" => Ok(CreatureRoleEnum::Skirmisher),
            "SNIPER" => Ok(CreatureRoleEnum::Sniper),
            "SOLDIER" => Ok(CreatureRoleEnum::Soldier),
            "SPELLCASTER" | "SPELL CASTER" => Ok(CreatureRoleEnum::SpellCaster),
            _ => Err(()),
        }
    }
}

impl fmt::Display for CreatureRoleEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CreatureRoleEnum::Brute => {
                write!(f, "Brute")
            }
            CreatureRoleEnum::MagicalStriker => {
                write!(f, "Magical Striker")
            }
            CreatureRoleEnum::SkillParagon => {
                write!(f, "Skill Paragon")
            }
            CreatureRoleEnum::Skirmisher => {
                write!(f, "Skirmisher")
            }
            CreatureRoleEnum::Sniper => {
                write!(f, "Sniper")
            }
            CreatureRoleEnum::Soldier => {
                write!(f, "Soldier")
            }
            CreatureRoleEnum::SpellCaster => {
                write!(f, "SpellCaster")
            }
        }
    }
}

/// Calculate value distance from upper bound, lower than ub value will
/// yield 0
fn calculate_ub_distance(ub: i64, value: i64) -> u16 {
    if value > ub {
        (value - ub).unsigned_abs() as u16
    } else {
        0
    }
}

/// Calculate value distance from lower bound, higher than lb value will
/// yield 0
fn calculate_lb_distance(lb: i64, value: i64) -> u16 {
    if value < lb {
        (lb - value).unsigned_abs() as u16
    } else {
        0
    }
}

/// Calculates value distance from bounds, it will exclude upper bound
fn calculate_dist(lb: i64, ub: i64, value: i64) -> u16 {
    if value < lb {
        (lb - value).unsigned_abs() as u16
    } else if value >= ub {
        // if value = 30 and ub=30 you are 1 from the range, not 0
        (value + 1 - ub).unsigned_abs() as u16
    } else {
        0
    }
}
