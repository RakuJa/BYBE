use crate::traits::url::paginated_request_ext::PaginatedRequestExt;
use cached::proc_macro::cached;
use std::env;

#[cached]
fn get_website_url() -> String {
    env::var("BACKEND_URL").expect("Error fetching backend URL")
}

pub fn next_url<T: PaginatedRequestExt>(pagination: &T, next_cursor: u32) -> String {
    let base_url = format!("{}/{}/", get_website_url(), T::base_path());
    let pagination_query = prepare_pagination_path(
        next_cursor,
        pagination.page_size(),
        &pagination.sort_by(),
        &pagination.order_by(),
    );
    format!("{base_url}{pagination_query}")
}

fn prepare_pagination_path(
    next_cursor: u32,
    page_size: i16,
    sort_by_key: &str,
    order_by_key: &str,
) -> String {
    format!(
        "&cursor={}&page_size={}&sort_by={}&order_by{}",
        next_cursor, page_size, sort_by_key, order_by_key
    )
}

pub fn add_boolean_query(url: Option<&String>, key: &String, value: bool) -> Option<String> {
    url.as_ref()
        .map(|base_url| format!("{base_url}&{key}={value}"))
}
