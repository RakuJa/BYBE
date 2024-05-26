use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};
use utoipa::ToSchema;

#[derive(
    Serialize, Deserialize, Eq, PartialEq, Hash, Default, ToSchema, Clone, EnumIter, Display,
)]
pub enum PathfinderVersionEnum {
    Legacy,
    Remaster,
    #[default]
    Any,
}
