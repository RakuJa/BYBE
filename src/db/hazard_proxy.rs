use crate::AppState;
use crate::db::data_providers::hazard_fetcher::fetch_traits_associated_with_hazards;
use crate::db::data_providers::{generic_fetcher, hazard_fetcher};
use crate::models::hazard::hazard_field_filter::HazardFieldFilters;
use crate::models::hazard::hazard_listing_struct::{
    HazardFilterQuery, HazardListingPaginatedRequest, HazardSortEnum,
};
use crate::models::hazard::hazard_struct::{Hazard, HazardRanges};
use crate::models::response_data::ResponseHazard;
use crate::models::routers_validator_structs::OrderEnum;
use crate::models::shared::game_system_enum::GameSystem;
use crate::traits::filterable::Filterable;
use anyhow::Result;
use cached::proc_macro::cached;

pub async fn get_hazard_by_id(
    app_state: &AppState,
    gs: GameSystem,
    id: i64,
) -> Option<ResponseHazard> {
    hazard_fetcher::fetch_hazard_by_id(&app_state.conn, gs, id)
        .await
        .ok()
}

pub async fn get_hazards_passing_all_filters(
    app_state: &AppState,
    gs: GameSystem,
    filters: &HazardFilterQuery,
) -> Result<Vec<Hazard>> {
    hazard_fetcher::fetch_hazard_core_data_with_filters(&app_state.conn, gs, filters).await
}

pub async fn get_paginated_hazards(
    app_state: &AppState,
    gs: GameSystem,
    filters: &HazardFieldFilters,
    pagination: &HazardListingPaginatedRequest,
) -> Result<(u32, Vec<ResponseHazard>)> {
    let mut filtered_list: Vec<ResponseHazard> = get_list(app_state, gs)
        .await
        .iter()
        .map(|x| ResponseHazard {
            core_hazard: x.clone(),
            game: gs,
        })
        .filter(|x| Hazard::is_passing_filters(&x.core_hazard, filters))
        .collect();

    let total_item_count = filtered_list.len();

    filtered_list.sort_by(|a, b| {
        let cmp = match pagination
            .hazard_sort_data
            .sort_by
            .clone()
            .unwrap_or_default()
        {
            HazardSortEnum::Id => a.core_hazard.essential.id.cmp(&b.core_hazard.essential.id),
            HazardSortEnum::Name => a
                .core_hazard
                .essential
                .name
                .cmp(&b.core_hazard.essential.name),
            HazardSortEnum::Ac => a.core_hazard.essential.ac.cmp(&b.core_hazard.essential.ac),
            HazardSortEnum::Hardness => a
                .core_hazard
                .essential
                .hardness
                .cmp(&b.core_hazard.essential.hardness),
            HazardSortEnum::Hp => a.core_hazard.essential.hp.cmp(&b.core_hazard.essential.hp),
            HazardSortEnum::Complexity => a
                .core_hazard
                .essential
                .complexity
                .cmp(&b.core_hazard.essential.complexity),
            HazardSortEnum::Level => a
                .core_hazard
                .essential
                .level
                .cmp(&b.core_hazard.essential.level),
            HazardSortEnum::Trait => a.core_hazard.traits.cmp(&b.core_hazard.traits),
            HazardSortEnum::Rarity => a
                .core_hazard
                .essential
                .rarity
                .cmp(&b.core_hazard.essential.rarity),
            HazardSortEnum::Size => a
                .core_hazard
                .essential
                .size
                .cmp(&b.core_hazard.essential.size),
            HazardSortEnum::Source => a
                .core_hazard
                .essential
                .source
                .cmp(&b.core_hazard.essential.source),
            HazardSortEnum::Fortitude => a
                .core_hazard
                .essential
                .fortitude
                .cmp(&b.core_hazard.essential.fortitude),
            HazardSortEnum::Reflex => a
                .core_hazard
                .essential
                .reflex
                .cmp(&b.core_hazard.essential.reflex),
            HazardSortEnum::Will => a
                .core_hazard
                .essential
                .will
                .cmp(&b.core_hazard.essential.will),
            HazardSortEnum::Stealth => a
                .core_hazard
                .essential
                .stealth
                .cmp(&b.core_hazard.essential.stealth),
        };
        match pagination.hazard_sort_data.order_by.unwrap_or_default() {
            OrderEnum::Ascending => cmp,
            OrderEnum::Descending => cmp.reverse(),
        }
    });

    let curr_slice: Vec<ResponseHazard> = filtered_list
        .iter()
        .skip(pagination.paginated_request.cursor as usize)
        .take(if pagination.paginated_request.page_size >= 0 {
            pagination.paginated_request.page_size.unsigned_abs() as usize
        } else {
            usize::MAX
        })
        .cloned()
        .collect();

    Ok((total_item_count as u32, curr_slice))
}

