use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq)]
pub struct SpellCasterEntry {
    pub spell_casting_name: Option<String>,
    pub is_spell_casting_flexible: Option<bool>,
    pub type_of_spell_caster: Option<String>,
    pub spell_casting_dc_mod: Option<i64>,
    pub spell_casting_atk_mod: Option<i64>,
    pub spell_casting_tradition: Option<String>,
}
