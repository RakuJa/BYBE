use crate::db::data_providers::raw_query_builder::prepare_filtered_get_creatures_core;
use crate::models::creature::Creature;
use crate::models::creature_component::creature_combat::{CreatureCombatData, SavingThrows};
use crate::models::creature_component::creature_core::{
    CreatureCoreData, DerivedData, EssentialData,
};
use crate::models::creature_component::creature_extra::{AbilityScores, CreatureExtraData};
use crate::models::creature_component::creature_spell_caster::CreatureSpellCasterData;
use crate::models::creature_component::creature_variant::CreatureVariantData;
use crate::models::creature_filter_enum::CreatureFilter;
use crate::models::creature_metadata::alignment_enum::{AlignmentEnum, ALIGNMENT_TRAITS};
use crate::models::creature_metadata::variant_enum::CreatureVariant;
use crate::models::db::raw_immunity::RawImmunity;
use crate::models::db::raw_language::RawLanguage;
use crate::models::db::raw_resistance::RawResistance;
use crate::models::db::raw_sense::RawSense;
use crate::models::db::raw_speed::RawSpeed;
use crate::models::db::raw_trait::RawTrait;
use crate::models::db::raw_weakness::RawWeakness;
use crate::models::items::action::Action;
use crate::models::items::skill::Skill;
use crate::models::items::spell::Spell;
use crate::models::items::spell_caster_entry::SpellCasterEntry;
use crate::models::items::weapon::Weapon;
use crate::models::response_data::OptionalData;
use crate::models::routers_validator_structs::PaginatedRequest;
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
use anyhow::Result;
use sqlx::{FromRow, Pool, Sqlite};
use std::collections::{HashMap, HashSet};

async fn fetch_creature_immunities(
    conn: &Pool<Sqlite>,
    creature_id: i64,
) -> Result<Vec<RawImmunity>> {
    Ok(sqlx::query_as!(
        RawImmunity,
        "SELECT * FROM IMMUNITY_TABLE INTERSECT SELECT immunity_id FROM IMMUNITY_CREATURE_ASSOCIATION_TABLE WHERE creature_id == ($1)",
        creature_id
    ).fetch_all(conn).await?)
}

async fn fetch_creature_languages(
    conn: &Pool<Sqlite>,
    creature_id: i64,
) -> Result<Vec<RawLanguage>> {
    Ok(sqlx::query_as!(
        RawLanguage,
        "SELECT * FROM LANGUAGE_TABLE INTERSECT SELECT language_id FROM LANGUAGE_CREATURE_ASSOCIATION_TABLE WHERE creature_id == ($1)",
        creature_id
    ).fetch_all(conn).await?)
}

async fn fetch_creature_resistances(
    conn: &Pool<Sqlite>,
    creature_id: i64,
) -> Result<Vec<RawResistance>> {
    Ok(sqlx::query_as!(
        RawResistance,
        "SELECT name, value FROM RESISTANCE_TABLE WHERE creature_id == ($1)",
        creature_id
    )
    .fetch_all(conn)
    .await?)
}

async fn fetch_creature_senses(conn: &Pool<Sqlite>, creature_id: i64) -> Result<Vec<RawSense>> {
    Ok(sqlx::query_as!(
        RawSense,
        "SELECT * FROM SENSE_TABLE INTERSECT SELECT sense_id FROM SENSE_CREATURE_ASSOCIATION_TABLE WHERE creature_id == ($1)",
        creature_id
    ).fetch_all(conn).await?)
}

async fn fetch_creature_speeds(conn: &Pool<Sqlite>, creature_id: i64) -> Result<Vec<RawSpeed>> {
    Ok(sqlx::query_as!(
        RawSpeed,
        "SELECT name, value FROM SPEED_TABLE WHERE creature_id == ($1)",
        creature_id
    )
    .fetch_all(conn)
    .await?)
}

async fn fetch_creature_weaknesses(
    conn: &Pool<Sqlite>,
    creature_id: i64,
) -> Result<Vec<RawWeakness>> {
    Ok(sqlx::query_as!(
        RawWeakness,
        "SELECT name, value FROM WEAKNESS_TABLE WHERE creature_id == ($1)",
        creature_id
    )
    .fetch_all(conn)
    .await?)
}

