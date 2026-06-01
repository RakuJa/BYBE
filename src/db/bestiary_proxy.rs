use crate::AppState;
use crate::db::data_providers::creature_fetcher::fetch_traits_associated_with_creatures;
use crate::db::data_providers::{creature_fetcher, fetch_unique_values_from_db};
use crate::models::bestiary_structs::{
    BestiaryFilterQuery, BestiaryPaginatedRequest, BestiaryRanges,
};
use crate::models::creature::creature_field_filter::CreatureFieldFilters;
use crate::models::creature::creature_filter_enum::{CreatureFilter, FieldsUniqueValuesStruct};
use crate::models::creature::creature_metadata::creature_role::CreatureRoleEnum;
use crate::models::creature::creature_metadata::type_enum::CreatureTypeEnum;
use crate::models::creature::creature_metadata::variant_enum::CreatureVariant;
use crate::models::creature::creature_struct::Creature;
use crate::models::response_data::CreatureResponseDataModifiers;
use crate::models::shared::alignment_enum::AlignmentEnum;
use crate::models::shared::game_system_enum::GameSystem;
use crate::models::shared::pf_version_enum::GameSystemVersionEnum;
use anyhow::Result;
#[cfg(feature = "cache")]
use cached::cached;
use itertools::Itertools;
use strum::IntoEnumIterator;

pub async fn get_creature_by_id(
    app_state: &AppState,
    gs: GameSystem,
    id: i64,
    variant: CreatureVariant,
    response_data_mods: &CreatureResponseDataModifiers,
) -> Option<Creature> {
    creature_fetcher::fetch_creature_by_id(&app_state.pool, gs, variant, response_data_mods, id)
        .await
        .ok()
}

pub async fn get_weak_creature_by_id(
    app_state: &AppState,
    gs: GameSystem,
    id: i64,
    optional_data: &CreatureResponseDataModifiers,
) -> Option<Creature> {
    get_creature_by_id(app_state, gs, id, CreatureVariant::Weak, optional_data).await
}

pub async fn get_elite_creature_by_id(
    app_state: &AppState,
    gs: GameSystem,
    id: i64,
    optional_data: &CreatureResponseDataModifiers,
) -> Option<Creature> {
    get_creature_by_id(app_state, gs, id, CreatureVariant::Elite, optional_data).await
}

pub async fn get_paginated_creatures(
    app_state: &AppState,
    gs: GameSystem,
    filters: &CreatureFieldFilters,
    pagination: &BestiaryPaginatedRequest,
) -> Result<(u32, Vec<Creature>)> {
    let count = creature_fetcher::fetch_creatures_listing_count(&app_state.pool, gs, filters)
        .await
        .unwrap_or(0) as u32;
    let core_data = creature_fetcher::fetch_paginated_creatures(
        &app_state.pool,
        gs,
        filters,
        pagination.bestiary_sort_data.sort_by.unwrap_or_default(),
        pagination.bestiary_sort_data.order_by.unwrap_or_default(),
        pagination.paginated_request.cursor,
        pagination.paginated_request.page_size,
    )
    .await?;
    let creatures = core_data
        .into_iter()
        .map(|x| Creature::from_core(x, gs))
        .collect();
    Ok((count, creatures))
}

pub async fn get_creatures_passing_all_filters(
    app_state: &AppState,
    gs: GameSystem,
    filters: &BestiaryFilterQuery,
    fetch_weak: bool,
    fetch_elite: bool,
) -> Result<Vec<Creature>> {
    let mut creature_vec = Vec::new();
    let level_vec = filters.creature_table_fields_filter.level_filter.clone();
    let mut modified_filters = filters.clone();
    modified_filters.creature_table_fields_filter.level_filter =
        prepare_level_filter_for_db_communication(
            filters
                .creature_table_fields_filter
                .level_filter
                .clone()
                .into_iter(),
            fetch_weak,
            fetch_elite,
        );

    for core in creature_fetcher::fetch_creatures_core_data_with_filters(
        &app_state.pool,
        gs,
        &modified_filters,
    )
    .await?
    {
        // We have fetched creature with level +1 if weak is allowed or level-1 if elite is allowed
        // (or both). Now we catalogue correctly giving them the elite or weak variant, this does not
        // mean that if we have [0,1,2,3] in the filter and allow_elite => [-1,0,1,2,3] then
        // a creature of level 1 will always be considered the elite variant of level 0. We'll
        // duplicate the data and will have a base 0 for level 0 and elite 0 for level 1
        if fetch_weak && level_vec.contains(&(core.essential.base_level - 1)) {
            creature_vec.push(Creature::from_core_with_variant(
                core.clone(),
                CreatureVariant::Weak,
                gs,
            ));
        }
        if fetch_elite && level_vec.contains(&(core.essential.base_level + 1)) {
            creature_vec.push(Creature::from_core_with_variant(
                core.clone(),
                CreatureVariant::Elite,
                gs,
            ));
        }
        creature_vec.push(Creature::from_core(core, gs));
    }
    Ok(creature_vec)
}

