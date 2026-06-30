use sqlx::FromRow;
#[derive(Default, Eq, PartialEq, Clone, FromRow)]
pub struct ResWeakScales {
    pub id: i64,
    pub level: i64,
    pub max: i64,
    pub min: i64,
}
