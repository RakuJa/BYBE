use crate::models::encounter_structs::{EncounterChallengeEnum, ExpRange};

/// Scales an encounter difficulty's XP budget to account for non-standard party sizes.
///
/// The canonical party size is 4. For each member above or below that, the XP budget
/// is adjusted using the difficulty's per-member XP adjustment value.
///
/// # Arguments
///
/// * `base_difficulty` - The target encounter difficulty.
/// * `party_size` - The number of players in the party.
///
/// # Returns
///
/// An [`ExpRange`] with lower and upper XP bounds scaled to the given party size.
pub const fn scale_difficulty_exp(
    base_difficulty: EncounterChallengeEnum,
    party_size: i64,
) -> ExpRange {
    let party_deviation = party_size - 4;
    let upper_difficulty = get_next_difficulty(base_difficulty);
    ExpRange {
        lower_bound: convert_difficulty_enum_to_base_xp_budget(base_difficulty)
            + (party_deviation * convert_difficulty_enum_to_xp_adjustment(base_difficulty)),
        upper_bound: convert_difficulty_enum_to_base_xp_budget(upper_difficulty)
            + (party_deviation * convert_difficulty_enum_to_xp_adjustment(upper_difficulty))
                * match base_difficulty {
                    EncounterChallengeEnum::Impossible => 2,
                    _ => 1,
                },
    }
}

const fn convert_difficulty_enum_to_base_xp_budget(diff: EncounterChallengeEnum) -> i64 {
    match diff {
        EncounterChallengeEnum::Trivial => 40,
        EncounterChallengeEnum::Low => 60,
        EncounterChallengeEnum::Moderate => 80,
        EncounterChallengeEnum::Severe => 120,
        EncounterChallengeEnum::Extreme => 160,
        EncounterChallengeEnum::Impossible => 320,
    }
}

const fn convert_difficulty_enum_to_xp_adjustment(diff: EncounterChallengeEnum) -> i64 {
    match diff {
        EncounterChallengeEnum::Trivial => 10,
        EncounterChallengeEnum::Low => 15,
        EncounterChallengeEnum::Moderate => 20,
        EncounterChallengeEnum::Severe => 30,
        EncounterChallengeEnum::Extreme => 40,
        EncounterChallengeEnum::Impossible => 60,
    }
}

const fn get_next_difficulty(diff: EncounterChallengeEnum) -> EncounterChallengeEnum {
    match diff {
        EncounterChallengeEnum::Trivial => EncounterChallengeEnum::Low,
        EncounterChallengeEnum::Low => EncounterChallengeEnum::Moderate,
        EncounterChallengeEnum::Moderate => EncounterChallengeEnum::Severe,
        EncounterChallengeEnum::Severe => EncounterChallengeEnum::Extreme,
        EncounterChallengeEnum::Extreme | EncounterChallengeEnum::Impossible => {
            EncounterChallengeEnum::Impossible
        }
    }
}
