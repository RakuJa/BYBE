use crate::models::creature::creature_metadata::variant_enum::CreatureVariant;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq)]
pub struct CreatureVariantData {
    pub variant: CreatureVariant,
    pub level: i64,
    pub archive_link: Option<String>,
}
