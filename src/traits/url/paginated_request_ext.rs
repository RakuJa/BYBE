use crate::models::routers_validator_structs::PaginatedRequest;
use crate::traits::url::sort_data_ext::SortDataExt;

pub trait PaginatedRequestExt {
    type Sort: SortDataExt;

    fn base_path() -> &'static str;
    fn sort_data(&self) -> &Self::Sort;
    fn paginated_request(&self) -> &PaginatedRequest;

    // Default implementations
    fn sort_by(&self) -> String {
        self.sort_data().sort_by()
    }
    fn order_by(&self) -> String {
        self.sort_data().order_by()
    }
    fn page_size(&self) -> i16 {
        self.paginated_request().page_size
    }
}
