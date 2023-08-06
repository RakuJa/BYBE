use serde::{Deserialize, Serialize};

use strum::Display;

#[derive(Serialize, Deserialize, Display, Copy, Clone, Default)]
pub enum SortEnum {
    #[default]
    Id,
    Name,
    Hp,
    Level,
    Family,
    Alignment,
    Size,
    Rarity,
}

#[derive(Serialize, Deserialize, Display, Copy, Clone, Default)]
pub enum OrderEnum {
    #[default]
    Ascending,
    Descending,
}
