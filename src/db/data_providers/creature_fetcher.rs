use crate::db::data_providers::generic_fetcher::{
    fetch_action_traits, fetch_armor_runes, fetch_armor_traits, fetch_item_traits,
    fetch_shield_traits, fetch_weapon_damage_data, fetch_weapon_runes, fetch_weapon_traits,
};
use crate::db::data_providers::raw_query_builder::{
    format_pagination_clause, prepare_filtered_get_creatures_core,
};
use crate::models::bestiary_structs::BestiaryFilterQuery;
use crate::models::creature::creature_component::creature_combat::{
    CreatureCombatData, SavingThrows,
};
use crate::models::creature::creature_component::creature_core::CreatureCoreData;
use crate::models::creature::creature_component::creature_extra::{
    AbilityScores, CreatureExtraData,
};
use crate::models::creature::creature_component::creature_spellcaster::CreatureSpellcasterData;
use crate::models::creature::creature_component::creature_variant::CreatureVariantData;
use crate::models::creature::creature_metadata::variant_enum::CreatureVariant;
use crate::models::creature::creature_struct::Creature;
use crate::models::creature::items::skill::Skill;
use crate::models::creature::items::spell::Spell;
use crate::models::creature::items::spellcaster_entry::{SpellcasterData, SpellcasterEntry};
use crate::models::db::raw_language::RawLanguage;
use crate::models::db::raw_speed::RawSpeed;
use crate::models::db::raw_weakness::RawWeakness;
use crate::models::db::resistance::CoreResistanceData;
use crate::models::db::resistance::Resistance;
use crate::models::db::sense::Sense;
use crate::models::item::armor_struct::Armor;
use crate::models::item::item_struct::Item;
use crate::models::item::shield_struct::Shield;
use crate::models::item::weapon_struct::Weapon;
use crate::models::response_data::CreatureResponseDataModifiers;
use crate::models::scales_struct::ability_scales::AbilityScales;
use crate::models::scales_struct::ac_scales::AcScales;
use crate::models::scales_struct::area_dmg_scales::AreaDmgScales;
use crate::models::scales_struct::creature_scales::CreatureScales;
use crate::models::scales_struct::hp_scales::HpScales;
use crate::models::scales_struct::item_scales::ItemScales;
use crate::models::scales_struct::perception_scales::PerceptionScales;
use crate::models::scales_struct::res_weak_scales::ResWeakScales;
use crate::models::scales_struct::saving_throw_scales::SavingThrowScales;
use crate::models::scales_struct::skill_scales::SkillScales;
use crate::models::scales_struct::spell_dc_and_atk_scales::SpellDcAndAtkScales;
use crate::models::scales_struct::strike_bonus_scales::StrikeBonusScales;
use crate::models::scales_struct::strike_dmg_scales::StrikeDmgScales;
use crate::models::shared::action::{Action, CoreAction};
use crate::models::shared::alignment_enum::ALIGNMENT_TRAITS;
use crate::models::shared::game_system_enum::GameSystem;
use anyhow::Result;
use futures::future::join_all;
use sqlx::PgPool;
use std::collections::HashMap;

async fn fetch_creature_immunities(
    pool: &PgPool,
    gs: GameSystem,
    creature_id: i64,
) -> Result<Vec<Option<String>>> {
    Ok(sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
        "SELECT name FROM {gs}_immunity_table INTERSECT SELECT immunity_id
         FROM {gs}_immunity_creature_association_table WHERE creature_id = $1"
    )))
    .bind(creature_id)
    .fetch_all(pool)
    .await?)
}

async fn fetch_creature_languages(
    pool: &PgPool,
    gs: GameSystem,
    creature_id: i64,
) -> Result<Vec<RawLanguage>> {
    Ok(sqlx::query_as(sqlx::AssertSqlSafe(format!(
        "SELECT * FROM {gs}_language_table INTERSECT SELECT language_id
         FROM {gs}_language_creature_association_table WHERE creature_id = $1"
    )))
    .bind(creature_id)
    .fetch_all(pool)
    .await?)
}

