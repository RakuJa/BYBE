use crate::models::creature::creature_metadata::variant_enum::CreatureVariant;
use crate::models::db::pg_type_helper::{get_i32_as_i64, get_opt_i32_as_i16};
use crate::models::routers_validator_structs::Dice;
use crate::models::shared::range_data::RangeData;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::{Error, FromRow, Row};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq, Debug)]
pub struct Spell {
    pub id: i64,
    pub name: String,
    pub area_type: Option<String>,
    #[schema(example = 5)]
    pub area_value: Option<i32>,
    pub counteraction: bool,

    pub basic_saving_throw: Option<bool>,
    pub saving_throw: Option<String>,
    pub sustained: bool,

    pub duration: Option<String>,
    #[schema(example = 1)]
    pub level: i64,
    pub range: Option<RangeData>,
    pub target: String,
    pub actions: String,

    pub license: String,
    pub remaster: bool,
    pub source: String,
    pub rarity: String, // use rarityenum

    pub slot: i64,
    pub creature_id: i64,
    pub spellcasting_entry_id: i64,
    pub description: String,
    pub damage: Vec<SpellDamageData>,
}

impl<'r> FromRow<'r, PgRow> for Spell {
    fn from_row(row: &'r PgRow) -> Result<Self, Error> {
        Ok(Self {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            area_type: row.try_get("area_type")?,
            area_value: row.try_get("area_value")?,
            counteraction: row.try_get("counteraction")?,
            basic_saving_throw: row.try_get("basic_saving_throw")?,
            saving_throw: row.try_get("saving_throw")?,
            sustained: row.try_get("sustained")?,
            duration: row.try_get("duration")?,
            level: get_i32_as_i64(row, "level")?,
            target: row.try_get("target")?,
            actions: row.try_get("actions")?,
            license: row.try_get("license")?,
            remaster: row.try_get("remaster")?,
            source: row.try_get("source")?,
            rarity: row.try_get("rarity")?,
            slot: get_i32_as_i64(row, "slot")?,
            creature_id: get_i32_as_i64(row, "creature_id")?,
            spellcasting_entry_id: get_i32_as_i64(row, "spellcasting_entry_id")?,
            description: row.try_get("description")?,
            range: RangeData::from_row(row).ok(),
            damage: vec![],
        })
    }
}

impl Spell {
    pub fn convert_from_base_to_variant(self, variant: CreatureVariant) -> Self {
        let modifier = variant.to_dmg_adjustment_modifier(true);
        Self {
            damage: self
                .damage
                .into_iter()
                .map(|dmg| dmg.add_mod(modifier))
                .collect(),
            ..self
        }
    }
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq, Debug)]
pub struct SpellDamageData {
    pub id: i64,
    #[schema(example = 10)]
    pub bonus_dmg: i64,
    pub dmg_type: Option<String>,
    pub category: Option<String>,
    pub kinds: Vec<String>,
    pub dice: Option<Dice>,
}

impl SpellDamageData {
    const fn add_mod(mut self, modifier: i64) -> Self {
        self.bonus_dmg += modifier;
        self
    }
}

impl<'r> FromRow<'r, PgRow> for SpellDamageData {
    fn from_row(row: &'r PgRow) -> Result<Self, Error> {
        Ok(Self {
            id: row.try_get("id")?,
            bonus_dmg: get_i32_as_i64(row, "bonus_dmg")?,
            dmg_type: row.try_get("dmg_type").ok(),
            category: row.try_get("category").ok(),
            kinds: row.try_get("kinds").unwrap_or_default(),
            dice: Dice::from_optional_dice_number_and_size(
                get_opt_i32_as_i16(row, "number_of_dice"),
                get_opt_i32_as_i16(row, "die_size"),
            ),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn dmg(bonus_dmg: i64) -> SpellDamageData {
        SpellDamageData {
            id: 1,
            bonus_dmg,
            dmg_type: Some("fire".to_string()),
            category: None,
            kinds: vec!["damage".to_string()],
            dice: Some(Dice {
                n_of_dices: 6,
                dice_size: 6,
            }),
        }
    }

    fn spell(damage: Vec<SpellDamageData>) -> Spell {
        Spell {
            id: 1,
            name: "Fireball".to_string(),
            area_type: None,
            area_value: None,
            counteraction: false,
            basic_saving_throw: None,
            saving_throw: None,
            sustained: false,
            duration: None,
            level: 3,
            range: None,
            target: String::new(),
            actions: String::new(),
            license: String::new(),
            remaster: false,
            source: String::new(),
            rarity: "common".to_string(),
            slot: 3,
            creature_id: 1,
            spellcasting_entry_id: 1,
            description: String::default(),
            damage,
        }
    }

    #[rstest]
    #[case(CreatureVariant::Base, 0)]
    #[case(CreatureVariant::Elite, 4)]
    #[case(CreatureVariant::Weak, -4)]
    fn convert_from_base_to_variant_adjusts_damage_by_four(
        #[case] variant: CreatureVariant,
        #[case] expected_delta: i64,
    ) {
        let og_dmg_zero = dmg(0);
        let og_dmg_two = dmg(2);
        let converted = spell(vec![dmg(2), dmg(0)]).convert_from_base_to_variant(variant);
        assert_eq!(
            og_dmg_two.bonus_dmg + expected_delta,
            converted.damage[0].bonus_dmg
        );
        assert_eq!(
            og_dmg_zero.bonus_dmg + expected_delta,
            converted.damage[1].bonus_dmg
        );
    }
}
