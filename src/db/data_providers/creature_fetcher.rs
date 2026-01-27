use crate::db::data_providers::generic_fetcher::{
    fetch_armor_runes, fetch_armor_traits, fetch_item_traits, fetch_shield_traits,
    fetch_weapon_damage_data, fetch_weapon_runes, fetch_weapon_traits,
};
use crate::db::data_providers::raw_query_builder::prepare_filtered_get_creatures_core;
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
use crate::models::creature::creature_metadata::alignment_enum::ALIGNMENT_TRAITS;
use crate::models::creature::creature_metadata::variant_enum::CreatureVariant;
use crate::models::creature::creature_struct::Creature;
use crate::models::creature::items::action::Action;
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
use crate::models::response_data::ResponseDataModifiers;
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
use crate::models::shared::game_system_enum::GameSystem;
use anyhow::Result;
use futures::future::join_all;
use sqlx::{Pool, Sqlite};

async fn fetch_creature_immunities(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    creature_id: i64,
) -> Result<Vec<String>> {
    let query = match gs {
        GameSystem::Pathfinder => {
            sqlx::query_scalar!(
                "SELECT name FROM pf_immunity_table INTERSECT SELECT immunity_id
         FROM pf_immunity_creature_association_table WHERE creature_id == ($1)",
                creature_id
            )
        }
        GameSystem::Starfinder => {
            sqlx::query_scalar!(
                "SELECT name FROM sf_immunity_table INTERSECT SELECT immunity_id
         FROM sf_immunity_creature_association_table WHERE creature_id == ($1)",
                creature_id
            )
        }
    };
    Ok(query.fetch_all(conn).await?)
}

async fn fetch_creature_languages(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    creature_id: i64,
) -> Result<Vec<RawLanguage>> {
    Ok(match gs {
        GameSystem::Pathfinder => {
            sqlx::query_as!(
                RawLanguage,
                "SELECT * FROM pf_language_table INTERSECT SELECT language_id
                 FROM pf_language_creature_association_table WHERE creature_id == ($1)",
                creature_id
            )
            .fetch_all(conn)
            .await?
        }
        GameSystem::Starfinder => {
            sqlx::query_as!(
                RawLanguage,
                "SELECT * FROM sf_language_table INTERSECT SELECT language_id
                 FROM sf_language_creature_association_table WHERE creature_id == ($1)",
                creature_id
            )
            .fetch_all(conn)
            .await?
        }
    })
}

