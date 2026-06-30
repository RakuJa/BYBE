use sqlx::FromRow;
#[derive(Default, Eq, PartialEq, Clone, FromRow)]
pub struct HpScales {
    #[sqlx(try_from = "i32")]
    pub level: i64,
    #[sqlx(try_from = "i32")]
    pub high_ub: i64,
    #[sqlx(try_from = "i32")]
    pub high_lb: i64,
    #[sqlx(try_from = "i32")]
    pub moderate_ub: i64,
    #[sqlx(try_from = "i32")]
    pub moderate_lb: i64,
    #[sqlx(try_from = "i32")]
    pub low_ub: i64,
    #[sqlx(try_from = "i32")]
    pub low_lb: i64,
}
