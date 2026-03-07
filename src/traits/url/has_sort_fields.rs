use crate::models::routers_validator_structs::OrderEnum;

pub trait HasSortFields {
    type SortBy: Default + ToString;

    fn sort_by_field(&self) -> &Option<Self::SortBy>;
    fn order_by_field(&self) -> &Option<OrderEnum>;
}