async fn fetch_creature_resistances(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    creature_id: i64,
) -> Result<Vec<Resistance>> {
    Ok(join_all(
        fetch_creature_resistances_core(conn, gs, creature_id)
            .await?
            .iter()
            .map(async |x| {
                let (double_vs, exception_vs) = fetch_creature_resistances_vs(conn, gs, x.id)
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
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    creature_id: i64,
) -> Result<Vec<CoreResistanceData>> {
    Ok(match gs {
        GameSystem::Pathfinder => {
            sqlx::query_as!(
                CoreResistanceData,
                "SELECT id, name, value FROM pf_resistance_table WHERE creature_id == ($1)",
                creature_id
            )
            .fetch_all(conn)
            .await?
        }
        GameSystem::Starfinder => {
            sqlx::query_as!(
                CoreResistanceData,
                "SELECT id, name, value FROM sf_resistance_table WHERE creature_id == ($1)",
                creature_id
            )
            .fetch_all(conn)
            .await?
        }
    })
}

async fn fetch_creature_resistances_vs(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    res_id: i64,
) -> Result<(Vec<String>, Vec<String>)> {
    Ok((
        match gs {
            GameSystem::Pathfinder => {
                sqlx::query_scalar!(
                    "SELECT vs_name FROM pf_resistance_double_vs_table WHERE resistance_id = ($1)",
                    res_id
                )
            } GameSystem::Starfinder => {
                sqlx::query_scalar!(
                    "SELECT vs_name FROM sf_resistance_double_vs_table WHERE resistance_id = ($1)",
                    res_id
                )
            }
        }
        .fetch_all(conn)
        .await?,
        match gs {
            GameSystem::Pathfinder => {
                sqlx::query_scalar!(
                    "SELECT vs_name FROM pf_resistance_exception_vs_table WHERE resistance_id = ($1)",
                    res_id
                )
            },
            GameSystem::Starfinder => {
                sqlx::query_scalar!(
                    "SELECT vs_name FROM sf_resistance_exception_vs_table WHERE resistance_id = ($1)",
                    res_id
                )
            }
        }
        .fetch_all(conn)
        .await?,
    ))
}

async fn fetch_creature_senses(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    creature_id: i64,
) -> Result<Vec<Sense>> {
    Ok(match gs {
        GameSystem::Pathfinder => {
            sqlx::query_as!(
                Sense,
                "SELECT * FROM pf_sense_table WHERE id IN (SELECT sense_id
                 FROM pf_sense_creature_association_table WHERE creature_id == ($1))",
                creature_id
            )
            .fetch_all(conn)
            .await?
        }
        GameSystem::Starfinder => {
            sqlx::query_as!(
                Sense,
                "SELECT * FROM sf_sense_table WHERE id IN (SELECT sense_id
                 FROM sf_sense_creature_association_table WHERE creature_id == ($1))",
                creature_id
            )
            .fetch_all(conn)
            .await?
        }
    })
}

async fn fetch_creature_speeds(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    creature_id: i64,
) -> Result<Vec<RawSpeed>> {
    Ok(match gs {
        GameSystem::Pathfinder => {
            sqlx::query_as!(
                RawSpeed,
                "SELECT name, value FROM pf_speed_table WHERE creature_id == ($1)",
                creature_id
            )
            .fetch_all(conn)
            .await?
        }
        GameSystem::Starfinder => {
            sqlx::query_as!(
                RawSpeed,
                "SELECT name, value FROM sf_speed_table WHERE creature_id == ($1)",
                creature_id
            )
            .fetch_all(conn)
            .await?
        }
    })
}

async fn fetch_creature_weaknesses(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    creature_id: i64,
) -> Result<Vec<RawWeakness>> {
    Ok(match gs {
        GameSystem::Pathfinder => {
            sqlx::query_as!(
                RawWeakness,
                "SELECT name, value FROM pf_weakness_table WHERE creature_id == ($1)",
                creature_id
            )
            .fetch_all(conn)
            .await?
        }
        GameSystem::Starfinder => {
            sqlx::query_as!(
                RawWeakness,
                "SELECT name, value FROM sf_weakness_table WHERE creature_id == ($1)",
                creature_id
            )
            .fetch_all(conn)
            .await?
        }
    })
}

async fn fetch_creature_saving_throws(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    creature_id: i64,
) -> Result<SavingThrows> {
    Ok(match gs {
        GameSystem::Pathfinder => {
            sqlx::query_as!(
                SavingThrows,
                "SELECT fortitude, reflex, will, fortitude_detail, reflex_detail, will_detail
                 FROM pf_creature_table WHERE id == ($1)",
                creature_id
            )
            .fetch_one(conn)
            .await?
        }
        GameSystem::Starfinder => {
            sqlx::query_as!(
                SavingThrows,
                "SELECT fortitude, reflex, will, fortitude_detail, reflex_detail, will_detail
                 FROM sf_creature_table WHERE id == ($1)",
                creature_id
            )
            .fetch_one(conn)
            .await?
        }
    })
}

async fn fetch_creature_ability_scores(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    creature_id: i64,
) -> Result<AbilityScores> {
    Ok(match gs {
        GameSystem::Pathfinder => {
            sqlx::query_as!(
                AbilityScores,
                "SELECT charisma, constitution, dexterity, intelligence, strength, wisdom
                 FROM pf_creature_table WHERE id == ($1)",
                creature_id
            )
            .fetch_one(conn)
            .await?
        }
        GameSystem::Starfinder => {
            sqlx::query_as!(
                AbilityScores,
                "SELECT charisma, constitution, dexterity, intelligence, strength, wisdom
                 FROM sf_creature_table WHERE id == ($1)",
                creature_id
            )
            .fetch_one(conn)
            .await?
        }
    })
}

async fn fetch_creature_ac(conn: &Pool<Sqlite>, gs: &GameSystem, creature_id: i64) -> Result<i8> {
    Ok(sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
        "SELECT ac FROM {gs}_creature_table WHERE id = $1"
    )))
    .bind(creature_id)
    .fetch_one(conn)
    .await?)
}

async fn fetch_creature_ac_detail(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    creature_id: i64,
) -> Result<Option<String>> {
    Ok(sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
        "SELECT ac_detail FROM {gs}_creature_table WHERE id = $1 LIMIT 1"
    )))
    .bind(creature_id)
    .fetch_optional(conn)
    .await?)
}

