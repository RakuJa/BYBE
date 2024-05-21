use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Clone)]
pub enum ItemField {
    #[serde(alias = "category", alias = "CATEGORY")]
    Category,
    #[serde(alias = "level", alias = "LEVEL")]
    Level,
    #[serde(alias = "usage", alias = "USAGE")]
    Usage,
    #[serde(alias = "item_type", alias = "ITEM_TYPE")]
    ItemType,
    #[serde(alias = "material_grade", alias = "MATERIAL_GRADE")]
    MaterialGrade,
    #[serde(alias = "material_type", alias = "MATERIAL_TYPE")]
    MaterialType,
    #[serde(alias = "number_of_uses", alias = "NUMBER_OF_USES")]
    NumberOfUses,
    #[serde(alias = "size", alias = "SIZE")]
    Size,
    #[serde(alias = "rarity", alias = "RARITY")]
    Rarity,
    #[serde(alias = "sources", alias = "SOURCES")]
    Sources,
    #[serde(alias = "traits", alias = "TRAITS")]
    Traits,
}

#[derive(Default, Eq, PartialEq, Clone)]
pub struct FieldsUniqueValuesStruct {
    pub list_of_levels: Vec<String>,
    pub list_of_categories: Vec<String>,
    pub list_of_traits: Vec<String>,
    pub list_of_sources: Vec<String>,
    pub list_of_sizes: Vec<String>,
    pub list_of_rarities: Vec<String>,
}
