use crate::db::shop_proxy;
use crate::models::response_data::ResponseItem;
use crate::models::routers_validator_structs::ItemFieldFilters;
use crate::models::shop_structs::{
    ItemTableFieldsFilter, RandomShopData, ShopFilterQuery, ShopPaginatedRequest, ShopTemplateData,
    ShopTemplateEnum,
};
use crate::services::url_calculator::shop_next_url_calculator;
use crate::AppState;
use anyhow::{bail, Context};
use num_traits::ToPrimitive;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum::IntoEnumIterator;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Default)]
pub struct ShopListingResponse {
    results: Option<Vec<ResponseItem>>,
    count: usize,
    total: usize,
    next: Option<String>,
}

pub async fn get_item(app_state: &AppState, id: i64) -> HashMap<String, Option<ResponseItem>> {
    hashmap! {
        String::from("results") =>
        shop_proxy::get_item_by_id(app_state, id).await
    }
}

pub async fn get_shop_listing(
    app_state: &AppState,
    field_filter: &ItemFieldFilters,
    pagination: &ShopPaginatedRequest,
) -> ShopListingResponse {
    convert_result_to_shop_response(
        field_filter,
        pagination,
        shop_proxy::get_paginated_items(app_state, field_filter, pagination).await,
    )
}

pub async fn generate_random_shop_listing(
    app_state: &AppState,
    shop_data: &RandomShopData,
) -> ShopListingResponse {
    let (type_filter, rarity_filter) = if let Some(x) = shop_data.shop_template.clone() {
        (x.to_item_types(), x.to_item_rarities())
    } else {
        (
            shop_data.type_filter.clone().unwrap_or_default(),
            shop_data.rarity_filter.clone().unwrap_or_default(),
        )
    };
    let shop_type = shop_data.shop_template.clone().unwrap_or_default();
    let n_of_consumables: i64 = shop_data.consumable_dices.iter().map(|x| x.roll()).sum();
    let n_of_equippables: i64 = shop_data.equippable_dices.iter().map(|x| x.roll()).sum();

    let equipment_percentage = shop_data.equipment_percentage;
    let weapon_percentage = shop_data.weapon_percentage;
    let armor_percentage = shop_data.armor_percentage;
    let shield_percentage = shop_data.shield_percentage;

    if let Ok((n_of_equipment, n_of_weapons, n_of_armors, n_of_shields)) =
        calculate_n_of_equippable_values(
            n_of_equippables,
            if equipment_percentage.is_none()
                && weapon_percentage.is_none()
                && armor_percentage.is_none()
                && shield_percentage.is_none()
            {
                shop_type.to_equippable_percentages()
            } else {
                (
                    equipment_percentage.unwrap_or(0),
                    weapon_percentage.unwrap_or(0),
                    armor_percentage.unwrap_or(0),
                    shield_percentage.unwrap_or(0),
                )
            },
        )
    {
        match shop_proxy::get_filtered_items(
            app_state,
            &ShopFilterQuery {
                item_table_fields_filter: ItemTableFieldsFilter {
                    category_filter: shop_data.category_filter.clone().unwrap_or_default(),
                    source_filter: shop_data.source_filter.clone().unwrap_or_default(),
                    type_filter,
                    rarity_filter,
                    size_filter: shop_data.size_filter.clone().unwrap_or_default(),
                    min_level: shop_data.min_level.unwrap_or(0),
                    max_level: shop_data.max_level.unwrap_or(30),
                    supported_version: shop_data
                        .pathfinder_version
                        .clone()
                        .unwrap_or_default()
                        .to_db_value(),
                },
                trait_whitelist_filter: shop_data
                    .trait_whitelist_filter
                    .clone()
                    .unwrap_or_default(),
                trait_blacklist_filter: shop_data
                    .trait_blacklist_filter
                    .clone()
                    .unwrap_or_default(),
                n_of_equipment,
                n_of_consumables,
                n_of_weapons,
                n_of_armors,
                n_of_shields,
            },
        )
        .await
        {
            Ok(result) => {
                let n_of_items = result.len();
                ShopListingResponse {
                    results: Some(result.into_iter().map(ResponseItem::from).collect()),
                    count: n_of_items,
                    next: None,
                    total: n_of_items,
                }
            }
            Err(_) => ShopListingResponse::default(),
        }
    } else {
        ShopListingResponse::default()
    }
}

pub async fn get_sources_list(app_state: &AppState) -> Vec<String> {
    shop_proxy::get_all_sources(app_state).await
}

pub async fn get_traits_list(app_state: &AppState) -> Vec<String> {
    shop_proxy::get_all_traits(app_state).await
}