async fn fetch_creature_hp_detail(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    creature_id: i64,
) -> Result<Option<String>> {
    Ok(sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
        "SELECT hp_detail FROM {gs}_creature_table WHERE id = $1 LIMIT 1"
    )))
    .bind(creature_id)
    .fetch_optional(conn)
    .await?)
}

async fn fetch_creature_language_detail(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    creature_id: i64,
) -> Result<Option<String>> {
    Ok(sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
        "SELECT language_detail FROM {gs}_creature_table WHERE id = $1 LIMIT 1"
    )))
    .bind(creature_id)
    .fetch_optional(conn)
    .await?)
}

async fn fetch_creature_perception(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    creature_id: i64,
) -> Result<i8> {
    Ok(sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
        "SELECT perception FROM {gs}_creature_table WHERE id = $1 LIMIT 1"
    )))
    .bind(creature_id)
    .fetch_one(conn)
    .await?)
}

async fn fetch_creature_vision(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    creature_id: i64,
) -> Result<bool> {
    Ok(sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
        "SELECT vision FROM {gs}_creature_table WHERE id = $1 LIMIT 1"
    )))
    .bind(creature_id)
    .fetch_one(conn)
    .await?)
}

async fn fetch_creature_perception_detail(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    creature_id: i64,
) -> Result<Option<String>> {
    Ok(sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
        "SELECT perception_detail FROM {gs}_creature_table WHERE id = $1 LIMIT 1"
    )))
    .bind(creature_id)
    .fetch_optional(conn)
    .await?)
}

pub async fn fetch_creature_traits(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    creature_id: i64,
) -> Result<Vec<String>> {
    Ok(sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
        "SELECT name FROM {gs}_trait_table INTERSECT SELECT trait_id
             FROM {gs}_trait_creature_association_table WHERE creature_id == ($1)"
    )))
    .bind(creature_id)
    .fetch_all(conn)
    .await?)
}

async fn fetch_creature_weapons(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
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
        WHERE ica.creature_id = ($1)
        GROUP BY ica.weapon_id
        ORDER BY name
        "
    )))
    .bind(creature_id)
    .fetch_all(conn)
    .await?;
    let mut result_vec = Vec::new();
    for mut el in weapons {
        el.item_core.traits = fetch_weapon_traits(conn, gs, el.weapon_data.id)
            .await
            .unwrap_or(vec![]);
        el.item_core.quantity =
            fetch_weapon_quantity(conn, gs, creature_id, el.weapon_data.id).await;
        el.weapon_data.property_runes = fetch_weapon_runes(conn, gs, el.weapon_data.id)
            .await
            .unwrap_or(vec![]);
        el.weapon_data.damage_data = fetch_weapon_damage_data(conn, gs, el.weapon_data.id)
            .await
            .unwrap_or(vec![]);
        result_vec.push(el);
    }
    Ok(result_vec)
}

async fn fetch_creature_armors(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
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
        WHERE aca.creature_id = ($1)
        GROUP BY aca.armor_id
        ORDER BY name
        "
    )))
    .bind(creature_id)
    .fetch_all(conn)
    .await?;
    let mut result_vec = Vec::new();
    for mut el in armors {
        el.item_core.traits = fetch_armor_traits(conn, gs, el.armor_data.id)
            .await
            .unwrap_or(vec![]);
        el.item_core.quantity = fetch_armor_quantity(conn, gs, creature_id, el.armor_data.id).await;
        el.armor_data.property_runes = fetch_armor_runes(conn, gs, el.armor_data.id)
            .await
            .unwrap_or(vec![]);
        result_vec.push(el);
    }
    Ok(result_vec)
}

async fn fetch_creature_shields(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    creature_id: i64,
) -> Result<Vec<Shield>> {
    let shields: Vec<Shield> = sqlx::query_as(sqlx::AssertSqlSafe(format!(
        "
        SELECT st.id AS shield_id, st.bonus_ac, st.n_of_reinforcing_runes, st.speed_penalty,
        it.*
        FROM {gs}_shield_creature_association_table sca
        LEFT JOIN {gs}_shield_table st ON st.id = sca.shield_id
        LEFT JOIN {gs}_item_table it ON it.id = st.base_item_id
        WHERE sca.creature_id = ($1)
        GROUP BY sca.shield_id
        ORDER BY name
        ",
    )))
    .bind(creature_id)
    .fetch_all(conn)
    .await?;
    let mut result_vec = Vec::new();
    for mut el in shields {
        el.item_core.traits = fetch_shield_traits(conn, gs, el.shield_data.id)
            .await
            .unwrap_or(vec![]);
        el.item_core.quantity =
            fetch_shield_quantity(conn, gs, creature_id, el.shield_data.id).await;
        result_vec.push(el);
    }
    Ok(result_vec)
}