async fn fetch_creature_resistances(
    pool: &PgPool,
    gs: GameSystem,
    creature_id: i64,
) -> Result<Vec<Resistance>> {
    Ok(join_all(
        fetch_creature_resistances_core(pool, gs, creature_id)
            .await?
            .iter()
            .map(async |x| {
                let (double_vs, exception_vs) = fetch_creature_resistances_vs(pool, gs, x.id)
                    .await
                    .unwrap_or_default();
                Resistance {
                    core: x.clone(),
                    double_vs,
                    exception_vs,
                }
            }),
    )
    .await)
}

async fn fetch_creature_resistances_core(
    pool: &PgPool,
    gs: GameSystem,
    creature_id: i64,
) -> Result<Vec<CoreResistanceData>> {
    Ok(sqlx::query_as(sqlx::AssertSqlSafe(format!(
        "SELECT id, name, value FROM {gs}_resistance_table WHERE creature_id = $1"
    )))
    .bind(creature_id)
    .fetch_all(pool)
    .await?)
}

async fn fetch_creature_resistances_vs(
    pool: &PgPool,
    gs: GameSystem,
    res_id: i64,
) -> Result<(Vec<String>, Vec<String>)> {
    Ok((
        sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
            "SELECT vs_name FROM {gs}_resistance_double_vs_table WHERE resistance_id = $1"
        )))
        .bind(res_id)
        .fetch_all(pool)
        .await?,
        sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
            "SELECT vs_name FROM {gs}_resistance_exception_vs_table WHERE resistance_id = $1"
        )))
        .bind(res_id)
        .fetch_all(pool)
        .await?,
    ))
}

async fn fetch_creature_senses(
    pool: &PgPool,
    gs: GameSystem,
    creature_id: i64,
) -> Result<Vec<Sense>> {
    Ok(sqlx::query_as(sqlx::AssertSqlSafe(format!(
        "SELECT * FROM {gs}_sense_table WHERE id IN (SELECT sense_id
         FROM {gs}_sense_creature_association_table WHERE creature_id = $1)"
    )))
    .bind(creature_id)
    .fetch_all(pool)
    .await?)
}

async fn fetch_creature_speeds(
    pool: &PgPool,
    gs: GameSystem,
    creature_id: i64,
) -> Result<Vec<RawSpeed>> {
    Ok(sqlx::query_as(sqlx::AssertSqlSafe(format!(
        "SELECT name, value FROM {gs}_speed_table WHERE creature_id = $1"
    )))
    .bind(creature_id)
    .fetch_all(pool)
    .await?)
}

async fn fetch_creature_weaknesses(
    pool: &PgPool,
    gs: GameSystem,
    creature_id: i64,
) -> Result<Vec<RawWeakness>> {
    Ok(sqlx::query_as(sqlx::AssertSqlSafe(format!(
        "SELECT name, value FROM {gs}_weakness_table WHERE creature_id = $1"
    )))
    .bind(creature_id)
    .fetch_all(pool)
    .await?)
}

async fn fetch_creature_saving_throws(
    pool: &PgPool,
    gs: GameSystem,
    creature_id: i64,
) -> Result<SavingThrows> {
    Ok(sqlx::query_as(sqlx::AssertSqlSafe(format!(
        "SELECT fortitude, reflex, will, fortitude_detail, reflex_detail, will_detail
         FROM {gs}_creature_table WHERE id = $1"
    )))
    .bind(creature_id)
    .fetch_one(pool)
    .await?)
}

async fn fetch_creature_ability_scores(
    pool: &PgPool,
    gs: GameSystem,
    creature_id: i64,
) -> Result<AbilityScores> {
    Ok(sqlx::query_as(sqlx::AssertSqlSafe(format!(
        "SELECT charisma, constitution, dexterity, intelligence, strength, wisdom
         FROM {gs}_creature_table WHERE id = $1"
    )))
    .bind(creature_id)
    .fetch_one(pool)
    .await?)
}

async fn fetch_creature_ac(pool: &PgPool, gs: GameSystem, creature_id: i64) -> Result<i32> {
    Ok(sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
        "SELECT ac FROM {gs}_creature_table WHERE id = $1"
    )))
    .bind(creature_id)
    .fetch_one(pool)
    .await?)
}

