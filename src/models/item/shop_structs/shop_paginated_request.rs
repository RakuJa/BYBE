use crate::models::item::shop_structs::sort_data::ShopSortData;
use crate::models::routers_validator_structs::PaginatedRequest;
use crate::traits::url::paginated_request_ext::PaginatedRequestExt;
use serde::{Deserialize, Serialize};
use utoipa::IntoParams;

#[derive(Serialize, Deserialize, IntoParams, Eq, PartialEq, Hash)]
pub struct ShopPaginatedRequest {
    pub paginated_request: PaginatedRequest,
    pub shop_sort_data: ShopSortData,
}

impl PaginatedRequestExt for ShopPaginatedRequest {
    type Sort = ShopSortData;
    fn base_path() -> &'static str {
        "shop/list"
    }
    fn sort_data(&self) -> &Self::Sort {
        &self.shop_sort_data
    }
    fn paginated_request(&self) -> &PaginatedRequest {
        &self.paginated_request
    }
}
