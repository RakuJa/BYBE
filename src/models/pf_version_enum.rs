use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Default, ToSchema, Clone)]
pub enum PathfinderVersionEnum {
    Legacy,
    Remaster,
    #[default]
    Any,
}
