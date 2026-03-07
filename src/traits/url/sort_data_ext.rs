use crate::traits::url::has_sort_fields::HasSortFields;

pub trait SortDataExt {
    fn sort_by(&self) -> String;
    fn order_by(&self) -> String;
}

impl<T: HasSortFields> SortDataExt for T {
    fn sort_by(&self) -> String {
        self.sort_by_field()
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_default()
    }
    fn order_by(&self) -> String {
        self.order_by_field()
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_default()
    }
}
