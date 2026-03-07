pub trait ListingResponse: Default {
    type Item;

    fn from_results(
        results: Vec<Self::Item>,
        count: usize,
        next: Option<String>,
        total: usize,
    ) -> Self;
}