async fn fetch_creature_items(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    creature_id: i64,
) -> Result<Vec<Item>> {
    let items: Vec<Item> = sqlx::query_as(sqlx::AssertSqlSafe(format!(
        "SELECT * FROM {gs}_item_table WHERE id IN (
            SELECT item_id FROM {gs}_item_creature_association_table WHERE creature_id == ($1)
            )
            AND id NOT IN (SELECT base_item_id FROM {gs}_armor_table)
            AND id NOT IN (SELECT base_item_id FROM {gs}_weapon_table)
            AND id NOT IN (SELECT base_item_id FROM {gs}_shield_table)
            "
    )))
    .bind(creature_id)
    .fetch_all(conn)
    .await?;
    let mut result_vec = Vec::new();
    for mut el in items {
        el.traits = fetch_item_traits(conn, gs, el.id).await.unwrap_or(vec![]);
        el.quantity = fetch_item_quantity(conn, gs, creature_id, el.id).await;
        result_vec.push(el);
    }
    Ok(result_vec)
}
/// Quantities are present ONLY for creature's item.
/// It needs to be fetched from the association table.
/// It defaults to 1 if error are found
async fn fetch_item_quantity(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    creature_id: i64,
    item_id: i64,
) -> i64 {
    match gs {
        GameSystem::Pathfinder => sqlx::query!(
            "SELECT quantity FROM pf_item_creature_association_table WHERE
                creature_id == ($1) AND item_id == ($2)",
            creature_id,
            item_id
        )
        .fetch_one(conn)
        .await
        .map_or(1, |q| q.quantity),
        GameSystem::Starfinder => sqlx::query!(
            "SELECT quantity FROM sf_item_creature_association_table WHERE
                creature_id == ($1) AND item_id == ($2)",
            creature_id,
            item_id
        )
        .fetch_one(conn)
        .await
        .map_or(1, |q| q.quantity),
    }
}

/// Quantities are present ONLY for creature's weapons.
/// It needs to be fetched from the association table.
/// It defaults to 1 if error are found
async fn fetch_weapon_quantity(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    creature_id: i64,
    weapon_id: i64,
) -> i64 {
    match gs {
        GameSystem::Pathfinder => sqlx::query!(
            "SELECT quantity FROM pf_weapon_creature_association_table WHERE
                creature_id == ($1) AND weapon_id == ($2)",
            creature_id,
            weapon_id
        )
        .fetch_one(conn)
        .await
        .map_or(1, |r| r.quantity),
        GameSystem::Starfinder => sqlx::query!(
            "SELECT quantity FROM sf_weapon_creature_association_table WHERE
                creature_id == ($1) AND weapon_id == ($2)",
            creature_id,
            weapon_id
        )
        .fetch_one(conn)
        .await
        .map_or(1, |r| r.quantity),
    }
}

/// Quantities are present ONLY for creature's shields.
/// It needs to be fetched from the association table.
/// It defaults to 1 if error are found
async fn fetch_shield_quantity(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    creature_id: i64,
    shield_id: i64,
) -> i64 {
    match gs {
        GameSystem::Pathfinder => sqlx::query!(
            "SELECT quantity FROM pf_shield_creature_association_table WHERE
                creature_id == ($1) AND shield_id == ($2)",
            creature_id,
            shield_id
        )
        .fetch_one(conn)
        .await
        .map_or(1, |r| r.quantity),
        GameSystem::Starfinder => sqlx::query!(
            "SELECT quantity FROM sf_shield_creature_association_table WHERE
                creature_id == ($1) AND shield_id == ($2)",
            creature_id,
            shield_id
        )
        .fetch_one(conn)
        .await
        .map_or(1, |r| r.quantity),
    }
}

