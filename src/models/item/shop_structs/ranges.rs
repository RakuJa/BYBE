use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, PartialEq, Clone, Copy)]
pub struct ShopRanges {
    pub min_bulk: f64,
    pub max_bulk: f64,
    pub min_quantity: i64,
    pub max_quantity: i64,
    pub min_hp: i64,
    pub max_hp: i64,
    pub min_level: i64,
    pub max_level: i64,
    pub min_price: i64,
    pub max_price: i64,
    pub min_number_of_uses: i64,
    pub max_number_of_uses: i64,
}

impl Default for ShopRanges {
    fn default() -> Self {
        Self {
            min_bulk: f64::MAX,
            max_bulk: f64::MIN,
            min_quantity: i64::MAX,
            max_quantity: i64::MIN,
            min_hp: i64::MAX,
            max_hp: i64::MIN,
            min_level: i64::MAX,
            max_level: i64::MIN,
            min_price: i64::MAX,
            max_price: i64::MIN,
            min_number_of_uses: i64::MAX,
            max_number_of_uses: i64::MIN,
        }
    }
}
