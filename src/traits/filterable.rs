pub trait Filterable {
    type FilterImpl: Default;

    fn is_passing_filters(&self, filters: &Self::FilterImpl) -> bool {
        self.does_it_pass_equality_filters(filters)
            && self.does_it_pass_lb_filters(filters)
            && self.does_it_pass_ub_filters(filters)
            && self.does_it_pass_string_filters(filters)
    }
    fn does_it_pass_ub_filters(&self, filters: &Self::FilterImpl) -> bool;
    fn does_it_pass_lb_filters(&self, filters: &Self::FilterImpl) -> bool;
    fn does_it_pass_string_filters(&self, filters: &Self::FilterImpl) -> bool;

    fn does_it_pass_equality_filters(&self, filters: &Self::FilterImpl) -> bool;
}
