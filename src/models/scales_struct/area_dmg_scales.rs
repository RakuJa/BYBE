#[derive(Default, Eq, PartialEq, Clone)]
pub struct AreaDmgScales {
    pub id: i64,
    pub level: i64,
    pub unlimited_use: String,
    pub limited_use: String,
}
