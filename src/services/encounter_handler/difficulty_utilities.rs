use crate::models::encounter_structs::EncounterDifficultyEnum;

pub fn scale_difficulty_exp(base_difficulty: &EncounterDifficultyEnum, party_size: u8) -> i16 {
    // Given the base difficulty and the party size, it scales the base difficulty.
    // Useful when a party is not the canon 4 party member
    let party_deviation = (party_size - 4) as i16;
    convert_difficulty_enum_to_base_xp_budget(base_difficulty)
        + (party_deviation * convert_difficulty_enum_to_xp_adjustment(base_difficulty))
}

fn convert_difficulty_enum_to_base_xp_budget(diff: &EncounterDifficultyEnum) -> i16 {
    match diff {
        EncounterDifficultyEnum::Trivial => 40,
        EncounterDifficultyEnum::Low => 60,
        EncounterDifficultyEnum::Moderate => 80,
        EncounterDifficultyEnum::Severe => 120,
        EncounterDifficultyEnum::Extreme => 160,
        EncounterDifficultyEnum::Impossible => 320,
        EncounterDifficultyEnum::Random => panic!(
            "This enumerator has not been implemented yet, \
            you should not use random directly, convert it to one of the accepted values!"
        ),
    }
}

fn convert_difficulty_enum_to_xp_adjustment(diff: &EncounterDifficultyEnum) -> i16 {
    match diff {
        EncounterDifficultyEnum::Trivial => 10,
        EncounterDifficultyEnum::Low => 15,
        EncounterDifficultyEnum::Moderate => 20,
        EncounterDifficultyEnum::Severe => 30,
        EncounterDifficultyEnum::Extreme => 40,
        EncounterDifficultyEnum::Impossible => 60,
        EncounterDifficultyEnum::Random => panic!(
            "This enumerator has not been implemented yet, \
            you should not use random directly, convert it to one of the accepted values!"
        ),
    }
}
