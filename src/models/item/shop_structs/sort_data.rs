use crate::models::item::shop_structs::item_sort_enum::ItemSortEnum;
use crate::models::routers_validator_structs::OrderEnum;
use crate::traits::url::has_sort_fields::HasSortFields;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Serialize, Deserialize, IntoParams, ToSchema, Eq, PartialEq, Hash, Default)]
pub struct ShopSortData {
    // Optional here for swagger, kinda bad but w/e
    pub sort_by: Option<ItemSortEnum>,
    pub order_by: Option<OrderEnum>,
}
impl HasSortFields for ShopSortData {
    type SortBy = ItemSortEnum;
    fn sort_by_field(&self) -> &Option<Self::SortBy> {
        &self.sort_by
    }
    fn order_by_field(&self) -> &Option<OrderEnum> {
        &self.order_by
    }
}
