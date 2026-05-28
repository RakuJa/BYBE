use sqlx::FromRow;
#[derive(Default, Eq, PartialEq, Clone, FromRow)]
pub struct AbilityScales {
    #[sqlx(try_from = "i32")]
    pub level: i64,
    pub extreme: Option<i32>,
    #[sqlx(try_from = "i32")]
    pub high: i64,
    #[sqlx(try_from = "i32")]
    pub moderate: i64,
    #[sqlx(try_from = "i32")]
    pub low: i64,
}