/// Quantities are present ONLY for creature's armors.
/// It needs to be fetched from the association table.
/// It defaults to 1 if error are found
async fn fetch_armor_quantity(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    creature_id: i64,
    armor_id: i64,
) -> i64 {
    match gs {
        GameSystem::Pathfinder => sqlx::query!(
            "SELECT quantity FROM pf_armor_creature_association_table WHERE
                creature_id == ($1) AND armor_id == ($2)",
            creature_id,
            armor_id
        )
        .fetch_one(conn)
        .await
        .map_or(1, |r| r.quantity),
        GameSystem::Starfinder => sqlx::query!(
            "SELECT quantity FROM sf_armor_creature_association_table WHERE
                creature_id == ($1) AND armor_id == ($2)",
            creature_id,
            armor_id
        )
        .fetch_one(conn)
        .await
        .map_or(1, |r| r.quantity),
    }
}

async fn fetch_creature_actions(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    creature_id: i64,
) -> Result<Vec<Action>> {
    Ok(match gs {
        GameSystem::Pathfinder => {
            sqlx::query_as!(
                Action,
                "SELECT * FROM pf_action_table WHERE creature_id == ($1)",
                creature_id
            )
            .fetch_all(conn)
            .await?
        }
        GameSystem::Starfinder => {
            sqlx::query_as!(
                Action,
                "SELECT * FROM sf_action_table WHERE creature_id == ($1)",
                creature_id
            )
            .fetch_all(conn)
            .await?
        }
    })
}

async fn fetch_creature_skills(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    creature_id: i64,
) -> Result<Vec<Skill>> {
    Ok(match gs {
        GameSystem::Pathfinder => {
            sqlx::query_as!(
                Skill,
                "SELECT name, description, modifier, proficiency FROM pf_skill_table WHERE creature_id == ($1)",
                creature_id
            ).fetch_all(conn).await?
        }
        GameSystem::Starfinder => {
            sqlx::query_as!(
                Skill,
                "SELECT name, description, modifier, proficiency FROM sf_skill_table WHERE creature_id == ($1)",
                creature_id
            ).fetch_all(conn).await?
        }
    })
}

pub async fn fetch_creature_spells(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    creature_id: i64,
    spellcaster_entry_id: i64,
) -> Result<Vec<Spell>> {
    Ok(match gs {
        GameSystem::Pathfinder => {
            sqlx::query_as!(
                Spell,
                "SELECT * FROM pf_spell_table WHERE creature_id == ($1) AND spellcasting_entry_id == ($2)",
                creature_id,
                spellcaster_entry_id
            ).fetch_all(conn).await?
        }
        GameSystem::Starfinder => {
            sqlx::query_as!(
                Spell,
                "SELECT * FROM pf_spell_table WHERE creature_id == ($1) AND spellcasting_entry_id == ($2)",
                creature_id,
                spellcaster_entry_id
            ).fetch_all(conn).await?
        }
    })
}

async fn fetch_creature_spellcaster_entries(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    creature_id: i64,
) -> Result<Vec<SpellcasterEntry>> {
    let mut result = Vec::new();
    let data = match gs {
        GameSystem::Pathfinder => {
            sqlx::query_as!(
                SpellcasterData,
                "SELECT
                id, spellcasting_name, is_spellcasting_flexible, type_of_spellcaster,
                spellcasting_dc_mod, spellcasting_atk_mod, spellcasting_tradition, heighten_level
                FROM pf_spellcasting_entry_table WHERE creature_id == ($1)",
                creature_id
            )
            .fetch_all(conn)
            .await?
        }
        GameSystem::Starfinder => {
            sqlx::query_as!(
                SpellcasterData,
                "SELECT
                id, spellcasting_name, is_spellcasting_flexible, type_of_spellcaster,
                spellcasting_dc_mod, spellcasting_atk_mod, spellcasting_tradition, heighten_level
                FROM sf_spellcasting_entry_table WHERE creature_id == ($1)",
                creature_id
            )
            .fetch_all(conn)
            .await?
        }
    };
    for sce in data {
        let sce_id = sce.id;
        result.push(SpellcasterEntry {
            spellcaster_data: sce,
            spells: fetch_creature_spells(conn, gs, creature_id, sce_id)
                .await
                .unwrap_or_default(),
        });
    }
    Ok(result)
}

