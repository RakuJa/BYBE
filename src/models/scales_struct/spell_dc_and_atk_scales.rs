#[derive(Default, Eq, PartialEq, Clone)]
pub struct SpellDcAndAtkScales {
    pub id: i64,
    pub level: i64,
    pub extreme_dc: i64,
    pub extreme_atk_bonus: i64,
    pub high_dc: i64,
    pub high_atk_bonus: i64,
    pub moderate_dc: i64,
    pub moderate_atk_bonus: i64,
}
