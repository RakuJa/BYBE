#[derive(Default, Eq, PartialEq, Clone)]
pub struct SkillScales {
    pub id: i64,
    pub level: i64,
    pub extreme: i64,
    pub high: i64,
    pub moderate: i64,
    pub low_ub: i64,
    pub low_lb: i64,
}
