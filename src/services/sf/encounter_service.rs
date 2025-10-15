use crate::AppState;
use crate::db::bestiary_proxy::get_creatures_passing_all_filters;
use crate::models::bestiary_structs::BestiaryFilterQuery;
use crate::models::creature::creature_struct::Creature;
use crate::models::shared::game_system_enum::GameSystem;

pub async fn get_filtered_creatures(
    app_state: &AppState,
    filters: &BestiaryFilterQuery,
    allow_weak: bool,
    allow_elite: bool,
) -> anyhow::Result<Vec<Creature>> {
    get_creatures_passing_all_filters(
        app_state,
        &GameSystem::Starfinder,
        filters,
        allow_weak,
        allow_elite,
    )
    .await
}
