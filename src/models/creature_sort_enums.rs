use serde::{Deserialize, Serialize};

use strum::Display;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Display, Copy, Clone, Default)]
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

#[derive(Serialize, Deserialize, ToSchema, Display, Copy, Clone, Default)]
pub enum OrderEnum {
    #[default]
    Ascending,
    Descending,
}
