use crate::models::routers_validator_structs::{FieldFilters, PaginatedRequest, SortData};

pub fn next_url_calculator(
    sort_field: &SortData,
    field_filters: &FieldFilters,
    pagination: &PaginatedRequest,
    next_cursor: u32,
) -> String {
    let base_url = "https://bybe.fly.dev/bestiary/list/"; //"0.0.0.0:25566/list/"
    let sort_query = format!(
        "?sort_key={}&order_by={}",
        sort_field.sort_key.unwrap_or_default(),
        sort_field.order_by.unwrap_or_default()
    );
    let filter_query = filter_query_calculator(field_filters);

    let pagination_query = format!("&cursor={}&page_size={}", next_cursor, pagination.page_size);
    format!(
        "{}{}{}{}",
        base_url, sort_query, filter_query, pagination_query
    )
}

pub fn generate_archive_link(id: i32) -> String {
    format!("https://2e.aonprd.com/Monsters.aspx?ID={}", id)
}

pub fn add_boolean_query(url: &str, key: &String, value: bool) -> String {
    let mut x = url.to_string();
    x.push_str(&format!("&{}={}", key, value));
    x
}

fn filter_query_calculator(field_filters: &FieldFilters) -> String {
    let queries: Vec<String> = [
        field_filters
            .family_filter
            .clone()
            .map(|fam| format!("family_filter={}", fam)),
        field_filters
            .name_filter
            .clone()
            .map(|name| format!("name_filter={}", name)),
        field_filters
            .rarity_filter
            .clone()
            .map(|rar| format!("rarity_filter={}", rar)),
        field_filters
            .size_filter
            .clone()
            .map(|size| format!("size_filter={}", size)),
        field_filters
            .alignment_filter
            .clone()
            .map(|align| format!("alignment_filter={}", align)),
        field_filters
            .min_hp_filter
            .map(|hp| format!("min_hp_filter={}", hp)),
        field_filters
            .max_hp_filter
            .map(|hp| format!("max_hp_filter={}", hp)),
        field_filters
            .min_hp_filter
            .map(|lvl| format!("min_level_filter={}", lvl)),
        field_filters
            .max_hp_filter
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
        _ => format!("{}{}", "&", queries.join("&")),
    }
}
