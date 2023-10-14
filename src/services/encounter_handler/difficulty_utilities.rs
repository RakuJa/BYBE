use crate::models::encounter_structs::EncounterChallengeEnum;

pub fn scale_difficulty_exp(base_difficulty: &EncounterChallengeEnum, party_size: i16) -> i16 {
    // Given the base difficulty and the party size, it scales the base difficulty.
    // Useful when a party is not the canon 4 party member
    let party_deviation = party_size - 4;
    convert_difficulty_enum_to_base_xp_budget(base_difficulty)
        + (party_deviation * convert_difficulty_enum_to_xp_adjustment(base_difficulty))
}

fn convert_difficulty_enum_to_base_xp_budget(diff: &EncounterChallengeEnum) -> i16 {
    match diff {
        EncounterChallengeEnum::Trivial => 40,
        EncounterChallengeEnum::Low => 60,
        EncounterChallengeEnum::Moderate => 80,
        EncounterChallengeEnum::Severe => 120,
        EncounterChallengeEnum::Extreme => 160,
        EncounterChallengeEnum::Impossible => 320,
    }
}

fn convert_difficulty_enum_to_xp_adjustment(diff: &EncounterChallengeEnum) -> i16 {
    match diff {
        EncounterChallengeEnum::Trivial => 10,
        EncounterChallengeEnum::Low => 15,
        EncounterChallengeEnum::Moderate => 20,
        EncounterChallengeEnum::Severe => 30,
        EncounterChallengeEnum::Extreme => 40,
        EncounterChallengeEnum::Impossible => 60,
    }
}
