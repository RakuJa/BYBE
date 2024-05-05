use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq)]
pub struct Skill {
    pub name: String,
    pub description: Option<String>,
    pub modifier: i64,
    pub proficiency: i64,
    // pub publication_info: PublicationInfo,
    // pub variant_label: Vec<String>,
}