async fn fetch_creature_saving_throws(
    conn: &Pool<Sqlite>,
    creature_id: i64,
) -> Result<SavingThrows> {
    Ok(
        sqlx::query_as!(
            SavingThrows,
            "SELECT fortitude, reflex, will, fortitude_detail, reflex_detail, will_detail FROM CREATURE_TABLE WHERE id == ($1)",
            creature_id
        ).fetch_one(conn).await?
    )
}

async fn fetch_creature_ability_scores(
    conn: &Pool<Sqlite>,
    creature_id: i64,
) -> Result<AbilityScores> {
    Ok(
        sqlx::query_as!(
            AbilityScores,
            "SELECT charisma, constitution, dexterity, intelligence, strength, wisdom FROM CREATURE_TABLE WHERE id == ($1)",
            creature_id
        ).fetch_one(conn).await?
    )
}

async fn fetch_creature_ac(conn: &Pool<Sqlite>, creature_id: i64) -> Result<i8> {
    Ok(
        sqlx::query_scalar("SELECT ac FROM CREATURE_TABLE WHERE id = $1")
            .bind(creature_id)
            .fetch_one(conn)
            .await?,
    )
}

async fn fetch_creature_ac_detail(conn: &Pool<Sqlite>, creature_id: i64) -> Result<Option<String>> {
    Ok(
        sqlx::query_scalar("SELECT ac_detail FROM CREATURE_TABLE WHERE id = $1 LIMIT 1")
            .bind(creature_id)
            .fetch_optional(conn)
            .await?,
    )
}

async fn fetch_creature_hp_detail(conn: &Pool<Sqlite>, creature_id: i64) -> Result<Option<String>> {
    Ok(
        sqlx::query_scalar("SELECT hp_detail FROM CREATURE_TABLE WHERE id = $1 LIMIT 1")
            .bind(creature_id)
            .fetch_optional(conn)
            .await?,
    )
}

async fn fetch_creature_language_detail(
    conn: &Pool<Sqlite>,
    creature_id: i64,
) -> Result<Option<String>> {
    Ok(
        sqlx::query_scalar("SELECT language_detail FROM CREATURE_TABLE WHERE id = $1 LIMIT 1")
            .bind(creature_id)
            .fetch_optional(conn)
            .await?,
    )
}

async fn fetch_creature_perception(conn: &Pool<Sqlite>, creature_id: i64) -> Result<i8> {
    Ok(
        sqlx::query_scalar("SELECT perception FROM CREATURE_TABLE WHERE id = $1 LIMIT 1")
            .bind(creature_id)
            .fetch_one(conn)
            .await?,
    )
}

async fn fetch_creature_perception_detail(
    conn: &Pool<Sqlite>,
    creature_id: i64,
) -> Result<Option<String>> {
    Ok(
        sqlx::query_scalar("SELECT perception_detail FROM CREATURE_TABLE WHERE id = $1 LIMIT 1")
            .bind(creature_id)
            .fetch_optional(conn)
            .await?,
    )
}

async fn fetch_creature_traits(conn: &Pool<Sqlite>, creature_id: i64) -> Result<Vec<RawTrait>> {
    Ok(sqlx::query_as!(
        RawTrait,
        "SELECT * FROM TRAIT_TABLE INTERSECT SELECT trait_id FROM TRAIT_CREATURE_ASSOCIATION_TABLE WHERE creature_id == ($1)",
        creature_id
    ).fetch_all(conn).await?)
}

async fn fetch_creature_weapons(conn: &Pool<Sqlite>, creature_id: i64) -> Result<Vec<Weapon>> {
    Ok(sqlx::query_as!(
        Weapon,
        "SELECT * FROM WEAPON_TABLE WHERE creature_id == ($1)",
        creature_id
    )
    .fetch_all(conn)
    .await?)
}

async fn fetch_creature_actions(conn: &Pool<Sqlite>, creature_id: i64) -> Result<Vec<Action>> {
    Ok(sqlx::query_as!(
        Action,
        "SELECT * FROM ACTION_TABLE WHERE creature_id == ($1)",
        creature_id
    )
    .fetch_all(conn)
    .await?)
}

async fn fetch_creature_skills(conn: &Pool<Sqlite>, creature_id: i64) -> Result<Vec<Skill>> {
    Ok(sqlx::query_as!(
        Skill,
        "SELECT name, description, modifier, proficiency FROM SKILL_TABLE WHERE creature_id == ($1)",
        creature_id
    )
    .fetch_all(conn)
    .await?)
}

