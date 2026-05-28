use sqlx::FromRow;
#[derive(Default, Eq, PartialEq, Clone, FromRow)]
pub struct SavingThrowScales {
    #[sqlx(try_from = "i32")]
    pub level: i64,
    #[sqlx(try_from = "i32")]
    pub extreme: i64,
    #[sqlx(try_from = "i32")]
    pub high: i64,
    #[sqlx(try_from = "i32")]
    pub moderate: i64,
    #[sqlx(try_from = "i32")]
    pub low: i64,
    #[sqlx(try_from = "i32")]
    pub terrible: i64,
}
