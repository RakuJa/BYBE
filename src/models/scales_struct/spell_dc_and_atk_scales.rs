use sqlx::FromRow;
#[derive(Default, Eq, PartialEq, Clone, FromRow)]
pub struct SpellDcAndAtkScales {
    #[sqlx(try_from = "i32")]
    pub level: i64,
    #[sqlx(try_from = "i32")]
    pub extreme_dc: i64,
    #[sqlx(try_from = "i32")]
    pub extreme_atk_bonus: i64,
    #[sqlx(try_from = "i32")]
    pub high_dc: i64,
    #[sqlx(try_from = "i32")]
    pub high_atk_bonus: i64,
    #[sqlx(try_from = "i32")]
    pub moderate_dc: i64,
    #[sqlx(try_from = "i32")]
    pub moderate_atk_bonus: i64,
}
