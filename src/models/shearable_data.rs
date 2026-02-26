use crate::models::response_data::ResponseNpc;
use crate::models::shared::game_system_enum::GameSystem;

use crate::models::creature::creature_metadata::variant_enum::CreatureVariant;
use crate::traits::base64::base64_decode::Base64Decode;
use crate::traits::base64::base64_encode::Base64Encode;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct ShareableNpcList {
    pub(crate) list_name: String,
    pub(crate) npcs_data: Vec<ResponseNpc>,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, PartialEq, Debug)]
pub struct ShareableShop {
    pub(crate) shop_name: String,
    pub(crate) items_data: Vec<ShareableItem>,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug, PartialEq, Eq)]
pub struct LegacyShareableEncounter {
    pub(crate) encounter_name: String,
    pub(crate) creatures_data: Vec<SharableCreature>,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug, PartialEq, Eq)]
pub struct ShareableEncounter {
    pub(crate) encounter_name: String,
    pub(crate) creatures_data: Vec<SharableCreature>,
    pub(crate) hazards_data: Vec<SharableHazard>,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Eq, PartialEq, Debug)]
pub struct ShareableItem {
    pub(crate) id: u64,
    pub(crate) qty: u64,
    pub(crate) game: GameSystem,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug, PartialEq, Eq)]
pub struct SharableCreature {
    pub(crate) id: u64,
    pub(crate) qty: u64,
    pub(crate) variant: CreatureVariant,
    pub(crate) game: GameSystem,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug, PartialEq, Eq)]
pub struct SharableHazard {
    pub(crate) id: u64,
    pub(crate) qty: u64,
    pub(crate) game: GameSystem,
}

impl Base64Encode for ShareableShop {}

impl Base64Encode for LegacyShareableEncounter {}

impl Base64Encode for ShareableEncounter {}

impl Base64Encode for ShareableNpcList {}

impl Base64Decode for ShareableShop {}

impl Base64Decode for LegacyShareableEncounter {}

impl Base64Decode for ShareableEncounter {}