async fn fetch_creature_ac_detail(
    pool: &PgPool,
    gs: GameSystem,
    creature_id: i64,
) -> Result<Option<String>> {
    Ok(sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
        "SELECT ac_detail FROM {gs}_creature_table WHERE id = $1 LIMIT 1"
    )))
    .bind(creature_id)
    .fetch_optional(pool)
    .await?)
}

async fn fetch_creature_hp_detail(
    pool: &PgPool,
    gs: GameSystem,
    creature_id: i64,
) -> Result<Option<String>> {
    Ok(sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
        "SELECT hp_detail FROM {gs}_creature_table WHERE id = $1 LIMIT 1"
    )))
    .bind(creature_id)
    .fetch_optional(pool)
    .await?)
}

async fn fetch_creature_language_detail(
    pool: &PgPool,
    gs: GameSystem,
    creature_id: i64,
) -> Result<Option<String>> {
    Ok(sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
        "SELECT language_detail FROM {gs}_creature_table WHERE id = $1 LIMIT 1"
    )))
    .bind(creature_id)
    .fetch_optional(pool)
    .await?)
}

async fn fetch_creature_perception(pool: &PgPool, gs: GameSystem, creature_id: i64) -> Result<i32> {
    Ok(sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
        "SELECT perception FROM {gs}_creature_table WHERE id = $1 LIMIT 1"
    )))
    .bind(creature_id)
    .fetch_one(pool)
    .await?)
}

async fn fetch_creature_vision(pool: &PgPool, gs: GameSystem, creature_id: i64) -> Result<bool> {
    Ok(sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
        "SELECT vision FROM {gs}_creature_table WHERE id = $1 LIMIT 1"
    )))
    .bind(creature_id)
    .fetch_one(pool)
    .await?)
}

async fn fetch_creature_perception_detail(
    pool: &PgPool,
    gs: GameSystem,
    creature_id: i64,
) -> Result<Option<String>> {
    Ok(sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
        "SELECT perception_detail FROM {gs}_creature_table WHERE id = $1 LIMIT 1"
    )))
    .bind(creature_id)
    .fetch_optional(pool)
    .await?)
}

pub async fn fetch_creature_traits(
    pool: &PgPool,
    gs: GameSystem,
    creature_id: i64,
) -> Result<Vec<String>> {
    Ok(sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
        "SELECT name FROM {gs}_trait_table INTERSECT SELECT trait_id
             FROM {gs}_trait_creature_association_table WHERE creature_id = $1"
    )))
    .bind(creature_id)
    .fetch_all(pool)
    .await?)
}

pub async fn fetch_all_creature_traits(
    pool: &PgPool,
    gs: GameSystem,
) -> Result<HashMap<i64, Vec<String>>> {
    let rows: Vec<(i64, String)> = sqlx::query_as(sqlx::AssertSqlSafe(format!(
        "SELECT a.creature_id, t.name
         FROM {gs}_trait_creature_association_table a
         JOIN {gs}_trait_table t ON t.name = a.trait_id"
    )))
    .fetch_all(pool)
    .await?;

    let mut map: HashMap<i64, Vec<String>> = HashMap::new();
    for (creature_id, trait_name) in rows {
        map.entry(creature_id).or_default().push(trait_name);
    }
    Ok(map)
}

async fn fetch_creature_weapons(
    pool: &PgPool,
    gs: GameSystem,
    creature_id: i64,
) -> Result<Vec<Weapon>> {
    let weapons: Vec<Weapon> = sqlx::query_as(sqlx::AssertSqlSafe(format!(
        "
        SELECT wt.id AS weapon_id, wt.to_hit_bonus, wt.splash_dmg, wt.n_of_potency_runes,
        wt.n_of_striking_runes, wt.range, wt.reload, wt.weapon_type, wt.base_item_id,
        it.*
        FROM {gs}_weapon_creature_association_table ica
        LEFT JOIN {gs}_weapon_table wt ON wt.id = ica.weapon_id
        LEFT JOIN {gs}_item_table it ON it.id = wt.base_item_id
        WHERE ica.creature_id = $1
        GROUP BY ica.weapon_id
        ORDER BY name
        "
    )))
    .bind(creature_id)
    .fetch_all(pool)
    .await?;
    let mut result_vec = Vec::new();
    for mut el in weapons {
        el.item_core.traits = fetch_weapon_traits(pool, gs, el.weapon_data.id)
            .await
            .unwrap_or(vec![]);
        el.item_core.quantity = fetch_weapon_quantity(pool, gs, creature_id, el.weapon_data.id)
            .await
            .unwrap_or(1);
        el.weapon_data.property_runes = fetch_weapon_runes(pool, gs, el.weapon_data.id)
            .await
            .unwrap_or(vec![]);
        el.weapon_data.damage_data = fetch_weapon_damage_data(pool, gs, el.weapon_data.id)
            .await
            .unwrap_or(vec![]);
        result_vec.push(el);
    }
    Ok(result_vec)
}

