#[derive(Default, Eq, PartialEq, Clone)]
pub struct HpScales {
    pub id: i64,
    pub level: i64,
    pub high_ub: i64,
    pub high_lb: i64,
    pub moderate_ub: i64,
    pub moderate_lb: i64,
    pub low_ub: i64,
    pub low_lb: i64,
}
