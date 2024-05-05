use crate::models::creature::PublicationInfo;
use crate::models::creature_metadata::alignment_enum::AlignmentEnum;
use crate::models::creature_metadata::rarity_enum::RarityEnum;
use crate::models::creature_metadata::size_enum::SizeEnum;
use crate::models::creature_metadata::type_enum::CreatureTypeEnum;
use crate::models::creature_metadata::variant_enum::CreatureVariant;
use crate::models::db::raw_creature::RawCreature;
use crate::models::db::raw_trait::RawTrait;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, Hash, PartialEq)]
pub struct CreatureCoreData {
    pub id: i32,
    pub aon_id: Option<i32>,
    pub name: String,
    pub hp: i16,
    // constant value, it will never change
    pub base_level: i8,
    pub alignment: AlignmentEnum,
    pub size: SizeEnum,
    pub family: Option<String>,
    pub rarity: RarityEnum,
    pub is_melee: bool,
    pub is_ranged: bool,
    pub is_spell_caster: bool,
    pub publication_info: PublicationInfo,
    pub traits: Vec<String>,
    pub archive_link: Option<String>,
    pub creature_type: CreatureTypeEnum,
    pub variant: CreatureVariant,
}

impl From<(RawCreature, Vec<RawTrait>, bool, bool, Option<String>)> for CreatureCoreData {
    fn from(tuple: (RawCreature, Vec<RawTrait>, bool, bool, Option<String>)) -> Self {
        let raw = tuple.0;
        let traits = tuple.1;
        let is_ranged = tuple.2;
        let is_melee = tuple.3;
        let archive_link = tuple.4;

        let alignment_enum = AlignmentEnum::from((&traits, raw.remaster));
        CreatureCoreData {
            id: raw.id as i32,
            aon_id: raw.aon_id.map(|x| x as i32),
            name: raw.name.clone(),
            hp: raw.hp as i16,
            base_level: raw.level as i8,
            alignment: alignment_enum,
            size: raw.size.clone(),
            family: raw.family.clone(),
            rarity: raw.rarity.clone(),
            is_spell_caster: raw.spell_casting_name.is_some(),
            publication_info: PublicationInfo {
                remaster: raw.remaster,
                source: raw.source,
                license: raw.license,
            },
            traits: traits
                .into_iter()
                .map(|curr_trait| curr_trait.name)
                .collect(),
            creature_type: raw.cr_type.clone(),
            archive_link: archive_link.clone(),
            variant: CreatureVariant::Base,
            is_ranged,
            is_melee,
        }
    }
}