async fn fetch_creature_armors(
    pool: &PgPool,
    gs: GameSystem,
    creature_id: i64,
) -> Result<Vec<Armor>> {
    let armors: Vec<Armor> = sqlx::query_as(sqlx::AssertSqlSafe(format!(
        "
        SELECT at.id AS armor_id, at.bonus_ac, at.check_penalty, at.dex_cap, at.n_of_potency_runes,
        at.n_of_resilient_runes, at.speed_penalty, at.strength_required, at.base_item_id,
        it.*
        FROM {gs}_armor_creature_association_table aca
        LEFT JOIN {gs}_armor_table at ON at.id = aca.armor_id
        LEFT JOIN {gs}_item_table it ON it.id = at.base_item_id
        WHERE aca.creature_id = $1
        GROUP BY aca.armor_id
        ORDER BY name
        "
    )))
    .bind(creature_id)
    .fetch_all(pool)
    .await?;
    let mut result_vec = Vec::new();
    for mut el in armors {
        el.item_core.traits = fetch_armor_traits(pool, gs, el.armor_data.id)
            .await
            .unwrap_or(vec![]);
        el.item_core.quantity = fetch_armor_quantity(pool, gs, creature_id, el.armor_data.id)
            .await
            .unwrap_or(1);
        el.armor_data.property_runes = fetch_armor_runes(pool, gs, el.armor_data.id)
            .await
            .unwrap_or(vec![]);
        result_vec.push(el);
    }
    Ok(result_vec)
}

async fn fetch_creature_shields(
    pool: &PgPool,
    gs: GameSystem,
    creature_id: i64,
) -> Result<Vec<Shield>> {
    let shields: Vec<Shield> = sqlx::query_as(sqlx::AssertSqlSafe(format!(
        "
        SELECT st.id AS shield_id, st.bonus_ac, st.n_of_reinforcing_runes, st.speed_penalty,
        it.*
        FROM {gs}_shield_creature_association_table sca
        LEFT JOIN {gs}_shield_table st ON st.id = sca.shield_id
        LEFT JOIN {gs}_item_table it ON it.id = st.base_item_id
        WHERE sca.creature_id = $1
        GROUP BY sca.shield_id
        ORDER BY name
        ",
    )))
    .bind(creature_id)
    .fetch_all(pool)
    .await?;
    let mut result_vec = Vec::new();
    for mut el in shields {
        el.item_core.traits = fetch_shield_traits(pool, gs, el.shield_data.id)
            .await
            .unwrap_or(vec![]);
        el.item_core.quantity = fetch_shield_quantity(pool, gs, creature_id, el.shield_data.id)
            .await
            .unwrap_or(1);
        result_vec.push(el);
    }
    Ok(result_vec)
}

async fn fetch_creature_items(
    pool: &PgPool,
    gs: GameSystem,
    creature_id: i64,
) -> Result<Vec<Item>> {
    let items: Vec<Item> = sqlx::query_as(sqlx::AssertSqlSafe(format!(
        "SELECT * FROM {gs}_item_table WHERE id IN (
            SELECT item_id FROM {gs}_item_creature_association_table WHERE creature_id = $1
            )
            AND id NOT IN (SELECT base_item_id FROM {gs}_armor_table)
            AND id NOT IN (SELECT base_item_id FROM {gs}_weapon_table)
            AND id NOT IN (SELECT base_item_id FROM {gs}_shield_table)
            "
    )))
    .bind(creature_id)
    .fetch_all(pool)
    .await?;
    let mut result_vec = Vec::new();
    for mut el in items {
        el.traits = fetch_item_traits(pool, gs, el.id).await.unwrap_or(vec![]);
        el.quantity = fetch_item_quantity(pool, gs, creature_id, el.id)
            .await
            .unwrap_or(1);
        result_vec.push(el);
    }
    Ok(result_vec)
}