async fn fetch_creature_spells(conn: &Pool<Sqlite>, creature_id: i64) -> Result<Vec<Spell>> {
    Ok(sqlx::query_as!(
        Spell,
        "SELECT * FROM SPELL_TABLE WHERE creature_id == ($1)",
        creature_id
    )
    .fetch_all(conn)
    .await?)
}

async fn fetch_creature_spell_caster_entry(
    conn: &Pool<Sqlite>,
    creature_id: i64,
) -> Result<SpellCasterEntry> {
    Ok(sqlx::query_as!(
        SpellCasterEntry,
        "SELECT spell_casting_name, is_spell_casting_flexible, type_of_spell_caster, spell_casting_dc_mod, spell_casting_atk_mod, spell_casting_tradition FROM CREATURE_TABLE WHERE id == ($1) LIMIT 1",
        creature_id
    ).fetch_one(conn).await?)
}

async fn fetch_creature_core_data(
    conn: &Pool<Sqlite>,
    creature_id: i64,
) -> Result<CreatureCoreData> {
    let essential = fetch_creature_essential_data(conn, creature_id).await?;
    let derived = fetch_creature_derived_data(conn, creature_id).await?;
    let traits = fetch_creature_traits(conn, creature_id)
        .await
        .unwrap_or_default();
    let is_remaster = essential.remaster;
    Ok(CreatureCoreData {
        essential,
        derived,
        traits: traits.iter().map(|x| x.name.clone()).collect(),
        alignment: AlignmentEnum::from((&traits, is_remaster)),
    })
}

async fn fetch_creature_essential_data(
    conn: &Pool<Sqlite>,
    creature_id: i64,
) -> Result<EssentialData> {
    Ok(sqlx::query_as!(
        EssentialData,
        "SELECT id, aon_id, name, hp, level, size, family, rarity,
            license, remaster, source, cr_type
        FROM CREATURE_CORE WHERE id = ? ORDER BY name LIMIT 1",
        creature_id,
    )
    .fetch_one(conn)
    .await?)
}

async fn fetch_creature_derived_data(conn: &Pool<Sqlite>, creature_id: i64) -> Result<DerivedData> {
    Ok(sqlx::query_as!(
        DerivedData,
        "SELECT
            archive_link, is_melee, is_ranged, is_spell_caster, brute_percentage,
            magical_striker_percentage, skill_paragon_percentage, skirmisher_percentage,
            sniper_percentage, soldier_percentage, spell_caster_percentage
         FROM CREATURE_CORE WHERE id = ? ORDER BY name LIMIT 1",
        creature_id,
    )
    .fetch_one(conn)
    .await?)
}

async fn update_creatures_core_with_traits(
    conn: &Pool<Sqlite>,
    mut creature_core_data: Vec<CreatureCoreData>,
) -> Vec<CreatureCoreData> {
    for core in &mut creature_core_data {
        let traits = fetch_creature_traits(conn, core.essential.id)
            .await
            .unwrap_or_default();
        let is_remaster = core.essential.remaster;
        core.traits = traits
            .iter()
            .filter(|x| !ALIGNMENT_TRAITS.contains(&&*x.name.as_str().to_uppercase()))
            .map(|x| x.name.clone())
            .collect();
        core.alignment = AlignmentEnum::from((&traits, is_remaster));
    }
    creature_core_data
}

#[derive(FromRow)]
struct MyString {
    my_str: String,
}

pub async fn fetch_unique_values_of_field(
    conn: &Pool<Sqlite>,
    table: &str,
    field: &str,
) -> Result<Vec<String>> {
    let query = format!(
        "SELECT CAST(t1.{field} AS TEXT) AS my_str FROM ((SELECT DISTINCT ({field}) FROM {table})) t1"
    );
    let x: Vec<MyString> = sqlx::query_as(query.as_str()).fetch_all(conn).await?;
    Ok(x.iter().map(|x| x.my_str.clone()).collect())
}

pub async fn fetch_traits_associated_with_creatures(conn: &Pool<Sqlite>) -> Result<Vec<String>> {
    let x: Vec<MyString> = sqlx::query_as(
        "
        SELECT
            tt.name AS my_str
        FROM TRAIT_CREATURE_ASSOCIATION_TABLE tcat
            LEFT JOIN TRAIT_TABLE tt ON tcat.trait_id = tt.name GROUP BY tt.name",
    )
    .fetch_all(conn)
    .await?;
    Ok(x.iter()
        .filter(|x| !ALIGNMENT_TRAITS.contains(&&*x.my_str.as_str().to_uppercase()))
        .map(|x| x.my_str.clone())
        .collect())
}

