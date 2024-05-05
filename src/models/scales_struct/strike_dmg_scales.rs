#[derive(Default, Eq, PartialEq, Clone)]
pub struct StrikeDmgScales {
    pub id: i64,
    pub level: i64,
    pub extreme: String,
    pub high: String,
    pub moderate: String,
    pub low: String,
}
