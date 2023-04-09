from app.core.resources.schema.encounter_params import EncounterParams
from app.core.services.encounter_handler.encounter_calculator import (
    calculate_encounter_exp,
    calculate_encounter_difficulty,
    calculate_encounter_scaling_difficulty,
)


def get_encounter_info(encounter_params: EncounterParams):
    party_levels = encounter_params.party_levels
    encounter_experience = calculate_encounter_exp(
        party_levels=party_levels, enemy_levels=encounter_params.enemy_levels
    )
    scaled_exp_levels = calculate_encounter_scaling_difficulty(
        party_size=len(party_levels)
    )
    encounter_difficulty = calculate_encounter_difficulty(
        encounter_exp=encounter_experience, scaled_exp_levels=scaled_exp_levels
    )
    return {
        "experience": encounter_experience,
        "difficulty": encounter_difficulty.value,
        "levels": scaled_exp_levels,
    }