pub async fn fetch_creature_by_id(
    conn: &Pool<Sqlite>,
    optional_data: &OptionalData,
    id: i64,
) -> Result<Creature> {
    let core_data = fetch_creature_core_data(conn, id).await?;
    let level = core_data.essential.level;
    let archive_link = core_data.derived.archive_link.clone();
    Ok(Creature {
        core_data,
        variant_data: CreatureVariantData {
            variant: CreatureVariant::Base,
            level,
            archive_link,
        },
        extra_data: if optional_data.extra_data.is_some_and(|x| x) {
            Some(fetch_creature_extra_data(conn, id).await?)
        } else {
            None
        },
        combat_data: if optional_data.combat_data.is_some_and(|x| x) {
            Some(fetch_creature_combat_data(conn, id).await?)
        } else {
            None
        },
        spell_caster_data: if optional_data.spell_casting_data.is_some_and(|x| x) {
            Some(fetch_creature_spell_caster_data(conn, id).await?)
        } else {
            None
        },
    })
}

pub async fn fetch_creatures_core_data_with_filters(
    conn: &Pool<Sqlite>,
    key_value_filters: &HashMap<CreatureFilter, HashSet<String>>,
) -> Result<Vec<CreatureCoreData>> {
    let query = prepare_filtered_get_creatures_core(key_value_filters);
    let core_data: Vec<CreatureCoreData> = sqlx::query_as(query.as_str()).fetch_all(conn).await?;
    Ok(update_creatures_core_with_traits(conn, core_data).await)
}

/// Gets all the creatures core it can find with the given pagination as boundaries
/// for the search.
pub async fn fetch_creatures_core_data(
    conn: &Pool<Sqlite>,
    paginated_request: &PaginatedRequest,
) -> Result<Vec<CreatureCoreData>> {
    let cr_core: Vec<CreatureCoreData> =
        sqlx::query_as("SELECT * FROM CREATURE_CORE ORDER BY name LIMIT ?,?")
            .bind(paginated_request.cursor)
            .bind(paginated_request.page_size)
            .fetch_all(conn)
            .await?;
    Ok(update_creatures_core_with_traits(conn, cr_core).await)
}

pub async fn fetch_creature_extra_data(
    conn: &Pool<Sqlite>,
    creature_id: i64,
) -> Result<CreatureExtraData> {
    let actions = fetch_creature_actions(conn, creature_id)
        .await
        .unwrap_or_default();
    let skills = fetch_creature_skills(conn, creature_id)
        .await
        .unwrap_or_default();
    let languages = fetch_creature_languages(conn, creature_id)
        .await
        .unwrap_or_default();
    let senses = fetch_creature_senses(conn, creature_id)
        .await
        .unwrap_or_default();
    let speeds = fetch_creature_speeds(conn, creature_id)
        .await
        .unwrap_or_default();
    let ability_scores = fetch_creature_ability_scores(conn, creature_id).await?;
    let hp_detail = fetch_creature_hp_detail(conn, creature_id).await?;
    let ac_detail = fetch_creature_ac_detail(conn, creature_id).await?;
    let language_detail = fetch_creature_language_detail(conn, creature_id).await?;
    let perception = fetch_creature_perception(conn, creature_id).await?;
    let perception_detail = fetch_creature_perception_detail(conn, creature_id).await?;

    Ok(CreatureExtraData {
        actions,
        skills,
        languages: languages.iter().map(|x| x.name.clone()).collect(),
        senses: senses.iter().map(|x| x.name.clone()).collect(),
        speeds: speeds
            .iter()
            .map(|x| (x.name.clone(), x.value as i16))
            .collect(),
        ability_scores,
        hp_detail,
        ac_detail,
        language_detail,
        perception,
        perception_detail,
    })
}