impl Base64Decode for ShareableNpcList {}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_shareable_shop(n_items: u64) -> ShareableShop {
        ShareableShop {
            shop_name: "Osi".to_string(),
            items_data: (0..=n_items)
                .map(|n| ShareableItem {
                    id: n,
                    qty: 1,
                    game: GameSystem::default(),
                })
                .collect(),
        }
    }

    fn get_test_legacy_shareable_encounter(n_items: u64) -> LegacyShareableEncounter {
        LegacyShareableEncounter {
            encounter_name: "Osi".to_string(),
            creatures_data: (0..=n_items)
                .map(|n| SharableCreature {
                    id: n,
                    qty: 1,
                    variant: CreatureVariant::Base,
                    game: GameSystem::default(),
                })
                .collect(),
        }
    }

    fn get_test_shareable_encounter(n_creatures: u64, n_hazards: u64) -> ShareableEncounter {
        ShareableEncounter {
            encounter_name: "Osi".to_string(),
            creatures_data: (0..=n_creatures)
                .map(|n| SharableCreature {
                    id: n,
                    qty: 1,
                    variant: CreatureVariant::Base,
                    game: GameSystem::default(),
                })
                .collect(),
            hazards_data: (0..=n_hazards)
                .map(|n| SharableHazard {
                    id: n,
                    qty: 1,
                    game: GameSystem::default(),
                })
                .collect(),
        }
    }

    #[tokio::test]
    async fn test_shop_encode_w_one_item() {
        let shop = get_test_shareable_shop(1);
        assert_eq!("KLUv_QBYWQAAA09zaQIAAQABAQA=", shop.encode().await.unwrap())
    }

    #[tokio::test]
    async fn test_shop_decode_w_one_item() {
        let shop = get_test_shareable_shop(1);
        assert_eq!(
            ShareableShop::decode("KLUv_QBYWQAAA09zaQIAAQABAQA=".to_string())
                .await
                .unwrap(),
            shop
        )
    }

    #[tokio::test]
    async fn test_shop_encode_w_20_items() {
        let shop = get_test_shareable_shop(20);
        assert_eq!(
            "KLUv_QBYrQEAQkQMFJA7Y_c7fXd3vbZt27bdRERIkpQCvrjiiSN-uOGFEz644IED3pz5cuXJz5HvMs7TXAA=",
            shop.encode().await.unwrap()
        )
    }

    #[tokio::test]
    async fn test_shop_decode_w_20_items() {
        let shop = get_test_shareable_shop(20);
        assert_eq!(
            ShareableShop::decode(
                "KLUv_QBYrQEAQkQMFJA7Y_c7fXd3vbZt27bdRERIkpQCvrjiiSN-uOGFEz644IED3pz5cuXJz5HvMs7TXAA=".to_string()
            )
            .await
            .unwrap(),
            shop
        )
    }

    #[tokio::test]
    async fn test_legacy_encounter_encode_w_one_item() {
        let encounter = get_test_legacy_shareable_encounter(1);
        assert_eq!(
            "KLUv_QBYaQAAA09zaQIAAQIAAQECAA==",
            encounter.encode().await.unwrap()
        )
    }

    #[tokio::test]
    async fn test_encounter_encode_w_one_item() {
        let encounter = get_test_shareable_encounter(1, 1);
        assert_eq!(
            "KLUv_QBYoQAAA09zaQIAAQIAAQECAAIAAQABAQA=",
            encounter.encode().await.unwrap()
        )
    }

    #[tokio::test]
    async fn test_legacy_encounter_decode_w_one_item() {
        let encounter = get_test_legacy_shareable_encounter(1);
        assert_eq!(
            LegacyShareableEncounter::decode("KLUv_QBYaQAAA09zaQIAAQIAAQECAA==".to_string())
                .await
                .unwrap(),
            encounter
        )
    }

    #[tokio::test]
    async fn test_encounter_decode_w_one_item() {
        let encounter = get_test_shareable_encounter(1, 1);
        assert_eq!(
            ShareableEncounter::decode("KLUv_QBYoQAAA09zaQIAAQIAAQECAAIAAQABAQA=".to_string())
                .await
                .unwrap(),
            encounter
        )
    }

    #[tokio::test]
    async fn test_legacy_encounter_encode_w_20_items() {
        let encounter = get_test_legacy_shareable_encounter(20);
        assert_eq!(
            "KLUv_QBY7QEAkkUOFJA7a_c7fXd3vbZt23YTEREhMqUUbaJFtIfW0BZaQjtoBW2gBbRNy7RLq7RJi7RHe9pb2xSMxuIEAA==",
            encounter.encode().await.unwrap()
        )
    }

    #[tokio::test]
    async fn test_encounter_encode_w_20_items() {
        let encounter = get_test_shareable_encounter(20, 20);
        assert_eq!(
            "KLUv_QBYxQIAkgkVFYBlhgMrE5vADMwMp6enpydt7y1TCv64440zvrjiiSN-uOGFEz644IEvV558vgu5H7fjbtyMe3Er7sSNuA-34S7chHtwC-7AvdzKndxz7xYCBKIUAA==",
            encounter.encode().await.unwrap()
        )
    }

    #[tokio::test]
    async fn test_legacy_encounter_decode_w_20_items() {
        let encounter = get_test_legacy_shareable_encounter(20);
        assert_eq!(
            LegacyShareableEncounter::decode(
                "KLUv_QBY7QEAkkUOFJA7a_c7fXd3vbZt23YTEREhMqUUbaJFtIfW0BZaQjtoBW2gBbRNy7RLq7RJi7RHe9pb2xSMxuIEAA=="
                    .to_string()
            )
            .await
            .unwrap(),
            encounter
        )
    }

    #[tokio::test]
    async fn test_encounter_decode_w_20_items() {
        let encounter = get_test_shareable_encounter(20, 20);
        assert_eq!(
            ShareableEncounter::decode(
                "KLUv_QBYxQIAkgkVFYBlhgMrE5vADMwMp6enpydt7y1TCv64440zvrjiiSN-uOGFEz644IEvV558vgu5H7fjbtyMe3Er7sSNuA-34S7chHtwC-7AvdzKndxz7xYCBKIUAA=="
                    .to_string()
            )
                .await
                .unwrap(),
            encounter
        )
    }
}
