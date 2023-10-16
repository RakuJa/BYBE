use serde::{Deserialize, Serialize};

use strum::Display;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Display, Copy, Clone, Default, Hash, Eq, PartialEq)]
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

#[derive(Serialize, Deserialize, ToSchema, Display, Copy, Clone, Default, Hash, Eq, PartialEq)]
pub enum OrderEnum {
    #[default]
    Ascending,
    Descending,
}