pub async fn get_shop_templates_data() -> HashMap<ShopTemplateEnum, ShopTemplateData> {
    ShopTemplateEnum::iter()
        .map(|shop_template| (shop_template.clone(), shop_template.into()))
        .collect()
}

fn convert_result_to_shop_response(
    field_filters: &ItemFieldFilters,
    pagination: &ShopPaginatedRequest,
    result: anyhow::Result<(u32, Vec<ResponseItem>)>,
) -> ShopListingResponse {
    match result {
        Ok(res) => {
            let item: Vec<ResponseItem> = res.1;
            let n_of_items = item.len();
            ShopListingResponse {
                results: Some(item),
                count: n_of_items,
                next: if n_of_items >= pagination.paginated_request.page_size as usize {
                    Some(shop_next_url_calculator(
                        field_filters,
                        pagination,
                        n_of_items as u32,
                    ))
                } else {
                    None
                },
                total: res.0 as usize,
            }
        }
        Err(_) => ShopListingResponse::default(),
    }
}

/// Gets the n of: equipment, weapons, armors, shields (in this order).
/// Changing order is considered a BREAKING CHANGE.
pub fn calculate_n_of_equippable_values(
    n_of_equippables: i64,
    percentages: (u8, u8, u8, u8),
) -> anyhow::Result<(i64, i64, i64, i64)> {
    let (e_p, w_p, a_p, s_p) = percentages;
    let sum_of_percentages = (e_p + w_p + a_p + s_p) as f64;
    if sum_of_percentages > 100. {
        bail!("Percentages sum value is higher than 100. Incorrect values.")
    }
    let f_n_of_equippables = n_of_equippables as f64;
    let (e_v, w_v, a_v, s_v) = (
        //Simpler form: (f_n_of_equippables * ((w_p as f64 * 100.) / sum_of_percentages)) / 100.,
        (f_n_of_equippables * e_p as f64) / sum_of_percentages,
        (f_n_of_equippables * w_p as f64) / sum_of_percentages,
        (f_n_of_equippables * a_p as f64) / sum_of_percentages,
        (f_n_of_equippables * s_p as f64) / sum_of_percentages,
    );

    Ok((
        e_v.ceil().to_i64().context("Error converting v to i64")?,
        w_v.ceil().to_i64().context("Error converting v to i64")?,
        a_v.ceil().to_i64().context("Error converting v to i64")?,
        s_v.ceil().to_i64().context("Error converting v to i64")?,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(10, (10,10,10,10), (3,3,3,3))]
    #[case(1, (10,10,10,10), (1,1,1,1))]
    fn calculate_equippable_values_rounded_over_desired_total_case(
        #[case] input_n_of_equippables: i64,
        #[case] input_percentages: (u8, u8, u8, u8),
        #[case] expected: (i64, i64, i64, i64),
    ) {
        let result = calculate_n_of_equippable_values(input_n_of_equippables, input_percentages);
        assert_eq!(true, result.is_ok());
        assert_eq!(expected, result.unwrap());
    }

    #[rstest]
    #[case(0, (0,0,0,0), true)]
    fn calculate_equippable_values_check_is_err(
        #[case] input_n_of_equippables: i64,
        #[case] input_percentages: (u8, u8, u8, u8),
        #[case] expected: bool,
    ) {
        let result = calculate_n_of_equippable_values(input_n_of_equippables, input_percentages);
        assert_eq!(expected, result.is_err());
    }

    #[rstest]
    #[case(0, (10,10,10,10), (0,0,0,0))]
    #[case(0, (10,20,10,0), (0,0,0,0))]
    fn calculate_equippable_values_zero_as_n_of_equippables(
        #[case] input_n_of_equippables: i64,
        #[case] input_percentages: (u8, u8, u8, u8),
        #[case] expected: (i64, i64, i64, i64),
    ) {
        let result = calculate_n_of_equippable_values(input_n_of_equippables, input_percentages);
        assert_eq!(true, result.is_ok());
        assert_eq!(expected, result.unwrap());
    }

    #[rstest]
    #[case(10, (10,0,0,0), (10,0,0,0))]
    #[case(10, (10,10,0,0), (5,5,0,0))]
    #[case(10, (10,10,10,0), (4,4,4,0))]
    fn calculate_equippable_values_with_missing_percentages(
        #[case] input_n_of_equippables: i64,
        #[case] input_percentages: (u8, u8, u8, u8),
        #[case] expected: (i64, i64, i64, i64),
    ) {
        let result = calculate_n_of_equippable_values(input_n_of_equippables, input_percentages);
        assert_eq!(true, result.is_ok());
        assert_eq!(expected, result.unwrap());
    }
}
