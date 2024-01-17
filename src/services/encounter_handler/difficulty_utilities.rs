use crate::models::encounter_structs::{EncounterChallengeEnum, ExpRange};

pub fn scale_difficulty_exp(base_difficulty: &EncounterChallengeEnum, party_size: i16) -> ExpRange {
    // Given the base difficulty and the party size, it scales the base difficulty.
    // Useful when a party is not the canon 4 party member.
    let party_deviation = party_size - 4;
    let upper_difficulty = get_next_difficulty(base_difficulty);
    ExpRange {
        lower_bound: convert_difficulty_enum_to_base_xp_budget(base_difficulty)
            + (party_deviation * convert_difficulty_enum_to_xp_adjustment(base_difficulty)),
        upper_bound: convert_difficulty_enum_to_base_xp_budget(&upper_difficulty)
            + (party_deviation * convert_difficulty_enum_to_xp_adjustment(&upper_difficulty))
                * match base_difficulty {
                    EncounterChallengeEnum::Impossible => 2,
                    _ => 1,
                },
    }
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

fn get_next_difficulty(diff: &EncounterChallengeEnum) -> EncounterChallengeEnum {
    match diff {
        EncounterChallengeEnum::Trivial => EncounterChallengeEnum::Low,
        EncounterChallengeEnum::Low => EncounterChallengeEnum::Moderate,
        EncounterChallengeEnum::Moderate => EncounterChallengeEnum::Severe,
        EncounterChallengeEnum::Severe => EncounterChallengeEnum::Extreme,
        EncounterChallengeEnum::Extreme => EncounterChallengeEnum::Impossible,
        EncounterChallengeEnum::Impossible => EncounterChallengeEnum::Impossible,
    }
}
