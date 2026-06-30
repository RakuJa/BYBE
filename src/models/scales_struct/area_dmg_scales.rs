use sqlx::FromRow;
#[derive(Default, Eq, PartialEq, Clone, FromRow)]
pub struct AreaDmgScales {
    pub level: i64,
    pub unlimited_use: String,
    pub limited_use: String,
}
