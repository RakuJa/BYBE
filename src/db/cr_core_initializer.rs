use crate::models::shared::game_system_enum::GameSystem;
use anyhow::Result;
use sqlx::{AssertSqlSafe, PgPool};
use tracing::warn;

pub async fn update_creature_core_table(pool: &PgPool, gs: GameSystem) -> Result<()> {
    warn!("Handler for startup, Should only be used once for each gamesystem");
    create_and_populate_core_table(pool, gs).await
}

async fn create_and_populate_core_table(pool: &PgPool, gs: GameSystem) -> Result<()> {
    let mut conn = pool.acquire().await?;

    sqlx::query(AssertSqlSafe(format!(
        "DROP TABLE IF EXISTS {gs}_creature_core"
    )))
    .execute(&mut *conn)
    .await?;

    // ---------------------------------------------------------------------
    // Scoring model (unchanged from the original Rust logic):
    // score  = sum of distance terms (lower = closer to the ideal role)
    // result = ROUND(EXP(-0.2 * score) * 100)
    sqlx::query(AssertSqlSafe(format!(r#"
CREATE TABLE {gs}_creature_core AS
WITH
-- per-weapon average damage + to-hit bonus
weapon_avgs AS (
    SELECT
        wca.creature_id,
        wt.weapon_type,
        COALESCE(wt.to_hit_bonus::bigint, 0) AS to_hit_bonus,
        COALESCE((
            SELECT SUM(FLOOR(((wd.die_size + 1.0) / 2.0) * wd.number_of_dice + wd.bonus_dmg))
            FROM   {gs}_weapon_damage_table wd
            WHERE  wd.weapon_id = wt.id
        ), 0)::bigint AS avg_dmg
    FROM {gs}_weapon_creature_association_table wca
    JOIN {gs}_weapon_table wt ON wt.id = wca.weapon_id
),

-- melee / ranged presence
weapon_flags AS (
    SELECT
        creature_id,
        bool_or(UPPER(weapon_type) = 'MELEE')  AS has_melee,
        bool_or(ranged IS NOT NULL AND ranged > 0) AS has_ranged
    FROM weapon_avgs
    GROUP BY creature_id
),

-- per-weapon strike distances, evaluated against the creature's level scales.
-- INNER JOINs to the scales: a creature whose level has no scale row yields no
-- strike row -> no strike_agg row -> the per-role guards below zero the role,
-- which is exactly the original behaviour.
weapon_strikes AS (
    SELECT
        w.creature_id,
        w.weapon_type,
        -- high-damage benchmark (magical_striker, soldier)
        GREATEST(0, atk.high - w.to_hit_bonus)
            + GREATEST(0, CAST(substring(ds.high FROM '\((\d+)\)') AS integer)::bigint - w.avg_dmg)
            AS strike_high,
        -- moderate-damage benchmark (sniper, ranged only)
        GREATEST(0, atk.high - w.to_hit_bonus)
            + GREATEST(0, CAST(substring(ds.moderate FROM '\((\d+)\)') AS integer)::bigint - w.avg_dmg)
            AS strike_mod,
        -- brute: LEAST of a high-damage path and an extreme-damage path
        LEAST(
            GREATEST(0, atk.high - w.to_hit_bonus)
                + GREATEST(0, CAST(substring(ds.high FROM '\((\d+)\)') AS integer)::bigint - w.avg_dmg),
            (CASE WHEN w.to_hit_bonus <  atk.moderate THEN atk.moderate - w.to_hit_bonus
                  WHEN w.to_hit_bonus >= atk.high     THEN w.to_hit_bonus + 1 - atk.high
                  ELSE 0 END)
                + GREATEST(0, CAST(substring(ds.extreme FROM '\((\d+)\)') AS integer)::bigint - w.avg_dmg)
        ) AS strike_brute
    FROM weapon_avgs w
    JOIN {gs}_creature_table        c   ON c.id     = w.creature_id
    JOIN strike_bonus_scales_table  atk ON atk.level = c.level
    JOIN strike_damage_scales_table ds  ON ds.level  = c.level::text
),

-- best (lowest-distance) strike per creature, by flavour
strike_agg AS (
    SELECT
        creature_id,
        MIN(strike_high)  AS best_strike_high,
        MIN(strike_brute) AS best_strike_brute,
        MIN(strike_mod) FILTER (WHERE UPPER(weapon_type) = 'RANGED') AS best_strike_ranged_mod
    FROM weapon_strikes
    GROUP BY creature_id
),

-- alignment trait presence
trait_flags AS (
    SELECT
        creature_id,
        bool_or(UPPER(trait_id) = 'GOOD')    AS is_good,
        bool_or(UPPER(trait_id) = 'EVIL')    AS is_evil,
        bool_or(UPPER(trait_id) = 'CHAOTIC') AS is_chaotic,
        bool_or(UPPER(trait_id) = 'LAWFUL')  AS is_lawful
    FROM {gs}_trait_creature_association_table
    GROUP BY creature_id
),

-- spellcasting: best DC + total spells. A row exists iff the creature casts.
spell_stats AS (
    SELECT
        se.creature_id,
        MAX(se.spellcasting_dc_mod) AS max_spell_dc,
        COUNT(s.id)                 AS spell_count
    FROM {gs}_spellcasting_entry_table se
    LEFT JOIN {gs}_spell_table s
           ON s.spellcasting_entry_id = se.id
          AND s.creature_id           = se.creature_id
    GROUP BY se.creature_id
),

-- skills: best modifier + total count
skill_stats AS (
    SELECT
        creature_id,
        MAX(modifier)::bigint AS max_skill_mod,
        COUNT(*)              AS skill_count
    FROM {gs}_skill_table
    GROUP BY creature_id
),

-- skills clearing the level's "moderate" saving-throw bar
skill_above_mod AS (
    SELECT
        sk.creature_id,
        COUNT(*) FILTER (WHERE sk.modifier::bigint >= st.moderate) AS n_above_moderate
    FROM {gs}_skill_table sk
    JOIN {gs}_creature_table       c  ON c.id      = sk.creature_id
    JOIN saving_throw_scales_table st ON st.level  = c.level
    GROUP BY sk.creature_id
),

-- action profile: offensive-action count + Attack of Opportunity presence
action_stats AS (
    SELECT
        ca.creature_id,
        COUNT(*) FILTER (
            WHERE UPPER(a.category) = 'OFFENSIVE' AND UPPER(a.action_type) = 'ACTION'
        ) AS n_offensive_actions,
        -- BUG? `a.slug IS NULL` makes ANY null-slug action count as AoO,
        -- granting the soldier bonus to creatures that have no real AoO.
        bool_or(
            UPPER(a.name) = 'ATTACK OF OPPORTUNITY'
            OR a.slug IS NULL
            OR UPPER(a.slug) = 'ATTACK-OF-OPPORTUNITY'
        ) AS has_aoo
    FROM {gs}_creature_action_association_table ca
    JOIN {gs}_action_table a ON a.id = ca.action_id
    GROUP BY ca.creature_id
),

-- speed shortfall vs. the 30 ft benchmark
speed_stats AS (
    SELECT
        creature_id,
        MIN(GREATEST(0, 30 - value::bigint)) AS speed_penalty
    FROM {gs}_speed_table
    GROUP BY creature_id
)

SELECT
    t.id,
    t.aon_id,
    t.name,
    t.hp,
    t.level,
    t.size,
    t.family,
    t.rarity,
    t.license,
    t.remaster,
    t.source,
    t.cr_type,
    t.n_of_focus_points AS focus_points,
    t.status,
    NULL::text AS archive_link,

    -- alignment
    CASE
        WHEN t.remaster                     THEN 'No Alignment'
        WHEN COALESCE(tf.is_good, false) THEN
            CASE WHEN COALESCE(tf.is_chaotic, false) THEN 'CG'
                 WHEN COALESCE(tf.is_lawful,  false) THEN 'LG'
                 ELSE 'NG' END
        WHEN COALESCE(tf.is_evil, false) THEN
            CASE WHEN COALESCE(tf.is_chaotic, false) THEN 'CE'
                 WHEN COALESCE(tf.is_lawful,  false) THEN 'LE'
                 ELSE 'NE' END
        WHEN COALESCE(tf.is_chaotic, false) THEN 'CN'
        WHEN COALESCE(tf.is_lawful,  false) THEN 'LN'
        ELSE 'N'
    END AS alignment,

    -- combat flags
    COALESCE(wf.has_melee,  false) AS is_melee,
    COALESCE(wf.has_ranged, false) AS is_ranged,
    (ss.creature_id IS NOT NULL)   AS is_spellcaster,

    -- brute: low perception, high str/con, low mental, low reflex/will,
    -- high fort, moderate AC, high HP, strong attack + damage
    COALESCE(
        CASE WHEN per.level IS NULL OR ab.level IS NULL OR st.level IS NULL
               OR acs.level IS NULL OR hp.level IS NULL OR atk.level IS NULL
               OR substring(ds.high    FROM '\((\d+)\)') IS NULL
               OR substring(ds.extreme FROM '\((\d+)\)') IS NULL
             THEN NULL
             ELSE ROUND(EXP(-0.2 * (
                   GREATEST(0, (t.perception::bigint   + 1) - per.moderate)
                 + GREATEST(0, ab.high     - t.strength::bigint)
                 + GREATEST(0, ab.moderate - t.constitution::bigint)
                 + GREATEST(0, (t.intelligence::bigint + 1) - ab.moderate)
                 + GREATEST(0, (t.wisdom::bigint       + 1) - ab.moderate)
                 + GREATEST(0, (t.charisma::bigint     + 1) - ab.moderate)
                 + GREATEST(0, (t.reflex::bigint       + 1) - st.moderate)
                 + GREATEST(0, st.high - t.fortitude::bigint)
                 + GREATEST(0, (t.will::bigint         + 1) - st.moderate)
                 + GREATEST(0, (t.ac::bigint           + 1) - acs.high)
                 + GREATEST(0, hp.high_lb - t.hp::bigint)
                 + COALESCE(sa.best_strike_brute, 20)
             )::double precision) * 100)::bigint
        END, 0) AS brute_percentage,

    -- magical striker: solid melee strike + >= moderate spell DC + enough spells
    COALESCE(
        CASE WHEN atk.level IS NULL
               OR substring(ds.high FROM '\((\d+)\)') IS NULL
               OR ss.creature_id IS NULL
               OR sda.level IS NULL
             THEN NULL
             ELSE ROUND(EXP(-0.2 * (
                   COALESCE(sa.best_strike_high, 20)
                 + GREATEST(0, sda.moderate_dc - ss.max_spell_dc)
                 + GREATEST(0, (CEIL(t.level::double precision / 2.0) - 1)::bigint - ss.spell_count)
             )::double precision) * 100)::bigint
        END, 0) AS magical_striker_percentage,

    -- skill paragon: high best-skill modifier, low fort, high reflex or will,
    -- many skills above the moderate bar, multiple offensive actions
    COALESCE(
        CASE WHEN ab.level IS NULL OR st.level IS NULL OR sk.level IS NULL
               OR sks.max_skill_mod IS NULL
             THEN NULL
             ELSE ROUND(EXP(-0.2 * (
                   GREATEST(0, ab.high - sks.max_skill_mod)
                 + GREATEST(0, (t.fortitude::bigint + 1) - st.moderate)
                 + LEAST(GREATEST(0, st.high - t.reflex::bigint),
                         GREATEST(0, st.high - t.will::bigint))
                 + ABS(COALESCE(sam.n_above_moderate, 0) - (sks.skill_count / 70 * 100))
                 + CASE WHEN COALESCE(act.n_offensive_actions, 0) < 2 THEN 20 ELSE 0 END
             )::double precision) * 100)::bigint
        END, 0) AS skill_paragon_percentage,

    -- skirmisher: high dex, low fort, high reflex, fast speed
    COALESCE(
        CASE WHEN ab.level IS NULL OR st.level IS NULL
             THEN NULL
             ELSE ROUND(EXP(-0.2 * (
                   GREATEST(0, ab.high - t.dexterity::bigint)
                 + GREATEST(0, (t.fortitude::bigint + 1) - st.moderate)
                 + GREATEST(0, st.high - t.reflex::bigint)
                 + COALESCE(spd.speed_penalty, 20)
             )::double precision) * 100)::bigint
        END, 0) AS skirmisher_percentage,

    -- sniper: high perception, high dex, high reflex, strong ranged strike
    COALESCE(
        CASE WHEN per.level IS NULL OR ab.level IS NULL OR st.level IS NULL OR atk.level IS NULL
               OR substring(ds.moderate FROM '\((\d+)\)') IS NULL
             THEN NULL
             ELSE ROUND(EXP(-0.2 * (
                   GREATEST(0, per.moderate - t.perception::bigint)
                 + GREATEST(0, ab.moderate  - t.dexterity::bigint)
                 + GREATEST(0, st.moderate  - t.reflex::bigint)
                 + COALESCE(sa.best_strike_ranged_mod, 20)
             )::double precision) * 100)::bigint
        END, 0) AS sniper_percentage,

    -- soldier: high str, high AC, high fort, strong melee, offensive actions + AoO
    COALESCE(
        CASE WHEN ab.level IS NULL OR acs.level IS NULL OR st.level IS NULL OR atk.level IS NULL
               OR substring(ds.high FROM '\((\d+)\)') IS NULL
             THEN NULL
             ELSE ROUND(EXP(-0.2 * (
                   GREATEST(0, ab.high  - t.strength::bigint)
                 + GREATEST(0, acs.high - t.ac::bigint)
                 + GREATEST(0, st.high  - t.fortitude::bigint)
                 + COALESCE(sa.best_strike_high, 20)
                 + CASE WHEN COALESCE(act.n_offensive_actions, 0) = 0 THEN 20 ELSE 0 END
                 + CASE WHEN NOT COALESCE(act.has_aoo, false)        THEN  3 ELSE 0 END
             )::double precision) * 100)::bigint
        END, 0) AS soldier_percentage,

    -- spellcaster: low fort, high will, low HP, high spell DC, enough spells, high mental
    COALESCE(
        CASE WHEN st.level IS NULL OR hp.level IS NULL OR sda.level IS NULL OR ab.level IS NULL
               OR ss.creature_id IS NULL
             THEN NULL
             ELSE ROUND(EXP(-0.2 * (
                   GREATEST(0, (t.fortitude::bigint + 1) - st.moderate)
                 + GREATEST(0, st.high - t.will::bigint)
                 + GREATEST(0, (t.hp::bigint + 1) - hp.high_lb)
                 + GREATEST(0, sda.high_dc - ss.max_spell_dc)
                 + GREATEST(0, CEIL(t.level::double precision / 2.0)::bigint - ss.spell_count)
                 + GREATEST(0, ab.high - GREATEST(t.intelligence::bigint,
                                                  t.wisdom::bigint,
                                                  t.charisma::bigint))
             )::double precision) * 100)::bigint
        END, 0) AS spellcaster_percentage

FROM {gs}_creature_table t
LEFT JOIN ability_scales_table             ab  ON ab.level  = t.level
LEFT JOIN ac_scales_table                  acs ON acs.level = t.level
LEFT JOIN hp_scales_table                  hp  ON hp.level  = t.level
LEFT JOIN perception_scales_table          per ON per.level = t.level
LEFT JOIN saving_throw_scales_table        st  ON st.level  = t.level
LEFT JOIN skill_scales_table               sk  ON sk.level  = t.level
LEFT JOIN spell_dc_and_attack_scales_table sda ON sda.level = t.level
LEFT JOIN strike_bonus_scales_table        atk ON atk.level = t.level
LEFT JOIN strike_damage_scales_table       ds  ON ds.level  = t.level::text
LEFT JOIN trait_flags     tf  ON tf.creature_id  = t.id
LEFT JOIN weapon_flags    wf  ON wf.creature_id  = t.id
LEFT JOIN spell_stats     ss  ON ss.creature_id  = t.id
LEFT JOIN skill_stats     sks ON sks.creature_id = t.id
LEFT JOIN skill_above_mod sam ON sam.creature_id = t.id
LEFT JOIN action_stats    act ON act.creature_id = t.id
LEFT JOIN speed_stats     spd ON spd.creature_id = t.id
LEFT JOIN strike_agg      sa  ON sa.creature_id  = t.id
    "#)))
        .execute(&mut *conn)
        .await?;

    // The primary key is the index that actually earns its keep: it backs
    // joins from other tables and id lookups.
    sqlx::query(AssertSqlSafe(format!(
        "ALTER TABLE {gs}_creature_core ADD PRIMARY KEY (id)"
    )))
    .execute(&mut *conn)
    .await?;

    sqlx::query(AssertSqlSafe(format!(
        "CREATE INDEX {gs}_creature_core_level_idx ON {gs}_creature_core (level)"
    )))
    .execute(&mut *conn)
    .await?;

    Ok(())
}
