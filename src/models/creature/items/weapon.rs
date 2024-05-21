use crate::models::creature::creature_metadata::rarity_enum::RarityEnum;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq)]
pub struct Weapon {
    pub id: i64,
    pub name: String,
    pub base: String,
    pub to_hit_bonus: i64,
    pub bulk: i64,
    pub category: String,

    pub dmg_type: Option<String>,
    pub n_of_dices: Option<i64>,
    pub die_size: Option<String>,
    pub bonus_dmg: Option<i64>,

    pub carry_type: Option<String>,
    pub hands_held: Option<i64>,
    pub invested: Option<bool>,

    pub weapon_group: String,
    pub hardness: Option<i64>,
    pub hp_max: Option<i64>,
    pub hp_curr: Option<i64>,
    pub level: Option<i64>,

    pub license: String,
    pub remaster: bool,
    pub source: String,

    pub quantity: Option<i64>,
    pub range: Option<String>,
    pub reload: Option<String>,
    pub size: String,
    pub rarity: RarityEnum,
    pub usage: String,
    pub wp_type: String,
    pub creature_id: i64,
}

impl Weapon {
    pub fn get_avg_dmg(&self) -> Option<i64> {
        // avg dice value is
        // AVG = (((M+1)/2)∗N)+B
        //
        // M = max value of the dice
        // N = number of dices
        // B = bonus dmg
        let m = self
            .die_size
            .clone()?
            .split_once('d')?
            .1
            .parse::<f64>()
            .ok()?;
        let n = self.n_of_dices? as f64;
        let b = self.bonus_dmg? as f64;

        let avg: f64 = (((m + 1.) / 2.) * n) + b;
        Some(avg.floor() as i64)
    }
}
