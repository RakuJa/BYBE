use crate::models::bestiary_structs::BestiaryPaginatedRequest;
use crate::models::routers_validator_structs::{CreatureFieldFilters, ItemFieldFilters};
use crate::models::shop_structs::ShopPaginatedRequest;

pub fn shop_next_url_calculator(
    field_filters: &ItemFieldFilters,
    pagination: &ShopPaginatedRequest,
    next_cursor: u32,
) -> String {
    let base_url = "https://backbybe.fly.dev/shop/list/";
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
    format!("{}{}{}", base_url, filter_query, pagination_query)
}

pub fn bestiary_next_url_calculator(
    field_filters: &CreatureFieldFilters,
    pagination: &BestiaryPaginatedRequest,
    next_cursor: u32,
) -> String {
    let base_url = "https://backbybe.fly.dev/bestiary/list/";
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
    format!("{}{}{}", base_url, filter_query, pagination_query)
}

pub fn add_boolean_query(url: &Option<String>, key: &String, value: bool) -> Option<String> {
    url.as_ref()
        .map(|base_url| format!("{}&{}={}", base_url, key, value))
}

fn creature_filter_query_calculator(field_filters: &CreatureFieldFilters) -> String {
    let queries: Vec<String> = [
        field_filters
            .name_filter
            .clone()
            .map(|name| format!("name_filter={}", name)),
        field_filters
            .role_threshold
            .map(|threshold| format!("role_threshold={}", threshold)),
        field_filters
            .min_hp_filter
            .map(|hp| format!("min_hp_filter={}", hp)),
        field_filters
            .max_hp_filter
            .map(|hp| format!("max_hp_filter={}", hp)),
        field_filters
            .min_level_filter
            .map(|lvl| format!("min_level_filter={}", lvl)),
        field_filters
            .max_level_filter
            .map(|lvl| format!("max_level_filter={}", lvl)),
        field_filters
            .is_melee_filter
            .map(|is| format!("is_melee_filter={}", is)),
        field_filters
            .is_ranged_filter
            .map(|is| format!("is_ranged_filter={}", is)),
        field_filters
            .is_spell_caster_filter
            .map(|is| format!("is_spell_caster_filter={}", is)),
    ]
    .iter()
    .filter_map(|opt| opt.clone())
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
            .map(|bulk| format!("min_bulk_filter={}", bulk)),
        field_filters
            .max_bulk_filter
            .map(|bulk| format!("max_bulk_filter={}", bulk)),
        field_filters
            .min_hardness_filter
            .map(|hn| format!("min_hardness_filter={}", hn)),
        field_filters
            .max_hardness_filter
            .map(|hn| format!("max_hardness_filter={}", hn)),
        field_filters
            .min_hp_filter
            .map(|hp| format!("min_hp_filter={}", hp)),
        field_filters
            .max_hp_filter
            .map(|hp| format!("max_hp_filter={}", hp)),
        field_filters
            .min_level_filter
            .map(|lvl| format!("min_level_filter={}", lvl)),
        field_filters
            .max_level_filter
            .map(|lvl| format!("max_level_filter={}", lvl)),
        field_filters
            .min_price_filter
            .map(|price| format!("min_price_filter={}", price)),
        field_filters
            .max_price_filter
            .map(|price| format!("max_price_filter={}", price)),
        field_filters
            .min_n_of_uses_filter
            .map(|uses| format!("min_n_of_uses_filter={}", uses)),
        field_filters
            .max_n_of_uses_filter
            .map(|uses| format!("max_n_of_uses_filter={}", uses)),
    ]
    .iter()
    .filter_map(|opt| opt.clone())
    .collect();
    match queries.len() {
        0 => String::new(),
        _ => format!("&{}", queries.join("&")),
    }
}