pub async fn fetch_creature_combat_data(
    conn: &Pool<Sqlite>,
    creature_id: i64,
) -> Result<CreatureCombatData> {
    let weapons = fetch_creature_weapons(conn, creature_id)
        .await
        .unwrap_or_default();
    let resistances = fetch_creature_resistances(conn, creature_id)
        .await
        .unwrap_or_default();
    let immunities = fetch_creature_immunities(conn, creature_id)
        .await
        .unwrap_or_default();
    let weaknesses = fetch_creature_weaknesses(conn, creature_id)
        .await
        .unwrap_or_default();
    let saving_throws = fetch_creature_saving_throws(conn, creature_id).await?;
    let creature_ac = fetch_creature_ac(conn, creature_id).await?;
    Ok(CreatureCombatData {
        weapons,
        resistances: resistances
            .iter()
            .map(|x| (x.name.clone(), x.value as i16))
            .collect(),
        immunities: immunities.iter().map(|x| x.name.clone()).collect(),
        weaknesses: weaknesses
            .iter()
            .map(|x| (x.name.clone(), x.value as i16))
            .collect(),
        saving_throws,
        ac: creature_ac,
    })
}

pub async fn fetch_creature_spell_caster_data(
    conn: &Pool<Sqlite>,
    creature_id: i64,
) -> Result<CreatureSpellCasterData> {
    let spells = fetch_creature_spells(conn, creature_id).await?;
    let spell_caster_entry = fetch_creature_spell_caster_entry(conn, creature_id).await?;
    Ok(CreatureSpellCasterData {
        spells,
        spell_caster_entry,
    })
}

pub async fn fetch_creature_scales(conn: &Pool<Sqlite>) -> Result<CreatureScales> {
    Ok(CreatureScales {
        ability_scales: sqlx::query_as!(AbilityScales, "SELECT * FROM ABILITY_SCALES_TABLE",)
            .fetch_all(conn)
            .await?
            .into_iter()
            .map(|n| (n.level, n))
            .collect(),
        ac_scales: sqlx::query_as!(AcScales, "SELECT * FROM AC_SCALES_TABLE",)
            .fetch_all(conn)
            .await?
            .into_iter()
            .map(|n| (n.level, n))
            .collect(),
        area_dmg_scales: sqlx::query_as!(AreaDmgScales, "SELECT * FROM AREA_DAMAGE_SCALES_TABLE",)
            .fetch_all(conn)
            .await?
            .into_iter()
            .map(|n| (n.level, n))
            .collect(),
        hp_scales: sqlx::query_as!(HpScales, "SELECT * FROM HP_SCALES_TABLE",)
            .fetch_all(conn)
            .await?
            .into_iter()
            .map(|n| (n.level, n))
            .collect(),
        item_scales: sqlx::query_as!(ItemScales, "SELECT * FROM ITEM_SCALES_TABLE",)
            .fetch_all(conn)
            .await?
            .into_iter()
            .map(|n| (n.cr_level.clone(), n))
            .collect(),
        perception_scales: sqlx::query_as!(
            PerceptionScales,
            "SELECT * FROM PERCEPTION_SCALES_TABLE",
        )
        .fetch_all(conn)
        .await?
        .into_iter()
        .map(|n| (n.level, n))
        .collect(),
        res_weak_scales: sqlx::query_as!(ResWeakScales, "SELECT * FROM RES_WEAK_SCALES_TABLE",)
            .fetch_all(conn)
            .await?
            .into_iter()
            .map(|n| (n.level, n))
            .collect(),
        saving_throw_scales: sqlx::query_as!(
            SavingThrowScales,
            "SELECT * FROM SAVING_THROW_SCALES_TABLE",
        )
        .fetch_all(conn)
        .await?
        .into_iter()
        .map(|n| (n.level, n))
        .collect(),
        skill_scales: sqlx::query_as!(SkillScales, "SELECT * FROM SKILL_SCALES_TABLE",)
            .fetch_all(conn)
            .await?
            .into_iter()
            .map(|n| (n.level, n))
            .collect(),
        spell_dc_and_atk_scales: sqlx::query_as!(
            SpellDcAndAtkScales,
            "SELECT * FROM SPELL_DC_AND_ATTACK_SCALES_TABLE",
        )
        .fetch_all(conn)
        .await?
        .into_iter()
        .map(|n| (n.level, n))
        .collect(),
        strike_bonus_scales: sqlx::query_as!(
            StrikeBonusScales,
            "SELECT * FROM STRIKE_BONUS_SCALES_TABLE",
        )
        .fetch_all(conn)
        .await?
        .into_iter()
        .map(|n| (n.level, n))
        .collect(),
        strike_dmg_scales: sqlx::query_as!(
            StrikeDmgScales,
            "SELECT * FROM STRIKE_DAMAGE_SCALES_TABLE",
        )
        .fetch_all(conn)
        .await?
        .into_iter()
        .map(|n| (n.level, n))
        .collect(),
    })
}