async fn fetch_creature_core_data(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    creature_id: i64,
) -> Result<CreatureCoreData> {
    let mut cr_core: CreatureCoreData = sqlx::query_as(sqlx::AssertSqlSafe(format!(
        "SELECT * FROM {gs}_creature_core WHERE id = ? ORDER BY name LIMIT 1"
    )))
    .bind(creature_id)
    .fetch_one(conn)
    .await?;
    cr_core.traits = fetch_creature_traits(conn, gs, creature_id)
        .await
        .unwrap_or_default()
        .iter()
        .filter(|x| !ALIGNMENT_TRAITS.contains(&&*x.as_str().to_uppercase()))
        .cloned()
        .collect();
    Ok(cr_core)
}

async fn update_creatures_core_with_traits(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    mut creature_core_data: Vec<CreatureCoreData>,
) -> Vec<CreatureCoreData> {
    for core in &mut creature_core_data {
        core.traits = fetch_creature_traits(conn, gs, core.essential.id)
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
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
) -> Result<Vec<String>> {
    Ok(sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
        "
        SELECT
            tt.name
        FROM {gs}_trait_creature_association_table tcat
            LEFT JOIN {gs}_trait_table tt ON tcat.trait_id = tt.name GROUP BY tt.name",
    )))
    .fetch_all(conn)
    .await?
    .iter()
    .filter(|x: &&String| !ALIGNMENT_TRAITS.contains(&&*x.to_uppercase()))
    .cloned()
    .collect())
}

pub async fn fetch_creature_by_id(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    variant: CreatureVariant,
    response_data_mods: &ResponseDataModifiers,
    id: i64,
) -> Result<Creature> {
    let core_data = fetch_creature_core_data(conn, gs, id).await?;
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
            Some(fetch_creature_extra_data(conn, gs, id).await?)
        } else {
            None
        },
        combat_data: if response_data_mods.combat_data.is_some_and(|x| x) {
            Some(fetch_creature_combat_data(conn, gs, id).await?)
        } else {
            None
        },
        spellcaster_data: if response_data_mods.spellcasting_data.is_some_and(|x| x) {
            Some(fetch_creature_spellcaster_data(conn, gs, id).await?)
        } else {
            None
        },
        game_system: *gs,
    }
    .convert_creature_to_variant(variant);
    Ok(if response_data_mods.is_pwl_on.unwrap_or(false) {
        cr.convert_creature_to_pwl()
    } else {
        cr
    })
}

pub async fn fetch_creatures_core_data_with_filters(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    bestiary_filter_query: &BestiaryFilterQuery,
) -> Result<Vec<CreatureCoreData>> {
    let query = prepare_filtered_get_creatures_core(gs, bestiary_filter_query);
    let core_data: Vec<CreatureCoreData> = sqlx::query_as(sqlx::AssertSqlSafe(query))
        .fetch_all(conn)
        .await?;
    Ok(update_creatures_core_with_traits(conn, gs, core_data).await)
}

/// Gets all the creatures core it can find with the given pagination as boundaries
/// for the search.
pub async fn fetch_creatures_core_data(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    cursor: u32,
    page_size: i16,
) -> Result<Vec<CreatureCoreData>> {
    let cr_core: Vec<CreatureCoreData> = sqlx::query_as(sqlx::AssertSqlSafe(format!(
        "SELECT * FROM {gs}_creature_core ORDER BY name LIMIT ?,?"
    )))
    .bind(cursor)
    .bind(page_size)
    .fetch_all(conn)
    .await?;
    Ok(update_creatures_core_with_traits(conn, gs, cr_core).await)
}

pub async fn fetch_creature_extra_data(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    creature_id: i64,
) -> Result<CreatureExtraData> {
    let items = fetch_creature_items(conn, gs, creature_id)
        .await
        .unwrap_or_default();
    let actions = fetch_creature_actions(conn, gs, creature_id)
        .await
        .unwrap_or_default();
    let skills = fetch_creature_skills(conn, gs, creature_id)
        .await
        .unwrap_or_default();
    let languages = fetch_creature_languages(conn, gs, creature_id)
        .await
        .unwrap_or_default();
    let senses = fetch_creature_senses(conn, gs, creature_id)
        .await
        .unwrap_or_default();
    let speeds = fetch_creature_speeds(conn, gs, creature_id)
        .await
        .unwrap_or_default();
    let ability_scores = fetch_creature_ability_scores(conn, gs, creature_id).await?;
    let hp_detail = fetch_creature_hp_detail(conn, gs, creature_id).await?;
    let ac_detail = fetch_creature_ac_detail(conn, gs, creature_id).await?;
    let language_detail = fetch_creature_language_detail(conn, gs, creature_id).await?;
    let perception = fetch_creature_perception(conn, gs, creature_id).await?;
    let has_vision = fetch_creature_vision(conn, gs, creature_id).await?;
    let perception_detail = fetch_creature_perception_detail(conn, gs, creature_id).await?;

    Ok(CreatureExtraData {
        actions,
        skills,
        items,
        languages: languages.iter().map(|x| x.name.clone()).collect(),
        senses,
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
        has_vision,
    })
}