pub async fn get_all_possible_values_of_filter(
    app_state: &AppState,
    gs: GameSystem,
    field: CreatureFilter,
) -> Vec<String> {
    let runtime_fields_values = get_all_keys(app_state, gs).await;
    let mut x = match field {
        CreatureFilter::Size => runtime_fields_values.list_of_sizes,
        CreatureFilter::Rarity => runtime_fields_values.list_of_rarities,
        CreatureFilter::Ranged | CreatureFilter::Melee | CreatureFilter::Spellcaster => {
            vec![true.to_string(), false.to_string()]
        }
        CreatureFilter::Family => runtime_fields_values.list_of_families,
        CreatureFilter::Traits => runtime_fields_values.list_of_traits,
        CreatureFilter::Sources => runtime_fields_values.list_of_sources,
        CreatureFilter::Alignment => AlignmentEnum::iter().map(|x| x.to_string()).collect(),
        CreatureFilter::Level => runtime_fields_values.list_of_levels,
        CreatureFilter::CreatureTypes => CreatureTypeEnum::iter().map(|x| x.to_string()).collect(),
        CreatureFilter::CreatureRoles => CreatureRoleEnum::iter().map(|x| x.to_string()).collect(),
        CreatureFilter::PathfinderVersion => GameSystemVersionEnum::iter()
            .map(|x| x.to_string())
            .collect(),
    };
    x.sort();
    x
}

#[cfg_attr(feature = "cache", cached(key = "i64", convert = r##"{ gs.into() }"##))]
async fn get_all_keys(app_state: &AppState, gs: GameSystem) -> FieldsUniqueValuesStruct {
    let table = format!("{gs}_creature_core");
    FieldsUniqueValuesStruct {
        list_of_levels: fetch_unique_values_from_db(app_state, table.clone(), "level".into()).await,
        list_of_families: fetch_unique_values_from_db(app_state, table.clone(), "family".into())
            .await,
        list_of_traits: fetch_traits_associated_with_creatures(&app_state.pool, gs)
            .await
            .unwrap_or_default(),
        list_of_sources: fetch_unique_values_from_db(app_state, table.clone(), "source".into())
            .await,
        list_of_sizes: fetch_unique_values_from_db(app_state, table.clone(), "size".into()).await,
        list_of_rarities: fetch_unique_values_from_db(app_state, table, "rarity".into()).await,
    }
}
#[cfg_attr(feature = "cache", cached(key = "i64", convert = r##"{ gs.into() }"##))]
pub async fn get_bestiary_ranges(app_state: &AppState, gs: GameSystem) -> Option<BestiaryRanges> {
    creature_fetcher::fetch_creature_ranges(&app_state.pool, gs)
        .await
        .ok()
}

/// Used to prepare the filters for db communication.
/// The level must be adjusted if elite/weak must be fetched.
///Example if we allow weak then we can fetch creature with level +1 => weak = level
fn prepare_level_filter_for_db_communication<I>(
    level_filter: I,
    fetch_weak: bool,
    fetch_elite: bool,
) -> Vec<i64>
where
    I: Iterator<Item = i64>,
{
    // do not remove sorted, it would break contract with merge and dedup
    let levels = level_filter.sorted().collect::<Vec<_>>();
    let levels_for_elite: Vec<i64> = if fetch_elite {
        levels.iter().map(|x| x - 1).collect()
    } else {
        vec![]
    };
    let levels_for_weak: Vec<i64> = if fetch_weak {
        levels.iter().map(|x| x + 1).collect()
    } else {
        vec![]
    };

    let x = itertools::merge(levels_for_elite, levels_for_weak).collect::<Vec<_>>();
    itertools::merge(x, levels).dedup().collect::<Vec<_>>()
}
