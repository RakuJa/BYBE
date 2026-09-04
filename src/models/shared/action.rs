use crate::models::shared::description::{Description, TagContext};
use crate::models::shared::rarity_enum::RarityEnum;
use crate::models::shared::trait_data::TraitData;
use crate::traits::resolve_tags::ResolveTags;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq, Debug, FromRow)]
pub struct CoreAction {
    pub id: i64,
    pub name: String,
    pub action_type: String,
    #[schema(example = 1)]
    pub n_of_actions: Option<i32>,
    pub category: Option<String>,
    pub description: Description,

    pub license: String,
    pub remaster: bool,
    pub source: String,

    pub slug: Option<String>,
    #[sqlx(try_from = "String")]
    pub rarity: RarityEnum,

    #[schema(example = 1)]
    pub frequency_max: Option<i32>,
    #[schema(example = "day")]
    pub frequency_per: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Eq, Hash, PartialEq, Debug, ToSchema)]
pub struct Action {
    pub core_action: CoreAction,
    pub traits: Vec<TraitData>,
}

impl ResolveTags for Action {
    fn resolve_tags(&mut self, ctx: &TagContext) {
        self.core_action.description = Description::new(
            self.core_action.description.resolve(&TagContext {
                variant_damage: Some(
                    ctx.creature_variant
                        .to_dmg_adjustment_modifier(self.core_action.frequency_per.is_some()),
                ),
                creature_variant: ctx.creature_variant,
                actor_level: ctx.actor_level,
            }),
        );
    }
}
