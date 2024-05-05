use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq)]
pub struct CreatureVariantData {
    pub level: i8,
    pub archive_link: Option<String>,
}

impl From<(i64, Option<String>)> for CreatureVariantData {
    fn from(value: (i64, Option<String>)) -> Self {
        Self {
            level: value.0 as i8,
            archive_link: value.1,
        }
    }
}
