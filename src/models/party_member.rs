use serde::{Deserialize, Serialize};
use utoipa::IntoParams;
use validator::Validate;

#[derive(Serialize, Deserialize, IntoParams, Validate)]
pub struct PartyMember {
    level: i8,
}
