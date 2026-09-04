use crate::models::item::shop_structs::item_table_fields_filter::ItemTableFieldsFilter;

pub struct ShopFilterQuery {
    pub item_table_fields_filter: ItemTableFieldsFilter,
    pub trait_whitelist_filter: Vec<String>,
    pub trait_blacklist_filter: Vec<String>,

    pub n_of_equipment: i64,
    pub n_of_weapons: i64,
    pub n_of_armors: i64,
    pub n_of_shields: i64,
    pub n_of_treasures: i64,
    pub n_of_backpacks: i64,

    pub n_of_generic_consumables: i64,
    pub n_of_ammunition: i64,
}
