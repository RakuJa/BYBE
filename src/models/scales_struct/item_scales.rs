#[derive(Default, Eq, PartialEq, Clone, sqlx::FromRow)]
pub struct ItemScales {
    pub cr_level: String,
    pub safe_item_level: String,
}
