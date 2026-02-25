use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq, Debug)]
pub struct Skill {
    pub name: String,
    pub description: Option<String>,
    #[schema(example = 0)]
    pub modifier: i64,
    #[schema(example = 0)]
    pub proficiency: i64,
    // pub publication_info: PublicationInfo,
    // pub variant_label: Vec<String>,
}
