use sqlx::FromRow;
#[derive(Default, Eq, PartialEq, Clone, FromRow)]
pub struct StrikeDmgScales {
    pub level: i64,
    pub extreme: String,
    pub high: String,
    pub moderate: String,
    pub low: String,
}
