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
    pub(crate) game_system: GameSystem,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, PartialEq, Debug)]
pub struct ShareableShop {
    pub(crate) items_ids: Vec<i64>,
    pub(crate) game_system: GameSystem,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug, PartialEq, Eq)]
pub struct ShareableEncounter {
    pub(crate) creatures_ids_and_variant: Vec<(i64, CreatureVariant)>,
    pub(crate) game_system: GameSystem,
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
            items_ids: (0..=n_items).collect(),
            game_system: Default::default(),
        }
    }

    fn get_test_shareable_encounter(n_items: i64) -> ShareableEncounter {
        ShareableEncounter {
            creatures_ids_and_variant: (0..=n_items)
                .map(|n| (n, CreatureVariant::Base))
                .collect::<Vec<_>>(),
            game_system: Default::default(),
        }
    }

    #[tokio::test]
    async fn test_shop_encode_w_one_item() {
        let shop = get_test_shareable_shop(1);
        assert_eq!("KLUv_QBYIQAAAgACAA==", shop.encode().await.unwrap())
    }

    #[tokio::test]
    async fn test_shop_encode_w_20_items() {
        let shop = get_test_shareable_shop(20);
        assert_eq!(
            "KLUv_QBYuQAAFQACBAYICgwOEBIUFhgaHB4gIiQmKAA=",
            shop.encode().await.unwrap()
        )
    }

    #[tokio::test]
    async fn test_shop_decode_w_20_items() {
        let shop = get_test_shareable_shop(20);
        assert_eq!(
            ShareableShop::decode("KLUv_QBYuQAAFQACBAYICgwOEBIUFhgaHB4gIiQmKAA=".to_string())
                .await
                .unwrap(),
            shop
        )
    }
    #[tokio::test]
    async fn test_shop_decode_w_one_item() {
        let shop = get_test_shareable_shop(1);
        assert_eq!(
            ShareableShop::decode("KLUv_QBYIQAAAgACAA==".to_string())
                .await
                .unwrap(),
            shop
        )
    }

    #[tokio::test]
    async fn test_encounter_encode_w_one_item() {
        let encounter = get_test_shareable_encounter(1);
        assert_eq!("KLUv_QBYMQAAAgACAgIA", encounter.encode().await.unwrap())
    }

    #[tokio::test]
    async fn test_encounter_encode_w_20_items() {
        let encounter = get_test_shareable_encounter(20);
        assert_eq!(
            "KLUv_QBYYQEAFQACAgIEAgYCCAIKAgwCDgIQAhICFAIWAhgCGgIcAh4CIAIiAiQCJgIoAgA=",
            encounter.encode().await.unwrap()
        )
    }

    #[tokio::test]
    async fn test_encounter_decode_w_20_items() {
        let encounter = get_test_shareable_encounter(20);
        assert_eq!(
            ShareableEncounter::decode(
                "KLUv_QBYYQEAFQACAgIEAgYCCAIKAgwCDgIQAhICFAIWAhgCGgIcAh4CIAIiAiQCJgIoAgA="
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
            ShareableEncounter::decode("KLUv_QBYMQAAAgACAgIA".to_string())
                .await
                .unwrap(),
            encounter
        )
    }
}
