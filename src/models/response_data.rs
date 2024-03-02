use crate::models::creature::{CoreCreatureData, Creature, ExtraCreatureData, VariantCreatureData};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Serialize, Deserialize, IntoParams, Default, Eq, PartialEq, Hash, Clone, Validate)]
pub struct ResponseData {
    pub essential_data: bool,
    pub variant_data: bool,
    pub extra_data: bool,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq)]
pub struct ResponseCreature {
    pub core_data: Option<CoreCreatureData>,
    pub variant_data: Option<VariantCreatureData>,
    pub extra_data: Option<ExtraCreatureData>,
}

impl From<(Creature, &ResponseData)> for ResponseCreature {
    fn from(value: (Creature, &ResponseData)) -> Self {
        let cr = value.0;
        let rd = value.1;
        Self {
            core_data: if rd.essential_data {
                Some(cr.core_data)
            } else {
                None
            },
            variant_data: if rd.variant_data {
                Some(cr.variant_data)
            } else {
                None
            },
            extra_data: if rd.extra_data {
                Some(cr.extra_data)
            } else {
                None
            },
        }
    }
}
