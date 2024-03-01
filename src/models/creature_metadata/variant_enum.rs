use serde::{Deserialize, Serialize};
use strum::Display;
use utoipa::ToSchema;

#[derive(
    Serialize, Deserialize, ToSchema, Display, Eq, Hash, PartialEq, Ord, PartialOrd, Default,
)]
pub enum CreatureVariant {
    Weak,
    Elite,
    #[default]
    Base,
}

impl Clone for CreatureVariant {
    fn clone(&self) -> CreatureVariant {
        match self {
            CreatureVariant::Elite => CreatureVariant::Elite,
            CreatureVariant::Weak => CreatureVariant::Weak,
            CreatureVariant::Base => CreatureVariant::Base,
        }
    }
}

impl CreatureVariant {
    pub fn to_level_delta(&self) -> i8 {
        match self {
            CreatureVariant::Weak => -1,
            CreatureVariant::Elite => 1,
            CreatureVariant::Base => 0,
        }
    }

    pub fn to_cache_index(&self) -> i32 {
        match self {
            CreatureVariant::Base => 0,
            CreatureVariant::Weak => 1,
            CreatureVariant::Elite => 2,
        }
    }
}
