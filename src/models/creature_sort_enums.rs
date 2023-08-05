use serde::{Deserialize, Serialize};
use std::fmt::Display;
use strum::Display;

#[derive(Serialize, Deserialize, Display, Copy, Clone)]
pub enum SortEnum {
    ID,
    NAME,
    HP,
    LEVEL,
    FAMILY,
    ALIGNMENT,
    SIZE,
    RARITY,
}

#[derive(Serialize, Deserialize, Display, Copy, Clone)]
pub enum OrderEnum {
    ASCENDING,
    DESCENDING,
}

impl Default for SortEnum {
    fn default() -> Self {
        SortEnum::ID
    }
}

impl Default for OrderEnum {
    fn default() -> Self {
        OrderEnum::ASCENDING
    }
}
