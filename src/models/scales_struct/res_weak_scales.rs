use sqlx::FromRow;
#[derive(Default, Eq, PartialEq, Clone, FromRow)]
pub struct ResWeakScales {
    #[sqlx(try_from = "i32")]
    pub level: i64,
    #[sqlx(try_from = "i32")]
    pub max: i64,
    #[sqlx(try_from = "i32")]
    pub min: i64,
}
