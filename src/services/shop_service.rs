use crate::AppState;
use crate::db::shop_proxy;
use crate::models::item::item_field_filter::ItemFieldFilters;
use crate::models::item::shop_structs::filter_query::ShopFilterQuery;
use crate::models::item::shop_structs::item_table_fields_filter::ItemTableFieldsFilter;
use crate::models::item::shop_structs::item_type_percentages::{
    ConsumablePercentages, EquippablePercentages,
};
use crate::models::item::shop_structs::random_shop_data::schemas::RandomShopData;
use crate::models::item::shop_structs::ranges::ShopRanges;
use crate::models::item::shop_structs::shop_paginated_request::ShopPaginatedRequest;
use crate::models::item::shop_structs::template_data::{
    PfShopTemplateEnum, SfShopTemplateEnum, ShopTemplateData,
};
use crate::models::response_data::{ResponseItem, ShopListingResponse, convert_result_to_response};
use crate::models::routers_validator_structs::Dice;
use crate::models::shared::game_system_enum::GameSystem;
use crate::traits::template_enum::{GenericPercentage, GenericTemplate, ItemTemplate};
use anyhow::{Context, bail};
use num_traits::ToPrimitive;
use std::collections::HashMap;
use strum::IntoEnumIterator;

pub async fn get_item(
    app_state: &AppState,
    id: i64,
    gs: GameSystem,
) -> HashMap<String, Option<ResponseItem>> {
    hashmap! {
        String::from("results") =>
        shop_proxy::get_item_by_id(app_state, gs,  id).await
    }
}

pub async fn get_sources_list(app_state: &AppState, gs: GameSystem) -> Vec<String> {
    shop_proxy::get_all_sources(app_state, gs).await
}

pub async fn get_traits_list(app_state: &AppState, gs: GameSystem) -> Vec<String> {
    shop_proxy::get_all_traits(app_state, gs).await
}

pub fn get_shop_templates_data(gs: GameSystem) -> Vec<ShopTemplateData> {
    match gs {
        GameSystem::Pathfinder => PfShopTemplateEnum::iter().map(Into::into).collect(),
        GameSystem::Starfinder => SfShopTemplateEnum::iter().map(Into::into).collect(),
    }
}

pub async fn get_shop_listing(
    app_state: &AppState,
    field_filter: &ItemFieldFilters,
    pagination: &ShopPaginatedRequest,
    gs: GameSystem,
) -> ShopListingResponse {
    convert_result_to_response(
        pagination,
        shop_proxy::get_paginated_items(app_state, gs, field_filter, pagination).await,
    )
}

pub async fn get_shop_ranges(app_state: &AppState, gs: GameSystem) -> Option<ShopRanges> {
    shop_proxy::get_shop_ranges(app_state, gs).await
}

pub async fn generate_random_shop_listing<T: GenericTemplate + ItemTemplate>(
    app_state: &AppState,
    shop_data: &RandomShopData<T>,
    gs: GameSystem,
) -> ShopListingResponse {
    let (type_filter, rarity_filter) = shop_data.shop_template.clone().map_or_else(
        || {
            (
                shop_data.type_filter.clone().unwrap_or_default(),
                shop_data.rarity_filter.clone().unwrap_or_default(),
            )
        },
        |x| (x.get_allowed_item_types(), x.get_allowed_rarities()),
    );
    let shop_type = shop_data.shop_template.clone().unwrap_or_default();
    let n_of_consumables = shop_data.consumable_dices.iter().map(Dice::roll).sum();
    let n_of_equippables = shop_data.equippable_dices.iter().map(Dice::roll).sum();
    // The request is correct, but will result in an empty list.
    if n_of_consumables == 0 && n_of_equippables == 0 {
        return ShopListingResponse::default_with_system(gs);
    }

    let equippable_contents = calculate_n_of_equippable_values(
        n_of_equippables,
        &if shop_data.percentages.equippable_percentages.is_empty() {
            shop_type.get_equippable_percentages()
        } else {
            EquippablePercentages::from(shop_data.percentages.equippable_percentages)
        }
        .to_vec(),
    );

    let consumable_contents = calculate_n_of_consumable_values(
        n_of_consumables,
        &if shop_data.percentages.consumable_percentages.is_empty() {
            shop_type.get_consumable_percentages()
        } else {
            ConsumablePercentages::from(shop_data.percentages.consumable_percentages)
        }
        .to_vec(),
    );

    if let Ok(e_contents) = equippable_contents
        && let Ok(c_contents) = consumable_contents
    {
        (shop_proxy::get_filtered_items(
            app_state,
            gs,
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
                        .game_system_version
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
                n_of_equipment: e_contents.n_of_equipment,
                n_of_weapons: e_contents.n_of_weapons,
                n_of_armors: e_contents.n_of_armors,
                n_of_shields: e_contents.n_of_shields,
                n_of_treasures: e_contents.n_of_treasures,
                n_of_backpacks: e_contents.n_of_backpacks,

                n_of_generic_consumables: c_contents.n_of_generics,
                n_of_ammunition: c_contents.n_of_ammunition,
            },
        )
        .await)
            .map_or_else(
                |_| ShopListingResponse::default_with_system(gs),
                |result| {
                    let n_of_items = result.len();
                    ShopListingResponse {
                        results: Some(
                            result
                                .into_iter()
                                .map(|x| ResponseItem::from((x, gs)))
                                .collect(),
                        ),
                        count: n_of_items,
                        next: None,
                        total: n_of_items,
                        game: gs,
                    }
                },
            )
    } else {
        ShopListingResponse::default_with_system(gs)
    }
}
#[derive(Debug, PartialEq, Eq)]
pub struct ConsumableContents {
    pub n_of_generics: i64,
    pub n_of_ammunition: i64,
}