/// Infallible method, it will expose a vector representing the values fetched from db or empty vec
#[cached(key = "i64", convert = r##"{ gs.into() }"##)]
async fn get_list(app_state: &AppState, gs: GameSystem) -> Vec<Hazard> {
    hazard_fetcher::fetch_hazards_data(&app_state.conn, gs, 0, -1)
        .await
        .unwrap_or_default()
}

/// Gets all the runtime sources. It will cache the result
#[cached(key = "i64", convert = r##"{ gs.into() }"##)]
pub async fn get_all_sources(app_state: &AppState, gs: GameSystem) -> Vec<String> {
    generic_fetcher::fetch_unique_values_of_field(
        &app_state.conn,
        format!("{gs}_hazard_table").as_str(),
        "source",
    )
    .await
    .unwrap_or_default()
}

/// Gets all the runtime sources. It will cache the result
#[cached(key = "i64", convert = r##"{ gs.into() }"##)]
pub async fn get_all_rarities(app_state: &AppState, gs: GameSystem) -> Vec<String> {
    generic_fetcher::fetch_unique_values_of_field(
        &app_state.conn,
        format!("{gs}_hazard_table").as_str(),
        "rarity",
    )
    .await
    .unwrap_or_default()
}

/// Gets all the runtime sources. It will cache the result
#[cached(key = "i64", convert = r##"{ gs.into() }"##)]
pub async fn get_all_sizes(app_state: &AppState, gs: GameSystem) -> Vec<String> {
    generic_fetcher::fetch_unique_values_of_field(
        &app_state.conn,
        format!("{gs}_hazard_table").as_str(),
        "size",
    )
    .await
    .unwrap_or_default()
}

/// Gets all the runtime traits. It will cache the result
#[cached(key = "i64", convert = r##"{ gs.into() }"##)]
pub async fn get_all_traits(app_state: &AppState, gs: GameSystem) -> Vec<String> {
    fetch_traits_associated_with_hazards(&app_state.conn, gs)
        .await
        .unwrap_or_default()
}

pub async fn get_hazard_ranges(app_state: &AppState, gs: GameSystem) -> Result<HazardRanges> {
    Ok(get_list(app_state, gs)
        .await
        .iter()
        .fold(HazardRanges::default(), |mut acc, x| {
            let e = &x.essential;
            acc.min_ac = acc.min_ac.min(e.ac);
            acc.max_ac = acc.max_ac.max(e.ac);
            acc.min_hardness = acc.min_hardness.min(e.hardness);
            acc.max_hardness = acc.max_hardness.max(e.hardness);
            acc.min_hp = acc.min_hp.min(e.hp);
            acc.max_hp = acc.max_hp.max(e.hp);
            acc.min_stealth = acc.min_stealth.min(e.stealth);
            acc.max_stealth = acc.max_stealth.max(e.stealth);
            acc.min_level = acc.min_level.min(e.level);
            acc.max_level = acc.max_level.max(e.level);
            if let Some(will) = e.will {
                acc.min_will = acc.min_will.min(will);
                acc.max_will = acc.max_will.max(will);
            }
            if let Some(reflex) = e.reflex {
                acc.min_reflex = acc.min_reflex.min(reflex);
                acc.max_reflex = acc.max_reflex.max(reflex);
            }
            if let Some(fortitude) = e.fortitude {
                acc.min_fortitude = acc.min_fortitude.min(fortitude);
                acc.max_fortitude = acc.max_fortitude.max(fortitude);
            }
            acc
        }))
}
