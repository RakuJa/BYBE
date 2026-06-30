pub mod creature_fetcher;
pub mod generic_fetcher;
pub mod hazard_fetcher;
mod raw_query_builder;
pub mod shop_fetcher;

/// Fetch distinct values of a single DB field, returning from the query cache when possible.
pub(crate) async fn fetch_unique_values_from_db(
    app_state: &crate::AppState,
    table: String,
    field: String,
) -> Vec<String> {
    generic_fetcher::fetch_unique_values_of_field(&app_state.pool, &table, &field)
        .await
        .unwrap_or_default()
}