impl TryFrom<Vec<i64>> for ConsumableContents {
    type Error = anyhow::Error;
    fn try_from(value: Vec<i64>) -> Result<Self, Self::Error> {
        if value.len() != 2 {
            bail!(
                "Given vector length should be exactly the number of fields present in the struct"
            );
        } else {
            Ok(Self {
                n_of_generics: value[0],
                n_of_ammunition: value[1],
            })
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct EquippableContents {
    pub n_of_equipment: i64,
    pub n_of_weapons: i64,
    pub n_of_armors: i64,
    pub n_of_shields: i64,
    pub n_of_treasures: i64,
    pub n_of_backpacks: i64,
}

impl TryFrom<Vec<i64>> for EquippableContents {
    type Error = anyhow::Error;
    fn try_from(value: Vec<i64>) -> Result<Self, Self::Error> {
        if value.len() != 6 {
            bail!(
                "Given vector length should be exactly the number of fields present in the struct"
            );
        } else {
            Ok(Self {
                n_of_equipment: value[0],
                n_of_weapons: value[1],
                n_of_armors: value[2],
                n_of_shields: value[3],
                n_of_treasures: value[4],
                n_of_backpacks: value[5],
            })
        }
    }
}

pub fn calculate_n_of_consumable_values(
    n_of_consumables: u32,
    percentage: &[u8],
) -> anyhow::Result<ConsumableContents> {
    calculate_n_of_values(n_of_consumables, percentage)
        .map(|items| ConsumableContents::try_from(items).unwrap())
}

pub fn calculate_n_of_equippable_values(
    n_of_equippables: u32,
    percentages: &[u8],
) -> anyhow::Result<EquippableContents> {
    calculate_n_of_values(n_of_equippables, percentages)
        .map(|items| EquippableContents::try_from(items).unwrap())
}

fn calculate_n_of_values(n_of_equippables: u32, percentages: &[u8]) -> anyhow::Result<Vec<i64>> {
    let n = percentages.len();
    if n == 0 {
        bail!("Must have at least one percentage category");
    }

    let sum_of_percentages = f64::from(percentages.iter().map(|p| u32::from(*p)).sum::<u32>());
    if sum_of_percentages > 100. {
        bail!("Percentages sum value is higher than 100. Incorrect values.")
    }
    let f_n_of_equippables = f64::from(n_of_equippables);

    let base_values: Vec<f64> = if sum_of_percentages == 0. {
        let equal_share = (100. / n as f64).floor();
        vec![equal_share; n]
    } else {
        //Simpler form: (f_n_of_equippables * ((w_p as f64 * 100.) / sum_of_percentages)) / 100.,
        percentages
            .iter()
            .map(|p| ((f_n_of_equippables * f64::from(*p)) / sum_of_percentages).floor())
            .collect()
    };

    let missing = f_n_of_equippables - base_values.iter().sum::<f64>();
    let distributed = order_distribution(&divide_equally(missing, n), percentages);

    base_values
        .iter()
        .zip(distributed.iter())
        .map(|(base, extra)| {
            (base + extra)
                .floor()
                .to_i64()
                .context("Error converting v to i64")
        })
        .collect()
}

/// The category with the highest percentage receives the largest value in
/// `to_distribute`, the category with the second-highest percentage
/// receives the second-largest value, and so on.
///
///
/// ```Rust
/// let p = (10, 10, 10, 30, 5, 5, 5)
/// let percentages = EquippablePercentages::from(p)
/// assert_eq!(
///     (7.0, 6.0, 5.0, 8.0, 4.0, 3.0, 2.0) ,
///     order_distribution((8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0), percentages)
/// ); // e.g. 7.0 is assigned to equipment and so on
/// ```
fn order_distribution(to_distribute: &[f64], order_by: &[u8]) -> Vec<f64> {
    debug_assert_eq!(to_distribute.len(), order_by.len());

    let mut indices_by: Vec<_> = order_by.iter().copied().enumerate().collect();

    indices_by.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let mut values = to_distribute.to_vec();
    values.sort_by(|a, b| b.partial_cmp(a).unwrap());

    let mut result = vec![0.0; to_distribute.len()];
    for (rank, (idx, _)) in indices_by.iter().enumerate() {
        result[*idx] = values[rank];
    }

    result
}

///
/// Returns a tuple of 7 elements that divide equally (as integer) `f` from left to right
/// e.g.
/// ```Rust
/// assert_eq!(divide_equally(3.), (1,1,1,0,0,0,0))
/// assert_eq!(divide_equally(4.), (1,1,1,1,0,0,0))
/// assert_eq!(divide_equally(8.), (2,1,1,1,1,1,1))
/// assert_eq!(divide_equally(9.), (2,2,1,1,1,1,1))
/// ```
fn divide_equally(f: f64, n: usize) -> Vec<f64> {
    let total = f.to_usize().unwrap_or(0);
    let base = total / n;
    let remainder = total % n;

    (0..n)
        .map(|i| {
            if i < remainder {
                (base + 1) as f64
            } else {
                base as f64
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(10, vec![10,10,10,10,0,0,0], vec![3,3,2,2,0,0,0])]
    #[case(1, vec![10,10,10,10,0,0,0], vec![1,0,0,0,0,0,0])]
    #[case(8, vec![20,20,20,10,0,0,0], vec![3,2,2,1,0,0,0])]
    #[case(8, vec![10,20,20,20,0,0,0], vec![1,3,2,2,0,0,0])]
    #[case(8, vec![10,20,20,30,0,0,0], vec![1,2,2,3,0,0,0])]
    #[case(8, vec![20,20,30,10,0,0,0], vec![2,2,3,1,0,0,0])]
    #[case(10, vec![10,10,10,10], vec![3,3,2,2])]
    #[case(1, vec![10,10,10,10], vec![1,0,0,0])]
    #[case(8, vec![20,20,20,10], vec![3,2,2,1])]
    #[case(8, vec![10,20,20,20], vec![1,3,2,2])]
    #[case(8, vec![10,20,20,30], vec![1,2,2,3])]
    #[case(8, vec![20,20,30,10], vec![2,2,3,1])]
    fn calculate_equippable_values_rounded_over_desired_total_case(
        #[case] input_n_of_equippables: u32,
        #[case] input_percentages: Vec<u8>,
        #[case] expected: Vec<i64>,
    ) {
        let result = calculate_n_of_values(input_n_of_equippables, input_percentages.as_slice());
        assert!(result.is_ok());
        assert_eq!(expected, result.unwrap());
    }

    #[rstest]
    #[case(0, vec![0,0,0,0,0,0,0], vec![14, 14, 14, 14, 14, 14 ,14])]
    #[case(0, vec![0,0,0,0], vec![25, 25, 25, 25])]
    fn calculate_equippable_values_with_all_0(
        #[case] input_n_of_equippables: u32,
        #[case] input_percentages: Vec<u8>,
        #[case] expected: Vec<i64>,
    ) {
        let result = calculate_n_of_values(input_n_of_equippables, input_percentages.as_slice());
        assert!(result.is_ok());
        assert_eq!(expected, result.unwrap());
    }

    #[rstest]
    #[case(0, vec![10,10,10,10,10,10,10], vec![0,0,0,0,0,0,0])]
    #[case(0, vec![10,20,10,0,10,10,10], vec![0,0,0,0,0,0,0])]
    #[case(0, vec![10,10,10,10], vec![0,0,0,0])]
    #[case(0, vec![10,20,10,0], vec![0,0,0,0])]
    fn calculate_equippable_values_zero_as_n_of_equippables(
        #[case] input_n_of_equippables: u32,
        #[case] input_percentages: Vec<u8>,
        #[case] expected: Vec<i64>,
    ) {
        let result = calculate_n_of_values(input_n_of_equippables, input_percentages.as_slice());
        assert!(result.is_ok());
        assert_eq!(expected, result.unwrap());
    }

    #[rstest]
    #[case(10, vec![10,0,0,0,0,0,0], vec![10,0,0,0,0,0,0])]
    #[case(10, vec![10,10,0,0,0,0,0], vec![5,5,0,0,0,0,0])]
    #[case(10, vec![10,10,10,0,0,0,0], vec![4,3,3,0,0,0,0])]
    #[case(10, vec![10,0,0,0], vec![10,0,0,0])]
    #[case(10, vec![10,10,0,0], vec![5,5,0,0])]
    #[case(10, vec![10,10,10,0], vec![4,3,3,0])]
    fn calculate_equippable_values_with_missing_percentages(
        #[case] input_n_of_equippables: u32,
        #[case] input_percentages: Vec<u8>,
        #[case] expected: Vec<i64>,
    ) {
        let result = calculate_n_of_values(input_n_of_equippables, input_percentages.as_slice());
        assert!(result.is_ok());
        assert_eq!(expected, result.unwrap());
    }

    #[rstest]
    #[case(7., vec![1.,1.,1.,1.,1.,1.,1.])]
    #[case(14., vec![2.,2.,2.,2.,2.,2.,2.])]
    #[case(21., vec![3.,3.,3.,3.,3.,3.,3.])]
    #[case(28., vec![4.,4.,4.,4.,4.,4.,4.])]
    #[case(35., vec![5.,5.,5.,5.,5.,5.,5.])]
    #[case(42., vec![6.,6.,6.,6.,6.,6.,6.])]
    fn divide_equally_multiple_of_seven_between_seven_categories(
        #[case] to_distribute: f64,
        #[case] expected: Vec<f64>,
    ) {
        let result = divide_equally(to_distribute, 7);
        assert_eq!(expected, result);
    }

    #[rstest]
    #[case(4., vec![1.,1.,1.,1.])]
    #[case(8., vec![2.,2.,2.,2.])]
    #[case(12., vec![3.,3.,3.,3.])]
    #[case(16., vec![4.,4.,4.,4.])]
    #[case(20., vec![5.,5.,5.,5.])]
    #[case(24., vec![6.,6.,6.,6.])]
    fn divide_equally_multiple_of_four_between_four_categories(
        #[case] to_distribute: f64,
        #[case] expected: Vec<f64>,
    ) {
        let result = divide_equally(to_distribute, 4);
        assert_eq!(expected, result);
    }

    #[rstest]
    #[case(1., vec![1.,0.,0.,0.,0.,0.,0.])]
    #[case(2., vec![1.,1.,0.,0.,0.,0.,0.])]
    #[case(3., vec![1.,1.,1.,0.,0.,0.,0.])]
    #[case(4., vec![1.,1.,1.,1.,0.,0.,0.])]
    #[case(5., vec![1.,1.,1.,1.,1.,0.,0.])]
    #[case(6., vec![1.,1.,1.,1.,1.,1.,0.])]
    #[case(8., vec![2.,1.,1.,1.,1.,1.,1.])]
    #[case(9., vec![2.,2.,1.,1.,1.,1.,1.])]
    #[case(10., vec![2.,2.,2.,1.,1.,1.,1.])]
    #[case(11., vec![2.,2.,2.,2.,1.,1.,1.])]
    #[case(12., vec![2.,2.,2.,2.,2.,1.,1.])]
    #[case(13., vec![2.,2.,2.,2.,2.,2.,1.])]
    #[case(15., vec![3.,2.,2.,2.,2.,2.,2.])]
    #[case(16., vec![3.,3.,2.,2.,2.,2.,2.])]
    #[case(17., vec![3.,3.,3.,2.,2.,2.,2.])]
    #[case(18., vec![3.,3.,3.,3.,2.,2.,2.])]
    #[case(19., vec![3.,3.,3.,3.,3.,2.,2.])]
    #[case(20., vec![3.,3.,3.,3.,3.,3.,2.])]
    #[case(22., vec![4.,3.,3.,3.,3.,3.,3.])]
    #[case(23., vec![4.,4.,3.,3.,3.,3.,3.])]
    #[case(24., vec![4.,4.,4.,3.,3.,3.,3.])]
    #[case(25., vec![4.,4.,4.,4.,3.,3.,3.])]
    fn divide_equally_not_multiple_of_seven_between_seven_categories(
        #[case] to_distribute: f64,
        #[case] expected: Vec<f64>,
    ) {
        let result = divide_equally(to_distribute, 7);
        assert_eq!(expected, result);
    }

    #[rstest]
    #[case(1., vec![1.,0.,0.,0.])]
    #[case(2., vec![1.,1.,0.,0.])]
    #[case(3., vec![1.,1.,1.,0.])]
    #[case(5., vec![2.,1.,1.,1.])]
    #[case(6., vec![2.,2.,1.,1.])]
    #[case(7., vec![2.,2.,2.,1.])]
    #[case(9., vec![3.,2.,2.,2.])]
    #[case(13., vec![4.,3.,3.,3.])]
    #[case(17., vec![5.,4.,4.,4.])]
    #[case(21., vec![6.,5.,5.,5.])]
    #[case(25., vec![7.,6.,6.,6.])]
    fn divide_equally_not_multiple_of_four_between_four_categories(
        #[case] to_distribute: f64,
        #[case] expected: Vec<f64>,
    ) {
        let result = divide_equally(to_distribute, 4);
        assert_eq!(expected, result);
    }

    #[rstest]
    #[case(vec![8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0], vec![10, 10, 10, 30, 5, 5, 5], vec![7.0, 6.0, 5.0, 8.0, 4.0, 3.0, 2.0])]
    #[case(vec![9.0, 7.0, 8.0, 6.0, 5.0, 4.0, 3.0], vec![20, 10, 20, 10, 5, 5, 5], vec![9.0, 7.0, 8.0, 6.0, 5.0, 4.0, 3.0])]
    #[case(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0], vec![1, 2, 3, 4, 5, 6, 7], vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0])] // already ordered
    #[case(vec![7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0], vec![30, 20, 10, 8, 7, 5, 4], vec![7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0])] // descending by weight
    #[case(vec![5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0], vec![1, 1, 1, 1, 1, 1, 1], vec![11.0, 10.0, 9.0, 8.0, 7.0, 6.0, 5.0])] // equal weights, to_order sorted
    #[case(vec![10.0, 20.0, 30.0, 40.0, 0.0, 0.0, 0.0], vec![5, 10, 5, 20, 0, 0, 0], vec![20.0, 30.0, 10.0, 40.0, 0.0, 0.0, 0.0])]
    #[case(vec![1.5, 3.3, 2.2, 4.4, 0.0, 0.0, 0.0], vec![3, 1, 4, 2, 0, 0, 0], vec![3.3, 1.5, 4.4, 2.2, 0.0, 0.0, 0.0])] // mixed values
    #[case(vec![10.0, 10.0, 10.0, 10.0, 0.0, 0.0, 0.0], vec![4, 3, 2, 1, 0, 0, 0], vec![10.0, 10.0, 10.0, 10.0, 0.0, 0.0, 0.0])] // identical values
    #[case(vec![1.0, 2.0, 3.0, 4.0, 0.0, 0.0, 0.0], vec![0, 0, 100, 100, 0, 0, 0], vec![2.0, 1.0, 4.0, 3.0, 0.0, 0.0, 0.0])] // tie on highest weights
    #[case(vec![4.0, 2.0, 3.0, 1.0], vec![20, 20, 20, 40], vec![3.0, 2.0, 1.0, 4.0])]
    #[case(vec![9.0, 7.0, 8.0, 6.0], vec![20, 10, 20, 10], vec![9.0, 7.0, 8.0, 6.0])]
    #[case(vec![1.0, 2.0, 3.0, 4.0], vec![10, 20, 30, 40], vec![1.0, 2.0, 3.0, 4.0])] // already ordered
    #[case(vec![4.0, 3.0, 2.0, 1.0], vec![40, 30, 20, 10], vec![4.0, 3.0, 2.0, 1.0])] // descending by weight
    #[case(vec![5.0, 6.0, 7.0, 8.0], vec![1, 1, 1, 1], vec![8.0, 7.0, 6.0, 5.0])] // equal weights, to_order sorted
    #[case(vec![10.0, 20.0, 30.0, 40.0], vec![5, 10, 5, 20], vec![20.0, 30.0, 10.0, 40.0])]
    #[case(vec![1.5, 3.3, 2.2, 4.4], vec![3, 1, 4, 2], vec![3.3, 1.5, 4.4, 2.2])] // mixed values
    #[case(vec![10.0, 10.0, 10.0, 10.0], vec![4, 3, 2, 1], vec![10.0, 10.0, 10.0, 10.0])] // identical values
    #[case(vec![1.0, 2.0, 3.0, 4.0], vec![0, 0, 100, 100], vec![2.0, 1.0, 4.0, 3.0])] // tie on highest weights
    fn order_mixed_values(
        #[case] to_distribute: Vec<f64>,
        #[case] by: Vec<u8>,
        #[case] expected: Vec<f64>,
    ) {
        let result = order_distribution(&to_distribute, &by);
        assert_eq!(expected, result);
    }
}