/// Quantities are present ONLY for creature's item.
/// It needs to be fetched from the association table.
async fn fetch_item_quantity(
    pool: &PgPool,
    gs: GameSystem,
    creature_id: i64,
    item_id: i64,
) -> Result<i64> {
    Ok(sqlx::query_scalar::<_, i64>(sqlx::AssertSqlSafe(format!(
        "SELECT quantity FROM {gs}_item_creature_association_table WHERE
        creature_id = $1 AND item_id = $2"
    )))
    .bind(creature_id)
    .bind(item_id)
    .fetch_one(pool)
    .await?)
}

/// Quantities are present ONLY for creature's weapons.
/// It needs to be fetched from the association table.
async fn fetch_weapon_quantity(
    pool: &PgPool,
    gs: GameSystem,
    creature_id: i64,
    weapon_id: i64,
) -> Result<i64> {
    Ok(sqlx::query_scalar::<_, i64>(sqlx::AssertSqlSafe(format!(
        "SELECT quantity FROM {gs}_weapon_creature_association_table WHERE
        creature_id = $1 AND weapon_id = $2"
    )))
    .bind(creature_id)
    .bind(weapon_id)
    .fetch_one(pool)
    .await?)
}

/// Quantities are present ONLY for creature's shields.
/// It needs to be fetched from the association table.
async fn fetch_shield_quantity(
    pool: &PgPool,
    gs: GameSystem,
    creature_id: i64,
    shield_id: i64,
) -> Result<i64> {
    Ok(sqlx::query_scalar::<_, i64>(sqlx::AssertSqlSafe(format!(
        "SELECT quantity FROM {gs}_shield_creature_association_table WHERE
        creature_id = $1 AND shield_id = $2"
    )))
    .bind(creature_id)
    .bind(shield_id)
    .fetch_one(pool)
    .await?)
}

/// Quantities are present ONLY for creature's armors.
/// It needs to be fetched from the association table.
async fn fetch_armor_quantity(
    pool: &PgPool,
    gs: GameSystem,
    creature_id: i64,
    armor_id: i64,
) -> Result<i64> {
    Ok(sqlx::query_scalar::<_, i64>(sqlx::AssertSqlSafe(format!(
        "SELECT quantity FROM {gs}_armor_creature_association_table WHERE
        creature_id = $1 AND armor_id = $2"
    )))
    .bind(creature_id)
    .bind(armor_id)
    .fetch_one(pool)
    .await?)
}

async fn fetch_creature_actions(
    pool: &PgPool,
    gs: GameSystem,
    creature_id: i64,
) -> Result<Vec<Action>> {
    let core_actions: Vec<CoreAction> = sqlx::query_as(sqlx::AssertSqlSafe(format!(
        "SELECT a.* FROM {gs}_action_table AS a
        JOIN {gs}_creature_action_association_table AS ca ON ca.action_id = a.id
        WHERE ca.creature_id = $1"
    )))
    .bind(creature_id)
    .fetch_all(pool)
    .await?;
    let mut res: Vec<Action> = Vec::with_capacity(core_actions.len());
    for action in core_actions {
        let action_id = action.id;
        res.push(Action {
            core_action: action,
            traits: fetch_action_traits(pool, gs, action_id).await?,
        });
    }
    Ok(res)
}

async fn fetch_creature_skills(
    pool: &PgPool,
    gs: GameSystem,
    creature_id: i64,
) -> Result<Vec<Skill>> {
    Ok(sqlx::query_as(sqlx::AssertSqlSafe(format!(
        "SELECT name, description, modifier, proficiency FROM {gs}_skill_table WHERE creature_id = $1"
    )))
    .bind(creature_id)
    .fetch_all(pool)
    .await?)
}

