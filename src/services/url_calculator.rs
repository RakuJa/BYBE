use crate::models::bestiary_structs::BestiaryPaginatedRequest;
use crate::models::creature::creature_field_filter::CreatureFieldFilters;
use crate::models::hazard::hazard_field_filter::HazardFieldFilters;
use crate::models::hazard::hazard_listing_struct::HazardListingPaginatedRequest;
use crate::models::item::item_field_filter::ItemFieldFilters;
use crate::models::item::shop_structs::ShopPaginatedRequest;
use cached::proc_macro::cached;
use std::env;

#[cached]
fn get_website_url() -> String {
    env::var("BACKEND_URL").expect("Error fetching backend URL")
}

pub fn shop_next_url(
    field_filters: &ItemFieldFilters,
    pagination: &ShopPaginatedRequest,
    next_cursor: u32,
) -> String {
    let base_url = format!("{}/shop/list/", get_website_url());
    let filter_query = shop_filter_query_calculator(field_filters);

    let pagination_query = format!(
        "&cursor={}&page_size={}&sort_by={}&order_by={}",
        next_cursor,
        pagination.paginated_request.page_size,
        pagination
            .shop_sort_data
            .sort_by
            .clone()
            .unwrap_or_default(),
        pagination
            .shop_sort_data
            .order_by
            .clone()
            .unwrap_or_default()
    );
    format!("{base_url}{filter_query}{pagination_query}")
}

pub fn bestiary_next_url(
    field_filters: &CreatureFieldFilters,
    pagination: &BestiaryPaginatedRequest,
    next_cursor: u32,
) -> String {
    let base_url = format!("{}/bestiary/list/", get_website_url());
    let filter_query = creature_filter_query_calculator(field_filters);

    let pagination_query = format!(
        "&cursor={}&page_size={}&sort_by={}&order_by={}",
        next_cursor,
        pagination.paginated_request.page_size,
        pagination
            .bestiary_sort_data
            .sort_by
            .clone()
            .unwrap_or_default(),
        pagination
            .bestiary_sort_data
            .order_by
            .clone()
            .unwrap_or_default()
    );
    format!("{base_url}{filter_query}{pagination_query}")
}

pub fn hazard_listing_next_url(
    _field_filters: &HazardFieldFilters,
    _pagination: &HazardListingPaginatedRequest,
    _next_cursor: u32,
) -> String {
    let base_url = format!("{}/hazards/list/", get_website_url());
    base_url
}

pub fn add_boolean_query(url: Option<&String>, key: &String, value: bool) -> Option<String> {
    url.as_ref()
        .map(|base_url| format!("{base_url}&{key}={value}"))
}

fn creature_filter_query_calculator(field_filters: &CreatureFieldFilters) -> String {
    let queries: Vec<String> = [
        field_filters
            .name_filter
            .clone()
            .map(|name| format!("name_filter={name}")),
        field_filters
            .role_threshold
            .map(|threshold| format!("role_threshold={threshold}")),
        field_filters
            .min_hp_filter
            .map(|hp| format!("min_hp_filter={hp}")),
        field_filters
            .max_hp_filter
            .map(|hp| format!("max_hp_filter={hp}")),
        field_filters
            .min_level_filter
            .map(|lvl| format!("min_level_filter={lvl}")),
        field_filters
            .max_level_filter
            .map(|lvl| format!("max_level_filter={lvl}")),
    ]
    .iter()
    .filter_map(std::clone::Clone::clone)
    .collect();
    match queries.len() {
        0 => String::new(),
        _ => format!("&{}", queries.join("&")),
    }
}

fn shop_filter_query_calculator(field_filters: &ItemFieldFilters) -> String {
    let queries: Vec<String> = [
        field_filters
            .min_bulk_filter
            .map(|bulk| format!("min_bulk_filter={bulk}")),
        field_filters
            .max_bulk_filter
            .map(|bulk| format!("max_bulk_filter={bulk}")),
        field_filters
            .min_hardness_filter
            .map(|hn| format!("min_hardness_filter={hn}")),
        field_filters
            .max_hardness_filter
            .map(|hn| format!("max_hardness_filter={hn}")),
        field_filters
            .min_hp_filter
            .map(|hp| format!("min_hp_filter={hp}")),
        field_filters
            .max_hp_filter
            .map(|hp| format!("max_hp_filter={hp}")),
        field_filters
            .min_level_filter
            .map(|lvl| format!("min_level_filter={lvl}")),
        field_filters
            .max_level_filter
            .map(|lvl| format!("max_level_filter={lvl}")),
        field_filters
            .min_price_filter
            .map(|price| format!("min_price_filter={price}")),
        field_filters
            .max_price_filter
            .map(|price| format!("max_price_filter={price}")),
        field_filters
            .min_n_of_uses_filter
            .map(|uses| format!("min_n_of_uses_filter={uses}")),
        field_filters
            .max_n_of_uses_filter
            .map(|uses| format!("max_n_of_uses_filter={uses}")),
    ]
    .iter()
    .filter_map(std::clone::Clone::clone)
    .collect();
    match queries.len() {
        0 => String::new(),
        _ => format!("&{}", queries.join("&")),
    }
}
