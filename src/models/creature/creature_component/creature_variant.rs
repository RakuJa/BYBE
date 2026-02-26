use crate::models::creature::creature_metadata::variant_enum::CreatureVariant;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq, Debug)]
pub struct CreatureVariantData {
    pub variant: CreatureVariant,
    #[schema(example = 0)]
    pub level: i64,
    pub archive_link: Option<String>,
}
