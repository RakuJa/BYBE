use crate::AppState;
use crate::db::shop_proxy;
use crate::models::response_data::ResponseItem;
use crate::models::routers_validator_structs::{Dice, ItemFieldFilters};
use crate::models::shop_structs::{
    ItemTableFieldsFilter, RandomShopData, ShopFilterQuery, ShopPaginatedRequest, ShopTemplateData,
    ShopTemplateEnum,
};
use crate::services::url_calculator::shop_next_url;
use anyhow::{Context, bail};
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
    let (type_filter, rarity_filter) = shop_data.shop_template.clone().map_or_else(
        || {
            (
                shop_data.type_filter.clone().unwrap_or_default(),
                shop_data.rarity_filter.clone().unwrap_or_default(),
            )
        },
        |x| (x.get_allowed_item_types(), x.get_allowed_item_rarities()),
    );
    let shop_type = shop_data.shop_template.clone().unwrap_or_default();
    let n_of_consumables = i64::from(
        shop_data
            .consumable_dices
            .iter()
            .map(Dice::roll)
            .sum::<u16>(),
    );
    let n_of_equippables = shop_data.equippable_dices.iter().map(Dice::roll).sum();
    // The request is correct, but will result in an empty list.
    if n_of_consumables == 0 && n_of_equippables == 0 {
        return ShopListingResponse::default();
    }

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
                shop_type.get_equippable_percentages()
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
        (shop_proxy::get_filtered_items(
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
        .await)
            .map_or_else(
                |_| ShopListingResponse::default(),
                |result| {
                    let n_of_items = result.len();
                    ShopListingResponse {
                        results: Some(result.into_iter().map(ResponseItem::from).collect()),
                        count: n_of_items,
                        next: None,
                        total: n_of_items,
                    }
                },
            )
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

pub fn get_shop_templates_data() -> Vec<ShopTemplateData> {
    ShopTemplateEnum::iter()
        .map(std::convert::Into::into)
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
                next: if n_of_items
                    >= pagination.paginated_request.page_size.unsigned_abs() as usize
                {
                    Some(shop_next_url(field_filters, pagination, n_of_items as u32))
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
    n_of_equippables: u16,
    percentages: (u8, u8, u8, u8),
) -> anyhow::Result<(i64, i64, i64, i64)> {
    let (e_p, w_p, a_p, s_p) = percentages;
    let sum_of_percentages = f64::from(e_p + w_p + a_p + s_p);
    if sum_of_percentages > 100. {
        bail!("Percentages sum value is higher than 100. Incorrect values.")
    }
    let f_n_of_equippables = f64::from(n_of_equippables);
    let (e_v, w_v, a_v, s_v) = if sum_of_percentages == 0. {
        (25., 25., 25., 25.)
    } else {
        (
            //Simpler form: (f_n_of_equippables * ((w_p as f64 * 100.) / sum_of_percentages)) / 100.,
            ((f_n_of_equippables * f64::from(e_p)) / sum_of_percentages).floor(),
            ((f_n_of_equippables * f64::from(w_p)) / sum_of_percentages).floor(),
            ((f_n_of_equippables * f64::from(a_p)) / sum_of_percentages).floor(),
            ((f_n_of_equippables * f64::from(s_p)) / sum_of_percentages).floor(),
        )
    };
    let missing = f_n_of_equippables - (e_v + w_v + a_v + s_v);
    let distributed = order_distribution(divide_equally(missing), percentages);
    Ok((
        (e_v + distributed.0)
            .floor()
            .to_i64()
            .context("Error converting v to i64")?,
        (w_v + distributed.1)
            .floor()
            .to_i64()
            .context("Error converting v to i64")?,
        (a_v + distributed.2)
            .floor()
            .to_i64()
            .context("Error converting v to i64")?,
        (s_v + distributed.3)
            .floor()
            .to_i64()
            .context("Error converting v to i64")?,
    ))
}

///
/// Returns the given `to_distribute` tuple ordered following `og_percentages`
/// ```Rust
/// assert_eq!(
///     (3.0, 2.0, 1.0, 4.0),
///     order_distribution((4., 2., 3., 1.), (20,20,20,40)
/// )
/// ```
fn order_distribution(
    to_distribute: (f64, f64, f64, f64),
    og_percentages: (u8, u8, u8, u8),
) -> (f64, f64, f64, f64) {
    let to_order = to_distribute;
    let by = og_percentages;

    let mut indices_by: Vec<(usize, f64)> = vec![
        (0, f64::from(by.0)),
        (1, f64::from(by.1)),
        (2, f64::from(by.2)),
        (3, f64::from(by.3)),
    ];

    indices_by.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let mut values: [f64; 4] = to_order.into();
    values.sort_by(|a, b| b.partial_cmp(a).unwrap());

    let mut result = [0.0; 4];
    for (i, (idx, _)) in indices_by.iter().enumerate() {
        result[*idx] = values[i];
    }

    result.into()
}

///
/// Returns a tuple of 4 elements that divide equally (as integer) `f` from left to right
/// e.g.
/// ```Rust
/// assert_eq!(divide_equally(3.), (1,1,1,0))
/// assert_eq!(divide_equally(4.), (1,1,1,1))
/// assert_eq!(divide_equally(5.), (2,1,1,1))
/// assert_eq!(divide_equally(6.), (2,2,1,1))
/// ```
fn divide_equally(f: f64) -> (f64, f64, f64, f64) {
    f.to_usize()
        .map_or([0.; 4], |n| {
            let base = n / 4;
            let remainder = n % 4;

            let mut result = [base as f64; 4];
            result.iter_mut().take(remainder).for_each(|x| *x += 1.);
            result
        })
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(10, (10,10,10,10), (3,3,2,2))]
    #[case(1, (10,10,10,10), (1,0,0,0))]
    #[case(8, (20,20,20,10), (3,2,2,1))]
    #[case(8, (10,20,20,20), (1,3,2,2))]
    #[case(8, (10,20,20,30), (1,2,2,3))]
    #[case(8, (20,20,30,10), (2,2,3,1))]
    fn calculate_equippable_values_rounded_over_desired_total_case(
        #[case] input_n_of_equippables: u16,
        #[case] input_percentages: (u8, u8, u8, u8),
        #[case] expected: (i64, i64, i64, i64),
    ) {
        let result = calculate_n_of_equippable_values(input_n_of_equippables, input_percentages);
        assert!(result.is_ok());
        assert_eq!(expected, result.unwrap());
    }

    #[rstest]
    #[case(0, (0,0,0,0), (25, 25, 25, 25))]
    fn calculate_equippable_values_with_all_0(
        #[case] input_n_of_equippables: u16,
        #[case] input_percentages: (u8, u8, u8, u8),
        #[case] expected: (i64, i64, i64, i64),
    ) {
        let result = calculate_n_of_equippable_values(input_n_of_equippables, input_percentages);
        assert!(result.is_ok());
        assert_eq!(expected, result.unwrap());
    }

    #[rstest]
    #[case(0, (10,10,10,10), (0,0,0,0))]
    #[case(0, (10,20,10,0), (0,0,0,0))]
    fn calculate_equippable_values_zero_as_n_of_equippables(
        #[case] input_n_of_equippables: u16,
        #[case] input_percentages: (u8, u8, u8, u8),
        #[case] expected: (i64, i64, i64, i64),
    ) {
        let result = calculate_n_of_equippable_values(input_n_of_equippables, input_percentages);
        assert!(result.is_ok());
        assert_eq!(expected, result.unwrap());
    }

    #[rstest]
    #[case(10, (10,0,0,0), (10,0,0,0))]
    #[case(10, (10,10,0,0), (5,5,0,0))]
    #[case(10, (10,10,10,0), (4,3,3,0))]
    fn calculate_equippable_values_with_missing_percentages(
        #[case] input_n_of_equippables: u16,
        #[case] input_percentages: (u8, u8, u8, u8),
        #[case] expected: (i64, i64, i64, i64),
    ) {
        let result = calculate_n_of_equippable_values(input_n_of_equippables, input_percentages);
        assert!(result.is_ok());
        assert_eq!(expected, result.unwrap());
    }

    #[rstest]
    #[case(4., (1.,1.,1.,1.))]
    #[case(8., (2.,2.,2.,2.))]
    #[case(12., (3.,3.,3.,3.))]
    #[case(16., (4.,4.,4.,4.))]
    #[case(20., (5.,5.,5.,5.))]
    #[case(24., (6.,6.,6.,6.))]
    fn divide_equally_multiple_of_four(
        #[case] to_distribute: f64,
        #[case] expected: (f64, f64, f64, f64),
    ) {
        let result = divide_equally(to_distribute);
        assert_eq!(expected, result);
    }

    #[rstest]
    #[case(1., (1.,0.,0.,0.))]
    #[case(2., (1.,1.,0.,0.))]
    #[case(3., (1.,1.,1.,0.))]
    #[case(5., (2.,1.,1.,1.))]
    #[case(6., (2.,2.,1.,1.))]
    #[case(7., (2.,2.,2.,1.))]
    #[case(9., (3.,2.,2.,2.))]
    #[case(13., (4.,3.,3.,3.))]
    #[case(17., (5.,4.,4.,4.))]
    #[case(21., (6.,5.,5.,5.))]
    #[case(25., (7.,6.,6.,6.))]
    fn divide_equally_not_multiple_of_four(
        #[case] to_distribute: f64,
        #[case] expected: (f64, f64, f64, f64),
    ) {
        let result = divide_equally(to_distribute);
        assert_eq!(expected, result);
    }

    #[rstest]
    #[case((4.0, 2.0, 3.0, 1.0), (20, 20, 20, 40), (3.0, 2.0, 1.0, 4.0))]
    #[case((9.0, 7.0, 8.0, 6.0), (20, 10, 20, 10), (9.0, 7.0, 8.0, 6.0))]
    #[case((1.0, 2.0, 3.0, 4.0), (10, 20, 30, 40), (1.0, 2.0, 3.0, 4.0))] // already ordered
    #[case((4.0, 3.0, 2.0, 1.0), (40, 30, 20, 10), (4.0, 3.0, 2.0, 1.0))] // descending by weight
    #[case((5.0, 6.0, 7.0, 8.0), (1, 1, 1, 1), (8.0, 7.0, 6.0, 5.0))] // equal weights, to_order sorted
    #[case((10.0, 20.0, 30.0, 40.0), (5, 10, 5, 20), (20.0, 30.0, 10.0, 40.0))]
    #[case((1.5, 3.3, 2.2, 4.4), (3, 1, 4, 2), (3.3, 1.5, 4.4, 2.2))] // mixed values
    #[case((10.0, 10.0, 10.0, 10.0), (4, 3, 2, 1), (10.0, 10.0, 10.0, 10.0))] // identical values
    #[case((1.0, 2.0, 3.0, 4.0), (0, 0, 100, 100), (2.0, 1.0, 4.0, 3.0))] // tie on highest weights
    fn order_mixed_values(
        #[case] to_distribute: (f64, f64, f64, f64),
        #[case] by: (u8, u8, u8, u8),
        #[case] expected: (f64, f64, f64, f64),
    ) {
        let result = order_distribution(to_distribute, by);
        assert_eq!(expected, result);
    }
}