pub async fn fetch_creature_spells(
    pool: &PgPool,
    gs: GameSystem,
    creature_id: i64,
    spellcaster_entry_id: i64,
) -> Result<Vec<Spell>> {
    Ok(sqlx::query_as(sqlx::AssertSqlSafe(format!(
        "SELECT * FROM {gs}_spell_table WHERE creature_id = $1 AND spellcasting_entry_id = $2"
    )))
    .bind(creature_id)
    .bind(spellcaster_entry_id)
    .fetch_all(pool)
    .await?)
}

async fn fetch_creature_spellcaster_entries(
    pool: &PgPool,
    gs: GameSystem,
    creature_id: i64,
) -> Result<Vec<SpellcasterEntry>> {
    let mut result = Vec::new();
    let data: Vec<SpellcasterData> = sqlx::query_as(sqlx::AssertSqlSafe(format!(
        "SELECT
        id, spellcasting_name, is_spellcasting_flexible, type_of_spellcaster,
        spellcasting_dc_mod, spellcasting_atk_mod, spellcasting_tradition, heighten_level
        FROM {gs}_spellcasting_entry_table WHERE creature_id = $1"
    )))
    .bind(creature_id)
    .fetch_all(pool)
    .await?;
    for sce in data {
        let sce_id = sce.id;
        result.push(SpellcasterEntry {
            spellcaster_data: sce,
            spells: fetch_creature_spells(pool, gs, creature_id, sce_id)
                .await
                .unwrap_or_default(),
        });
    }
    Ok(result)
}

async fn fetch_creature_core_data(
    pool: &PgPool,
    gs: GameSystem,
    creature_id: i64,
) -> Result<CreatureCoreData> {
    let mut cr_core: CreatureCoreData = sqlx::query_as(sqlx::AssertSqlSafe(format!(
        "SELECT * FROM {gs}_creature_core WHERE id = $1 AND status = 'valid' ORDER BY name LIMIT 1"
    )))
    .bind(creature_id)
    .fetch_one(pool)
    .await?;
    cr_core.traits = fetch_creature_traits(pool, gs, creature_id)
        .await
        .unwrap_or_default()
        .iter()
        .filter(|x| !ALIGNMENT_TRAITS.contains(&&*x.as_str().to_uppercase()))
        .cloned()
        .collect();
    Ok(cr_core)
}

async fn update_creatures_core_with_traits(
    pool: &PgPool,
    gs: GameSystem,
    mut creature_core_data: Vec<CreatureCoreData>,
) -> Vec<CreatureCoreData> {
    for core in &mut creature_core_data {
        core.traits = fetch_creature_traits(pool, gs, core.essential.id)
            .await
            .unwrap_or_default()
            .iter()
            .filter(|x| !ALIGNMENT_TRAITS.contains(&&*x.as_str().to_uppercase()))
            .cloned()
            .collect();
    }
    creature_core_data
}

pub async fn fetch_traits_associated_with_creatures(
    pool: &PgPool,
    gs: GameSystem,
) -> Result<Vec<String>> {
    Ok(sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
        "
        SELECT
            tt.name
        FROM {gs}_trait_creature_association_table tcat
            LEFT JOIN {gs}_trait_table tt ON tcat.trait_id = tt.name GROUP BY tt.name",
    )))
    .fetch_all(pool)
    .await?
    .iter()
    .filter(|x: &&String| !ALIGNMENT_TRAITS.contains(&&*x.to_uppercase()))
    .cloned()
    .collect())
}

pub async fn fetch_creature_by_id(
    pool: &PgPool,
    gs: GameSystem,
    variant: CreatureVariant,
    response_data_mods: &CreatureResponseDataModifiers,
    id: i64,
) -> Result<Creature> {
    let core_data = fetch_creature_core_data(pool, gs, id).await?;
    let level = core_data.essential.base_level;
    let archive_link = core_data.derived.archive_link.clone();
    let cr = Creature {
        core_data,
        variant_data: CreatureVariantData {
            variant: CreatureVariant::Base,
            level,
            archive_link,
        },
        extra_data: if response_data_mods.extra_data.is_some_and(|x| x) {
            Some(fetch_creature_extra_data(pool, gs, id).await?)
        } else {
            None
        },
        combat_data: if response_data_mods.combat_data.is_some_and(|x| x) {
            Some(fetch_creature_combat_data(pool, gs, id).await?)
        } else {
            None
        },
        spellcaster_data: if response_data_mods.spellcasting_data.is_some_and(|x| x) {
            Some(fetch_creature_spellcaster_data(pool, gs, id).await?)
        } else {
            None
        },
        game_system: gs,
    }
    .convert_creature_to_variant(variant);
    Ok(if response_data_mods.is_pwl_on.unwrap_or(false) {
        cr.convert_creature_to_pwl()
    } else {
        cr
    })
}

