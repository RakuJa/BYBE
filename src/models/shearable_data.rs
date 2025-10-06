use crate::models::response_data::ResponseNpc;
use crate::models::shared::game_system_enum::GameSystem;

use crate::models::creature::creature_metadata::variant_enum::CreatureVariant;
use crate::traits::base64::base64_decode::Base64Decode;
use crate::traits::base64::base64_encode::Base64Encode;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct ShareableNpcList {
    pub(crate) npcs: Vec<ResponseNpc>,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, PartialEq, Debug)]
pub struct ShareableShop {
    pub(crate) items_data: Vec<ShareableItem>,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug, PartialEq, Eq)]
pub struct ShareableEncounter {
    pub(crate) creatures_data: Vec<SharableCreature>,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, PartialEq, Debug)]
pub struct ShareableItem {
    pub(crate) id: i64,
    pub(crate) game: GameSystem,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug, PartialEq, Eq)]
pub struct SharableCreature {
    pub(crate) id: i64,
    pub(crate) variant: CreatureVariant,
    pub(crate) game: GameSystem,
}

impl Base64Encode for ShareableShop {}

impl Base64Encode for ShareableEncounter {}

impl Base64Encode for ShareableNpcList {}

impl Base64Decode for ShareableShop {}

impl Base64Decode for ShareableEncounter {}

impl Base64Decode for ShareableNpcList {}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_shareable_shop(n_items: i64) -> ShareableShop {
        ShareableShop {
            items_data: (0..=n_items)
                .map(|n| ShareableItem {
                    id: n,
                    game: GameSystem::default(),
                })
                .collect(),
        }
    }

    fn get_test_shareable_encounter(n_items: i64) -> ShareableEncounter {
        ShareableEncounter {
            creatures_data: (0..=n_items)
                .map(|n| SharableCreature {
                    id: n,
                    variant: CreatureVariant::Base,
                    game: GameSystem::default(),
                })
                .collect(),
        }
    }

    #[tokio::test]
    async fn test_shop_encode_w_one_item() {
        let shop = get_test_shareable_shop(1);
        assert_eq!("KLUv_QBYKQAAAgAAAgA=", shop.encode().await.unwrap())
    }

    #[tokio::test]
    async fn test_shop_encode_w_20_items() {
        let shop = get_test_shareable_shop(20);
        assert_eq!(
            "KLUv_QBYWQEAFQAAAgAEAAYACAAKAAwADgAQABIAFAAWABgAGgAcAB4AIAAiACQAJgAoAA==",
            shop.encode().await.unwrap()
        )
    }

    #[tokio::test]
    async fn test_shop_decode_w_20_items() {
        let shop = get_test_shareable_shop(20);
        assert_eq!(
            ShareableShop::decode("KLUv_QBYWQEAFQAAAgAEAAYACAAKAAwADgAQABIAFAAWABgAGgAcAB4AIAAiACQAJgAoAA==".to_string())
                .await
                .unwrap(),
            shop
        )
    }
    #[tokio::test]
    async fn test_shop_decode_w_one_item() {
        let shop = get_test_shareable_shop(1);
        assert_eq!(
            ShareableShop::decode("KLUv_QBYKQAAAgAAAgA=".to_string())
                .await
                .unwrap(),
            shop
        )
    }

    #[tokio::test]
    async fn test_encounter_encode_w_one_item() {
        let encounter = get_test_shareable_encounter(1);
        assert_eq!("KLUv_QBYOQAAAgACAAICAA==", encounter.encode().await.unwrap())
    }

    #[tokio::test]
    async fn test_encounter_encode_w_20_items() {
        let encounter = get_test_shareable_encounter(20);
        assert_eq!(
            "KLUv_QBYZQEAAgQKDhCt0wFmQDOEbFICQuIUO8ywwggbTLDAAHvmjNkyZcmQHTNWjFjbBAA=",
            encounter.encode().await.unwrap()
        )
    }

    #[tokio::test]
    async fn test_encounter_decode_w_20_items() {
        let encounter = get_test_shareable_encounter(20);
        assert_eq!(
            ShareableEncounter::decode(
                "KLUv_QBYZQEAAgQKDhCt0wFmQDOEbFICQuIUO8ywwggbTLDAAHvmjNkyZcmQHTNWjFjbBAA="
                    .to_string()
            )
            .await
            .unwrap(),
            encounter
        )
    }
    #[tokio::test]
    async fn test_encounter_decode_w_one_item() {
        let encounter = get_test_shareable_encounter(1);
        assert_eq!(
            ShareableEncounter::decode("KLUv_QBYOQAAAgACAAICAA==".to_string())
                .await
                .unwrap(),
            encounter
        )
    }
}
