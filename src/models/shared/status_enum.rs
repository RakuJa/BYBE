use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default, Eq, Hash, ToSchema)]
pub enum Status {
    #[default]
    Valid,
    Archived,
    Deprecated,
}

impl From<&str> for Status {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "valid" => Self::Valid,
            "archived" => Self::Archived,
            _ => Self::Deprecated,
        }
    }
}

impl From<String> for Status {
    fn from(s: String) -> Self {
        Self::from(s.as_ref())
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Valid => "valid",
                Self::Archived => "archived",
                Self::Deprecated => "deprecated",
            }
        )
    }
}