pub async fn fetch_creatures_core_data_with_filters(
    pool: &PgPool,
    gs: GameSystem,
    bestiary_filter_query: &BestiaryFilterQuery,
) -> Result<Vec<CreatureCoreData>> {
    let query = prepare_filtered_get_creatures_core(gs, bestiary_filter_query);
    let core_data: Vec<CreatureCoreData> = sqlx::query_as(sqlx::AssertSqlSafe(query))
        .fetch_all(pool)
        .await?;
    Ok(update_creatures_core_with_traits(pool, gs, core_data).await)
}

/// Gets all the creatures core it can find with the given pagination as boundaries
/// for the search.
pub async fn fetch_creatures_core_data(
    pool: &PgPool,
    gs: GameSystem,
    cursor: i64,
    page_size: i16,
) -> Result<Vec<CreatureCoreData>> {
    let pagination = format_pagination_clause(cursor, page_size);
    let cr_core: Vec<CreatureCoreData> = sqlx::query_as(sqlx::AssertSqlSafe(format!(
        "SELECT * FROM {gs}_creature_core WHERE status = 'valid' ORDER BY name {pagination}"
    )))
    .fetch_all(pool)
    .await?;
    Ok(update_creatures_core_with_traits(pool, gs, cr_core).await)
}

pub async fn fetch_creature_extra_data(
    pool: &PgPool,
    gs: GameSystem,
    creature_id: i64,
) -> Result<CreatureExtraData> {
    let items = fetch_creature_items(pool, gs, creature_id).await;
    let actions = fetch_creature_actions(pool, gs, creature_id).await;
    let skills = fetch_creature_skills(pool, gs, creature_id).await;
    let languages = fetch_creature_languages(pool, gs, creature_id).await;
    let senses = fetch_creature_senses(pool, gs, creature_id).await;
    let speeds = fetch_creature_speeds(pool, gs, creature_id).await;
    let ability_scores = fetch_creature_ability_scores(pool, gs, creature_id).await;
    let hp_detail = fetch_creature_hp_detail(pool, gs, creature_id).await;
    let ac_detail = fetch_creature_ac_detail(pool, gs, creature_id).await;
    let language_detail = fetch_creature_language_detail(pool, gs, creature_id).await;
    let perception = fetch_creature_perception(pool, gs, creature_id).await;
    let has_vision = fetch_creature_vision(pool, gs, creature_id).await;
    let perception_detail = fetch_creature_perception_detail(pool, gs, creature_id).await;
    Ok(CreatureExtraData {
        actions: actions.unwrap_or_default(),
        skills: skills.unwrap_or_default(),
        items: items.unwrap_or_default(),
        languages: languages
            .unwrap_or_default()
            .iter()
            .filter_map(|x| x.name.clone())
            .collect(),
        senses: senses.unwrap_or_default(),
        speeds: speeds
            .unwrap_or_default()
            .iter()
            .map(|x| (x.name.clone(), x.value as i16))
            .collect(),
        ability_scores: ability_scores?,
        hp_detail: hp_detail?,
        ac_detail: ac_detail?,
        language_detail: language_detail?,
        perception: perception?,
        perception_detail: perception_detail?,
        has_vision: has_vision?,
    })
}

