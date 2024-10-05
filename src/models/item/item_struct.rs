use crate::models::item::item_metadata::type_enum::ItemTypeEnum;
use crate::models::ordered_float_to_schema;
use crate::models::pf_version_enum::PathfinderVersionEnum;
use crate::models::routers_validator_structs::ItemFieldFilters;
use crate::models::shared::rarity_enum::RarityEnum;
use crate::models::shared::size_enum::SizeEnum;
use ordered_float::OrderedFloat;
use ordered_float_to_schema::ordered_float_to_schema;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;
use sqlx::{Error, FromRow, Row};
use std::str::FromStr;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema, Hash, Eq, PartialEq)]
pub struct Item {
    pub id: i64,
    pub name: String,
    #[schema(schema_with = ordered_float_to_schema)]
    pub bulk: OrderedFloat<f64>,
    #[schema(example = 0)]
    pub quantity: i64,
    pub base_item: Option<String>,
    pub category: Option<String>,
    pub description: String,
    #[schema(example = 0)]
    pub hardness: i64,
    #[schema(example = 0)]
    pub hp: i64,
    #[schema(example = 0)]
    pub level: i64,
    #[schema(example = 0)]
    pub price: i64, // in cp,
    pub usage: Option<String>,
    pub group: Option<String>,
    pub item_type: ItemTypeEnum,
    pub material_grade: Option<String>,
    pub material_type: Option<String>,
    #[schema(example = 0)]
    pub number_of_uses: Option<i64>, // for consumables, for equip set as null.

    // source details (title, license, remastered)
    pub license: String,
    pub remaster: bool,
    pub source: String,

    pub rarity: RarityEnum,
    pub size: SizeEnum,
    pub traits: Vec<String>,
}

impl<'r> FromRow<'r, SqliteRow> for Item {
    fn from_row(row: &'r SqliteRow) -> Result<Self, Error> {
        let rarity: String = row.try_get("rarity")?;
        let size: String = row.try_get("size")?;
        let type_str: String = row.try_get("item_type")?;
        let bulk: f64 = row.try_get("bulk")?;
        Ok(Item {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            bulk: OrderedFloat::from(bulk),
            // Db table does not contain this information. Only creature's items
            // have a quantity !=1. So we default to 1 and then fetch when needed.
            quantity: 1,
            base_item: row.try_get("base_item")?,
            category: row.try_get("category").ok(),
            description: row.try_get("description")?,
            hardness: row.try_get("hardness")?,
            hp: row.try_get("hp")?,
            level: row.try_get("level")?,
            price: row.try_get("price")?,
            size: SizeEnum::from(size),
            rarity: RarityEnum::from(rarity),
            license: row.try_get("license")?,
            remaster: row.try_get("remaster")?,
            source: row.try_get("source")?,
            item_type: ItemTypeEnum::from_str(type_str.as_str()).unwrap(),
            material_grade: row.try_get("material_grade").ok(),
            material_type: row.try_get("material_type").ok(),
            traits: vec![],
            usage: row.try_get("usage")?,
            number_of_uses: row.try_get("number_of_uses").ok(),
            group: row.try_get("item_group")?,
        })
    }
}

impl Item {
    pub fn is_passing_filters(&self, filters: &ItemFieldFilters) -> bool {
        self.check_item_pass_equality_filters(filters)
            && self.check_item_pass_lb_filters(filters)
            && self.check_item_pass_ub_filters(filters)
            && self.check_item_pass_string_filters(filters)
    }

    fn check_item_pass_ub_filters(&self, filters: &ItemFieldFilters) -> bool {
        filters
            .max_bulk_filter
            .map_or(true, |max_bulk| self.bulk <= OrderedFloat(max_bulk))
            && filters
                .max_hardness_filter
                .map_or(true, |max_hard| self.hardness <= max_hard)
            && filters
                .max_hp_filter
                .map_or(true, |max_hp| self.hp <= max_hp)
            && filters
                .max_level_filter
                .map_or(true, |max_lvl| self.level <= max_lvl)
            && filters
                .max_price_filter
                .map_or(true, |max_price| self.price <= max_price)
            && filters.max_n_of_uses_filter.map_or(true, |max_uses| {
                if self.number_of_uses.is_none() {
                    true
                } else {
                    self.number_of_uses.unwrap_or(0) <= max_uses
                }
            })
    }

    fn check_item_pass_lb_filters(&self, filters: &ItemFieldFilters) -> bool {
        filters
            .min_bulk_filter
            .map_or(true, |min_bulk| self.bulk >= OrderedFloat(min_bulk))
            && filters
                .min_hardness_filter
                .map_or(true, |min_hard| self.hardness >= min_hard)
            && filters
                .min_hp_filter
                .map_or(true, |min_hp| self.hp >= min_hp)
            && filters
                .min_level_filter
                .map_or(true, |min_lvl| self.level >= min_lvl)
            && filters
                .min_price_filter
                .map_or(true, |min_price| self.price >= min_price)
            && filters.min_n_of_uses_filter.map_or(true, |min_uses| {
                if self.number_of_uses.is_none() {
                    false
                } else {
                    self.number_of_uses.unwrap_or(0) >= min_uses
                }
            })
    }

    fn check_item_pass_equality_filters(&self, filters: &ItemFieldFilters) -> bool {
        filters
            .rarity_filter
            .as_ref()
            .map_or(true, |x| x.iter().any(|rarity| self.rarity == *rarity))
            && filters
                .size_filter
                .as_ref()
                .map_or(true, |x| x.iter().any(|size| self.size == *size))
            && filters
                .type_filter
                .as_ref()
                .map_or(true, |x| x.iter().any(|t_filt| self.item_type == *t_filt))
    }

    fn check_item_pass_string_filters(&self, filters: &ItemFieldFilters) -> bool {
        filters.name_filter.as_ref().map_or(true, |name| {
            self.name
                .to_lowercase()
                .contains(name.to_lowercase().as_str())
        }) && filters.category_filter.as_ref().map_or(true, |x| {
            x.iter().any(|cat| {
                self.category
                    .clone()
                    .unwrap_or_default()
                    .to_lowercase()
                    .contains(cat.to_lowercase().as_str())
            })
        }) && match filters.pathfinder_version.clone().unwrap_or_default() {
            PathfinderVersionEnum::Legacy => !self.remaster,
            PathfinderVersionEnum::Remaster => self.remaster,
            PathfinderVersionEnum::Any => true,
        } && filters.source_filter.as_ref().map_or(true, |x| {
            x.iter().any(|source| {
                self.source
                    .to_lowercase()
                    .contains(source.to_lowercase().as_str())
            })
        }) && filters.trait_whitelist_filter.as_ref().map_or(true, |x| {
            x.iter().any(|filter_trait| {
                self.traits.iter().any(|item_trait| {
                    item_trait
                        .to_lowercase()
                        .contains(filter_trait.to_lowercase().as_str())
                })
            })
        }) && !filters.trait_blacklist_filter.as_ref().map_or(false, |x| {
            x.iter().any(|filter_trait| {
                self.traits.iter().any(|item_trait| {
                    item_trait
                        .to_lowercase()
                        .eq(filter_trait.to_lowercase().as_str())
                })
            })
        })
    }
}