pub async fn fetch_creature_combat_data(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    creature_id: i64,
) -> Result<CreatureCombatData> {
    let weapons = fetch_creature_weapons(conn, gs, creature_id)
        .await
        .unwrap_or_default();
    let armors = fetch_creature_armors(conn, gs, creature_id)
        .await
        .unwrap_or_default();
    let shields = fetch_creature_shields(conn, gs, creature_id)
        .await
        .unwrap_or_default();
    let resistances = fetch_creature_resistances(conn, gs, creature_id)
        .await
        .unwrap_or_default();
    let immunities = fetch_creature_immunities(conn, gs, creature_id)
        .await
        .unwrap_or_default();
    let weaknesses = fetch_creature_weaknesses(conn, gs, creature_id)
        .await
        .unwrap_or_default();
    let saving_throws = fetch_creature_saving_throws(conn, gs, creature_id).await?;
    let creature_ac = fetch_creature_ac(conn, gs, creature_id).await?;
    Ok(CreatureCombatData {
        weapons,
        armors,
        shields,
        resistances,
        immunities,
        weaknesses: weaknesses
            .iter()
            .map(|x| (x.name.clone(), i16::try_from(x.value).unwrap_or(0)))
            .collect(),
        saving_throws,
        ac: creature_ac,
    })
}

pub async fn fetch_creature_spellcaster_data(
    conn: &Pool<Sqlite>,
    gs: &GameSystem,
    creature_id: i64,
) -> Result<CreatureSpellcasterData> {
    Ok(CreatureSpellcasterData {
        spellcaster_entries: fetch_creature_spellcaster_entries(conn, gs, creature_id).await?,
    })
}

pub async fn fetch_creature_scales(conn: &Pool<Sqlite>) -> Result<CreatureScales> {
    Ok(CreatureScales {
        ability_scales: sqlx::query_as!(AbilityScales, "SELECT * FROM ability_scales_table",)
            .fetch_all(conn)
            .await?
            .into_iter()
            .map(|n| (n.level, n))
            .collect(),
        ac_scales: sqlx::query_as!(AcScales, "SELECT * FROM ac_scales_table",)
            .fetch_all(conn)
            .await?
            .into_iter()
            .map(|n| (n.level, n))
            .collect(),
        area_dmg_scales: sqlx::query_as!(AreaDmgScales, "SELECT * FROM area_damage_scales_table",)
            .fetch_all(conn)
            .await?
            .into_iter()
            .map(|n| (n.level, n))
            .collect(),
        hp_scales: sqlx::query_as!(HpScales, "SELECT * FROM hp_scales_table",)
            .fetch_all(conn)
            .await?
            .into_iter()
            .map(|n| (n.level, n))
            .collect(),
        item_scales: sqlx::query_as!(ItemScales, "SELECT * FROM item_scales_table",)
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
        res_weak_scales: sqlx::query_as!(ResWeakScales, "SELECT * FROM res_weak_scales_table",)
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
        skill_scales: sqlx::query_as!(SkillScales, "SELECT * FROM skill_scales_table",)
            .fetch_all(conn)
            .await?
            .into_iter()
            .map(|n| (n.level, n))
            .collect(),
        spell_dc_and_atk_scales: sqlx::query_as!(
            SpellDcAndAtkScales,
            "SELECT * FROM spell_dc_and_attack_scales_table",
        )
        .fetch_all(conn)
        .await?
        .into_iter()
        .map(|n| (n.level, n))
        .collect(),
        strike_bonus_scales: sqlx::query_as!(
            StrikeBonusScales,
            "SELECT * FROM strike_bonus_scales_table",
        )
        .fetch_all(conn)
        .await?
        .into_iter()
        .map(|n| (n.level, n))
        .collect(),
        strike_dmg_scales: sqlx::query_as!(
            StrikeDmgScales,
            "SELECT * FROM strike_damage_scales_table",
        )
        .fetch_all(conn)
        .await?
        .into_iter()
        .map(|n| (n.level, n))
        .collect(),
    })
}