pub async fn fetch_creature_combat_data(
    pool: &PgPool,
    gs: GameSystem,
    creature_id: i64,
) -> Result<CreatureCombatData> {
    let weapons = fetch_creature_weapons(pool, gs, creature_id).await;
    let armors = fetch_creature_armors(pool, gs, creature_id).await;
    let shields = fetch_creature_shields(pool, gs, creature_id).await;
    let resistances = fetch_creature_resistances(pool, gs, creature_id).await;
    let immunities = fetch_creature_immunities(pool, gs, creature_id).await;
    let weaknesses = fetch_creature_weaknesses(pool, gs, creature_id).await;
    let saving_throws = fetch_creature_saving_throws(pool, gs, creature_id).await;
    let creature_ac = fetch_creature_ac(pool, gs, creature_id).await;

    Ok(CreatureCombatData {
        weapons: weapons.unwrap_or_default(),
        armors: armors.unwrap_or_default(),
        shields: shields.unwrap_or_default(),
        resistances: resistances.unwrap_or_default(),
        immunities: immunities
            .unwrap_or_default()
            .iter()
            .flatten()
            .cloned()
            .collect(),
        weaknesses: weaknesses
            .unwrap_or_default()
            .iter()
            .map(|x| (x.name.clone(), i16::try_from(x.value).unwrap_or(0)))
            .collect(),
        saving_throws: saving_throws?,
        ac: creature_ac?,
    })
}

pub async fn fetch_creature_spellcaster_data(
    pool: &PgPool,
    gs: GameSystem,
    creature_id: i64,
) -> Result<CreatureSpellcasterData> {
    Ok(CreatureSpellcasterData {
        spellcaster_entries: fetch_creature_spellcaster_entries(pool, gs, creature_id).await?,
    })
}

pub async fn fetch_creature_scales(pool: &PgPool) -> Result<CreatureScales> {
    Ok(CreatureScales {
        ability_scales: sqlx::query_as::<_, AbilityScales>("SELECT * FROM ability_scales_table")
            .fetch_all(pool)
            .await?
            .into_iter()
            .map(|n| (n.level, n))
            .collect(),
        ac_scales: sqlx::query_as::<_, AcScales>("SELECT * FROM ac_scales_table")
            .fetch_all(pool)
            .await?
            .into_iter()
            .map(|n| (n.level, n))
            .collect(),
        area_dmg_scales: sqlx::query_as::<_, AreaDmgScales>(
            "SELECT * FROM area_damage_scales_table",
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|n| (n.level, n))
        .collect(),
        hp_scales: sqlx::query_as::<_, HpScales>("SELECT * FROM hp_scales_table")
            .fetch_all(pool)
            .await?
            .into_iter()
            .map(|n| (n.level, n))
            .collect(),
        item_scales: sqlx::query_as::<_, ItemScales>("SELECT * FROM item_scales_table")
            .fetch_all(pool)
            .await?
            .into_iter()
            .map(|n| (n.cr_level.clone(), n))
            .collect(),
        perception_scales: sqlx::query_as::<_, PerceptionScales>(
            "SELECT * FROM PERCEPTION_SCALES_TABLE",
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|n| (n.level, n))
        .collect(),
        res_weak_scales: sqlx::query_as::<_, ResWeakScales>("SELECT * FROM res_weak_scales_table")
            .fetch_all(pool)
            .await?
            .into_iter()
            .map(|n| (n.level, n))
            .collect(),
        saving_throw_scales: sqlx::query_as::<_, SavingThrowScales>(
            "SELECT * FROM SAVING_THROW_SCALES_TABLE",
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|n| (n.level, n))
        .collect(),
        skill_scales: sqlx::query_as::<_, SkillScales>("SELECT * FROM skill_scales_table")
            .fetch_all(pool)
            .await?
            .into_iter()
            .map(|n| (n.level, n))
            .collect(),
        spell_dc_and_atk_scales: sqlx::query_as::<_, SpellDcAndAtkScales>(
            "SELECT * FROM spell_dc_and_attack_scales_table",
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|n| (n.level, n))
        .collect(),
        strike_bonus_scales: sqlx::query_as::<_, StrikeBonusScales>(
            "SELECT * FROM strike_bonus_scales_table",
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|n| (n.level, n))
        .collect(),
        strike_dmg_scales: sqlx::query_as::<_, StrikeDmgScales>(
            "SELECT * FROM strike_damage_scales_table",
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|n| (n.level, n))
        .collect(),
    })
}
