from statistics import mean
from typing import List, Dict

from app.core.resources.schema.difficulty_enum import DifficultyEnum


def convert_level_difference_into_experience(
    level_difference: float, party_size: int
) -> int:
    """
    Given the level difference and party size, it calculate the exp for the encounter
    :param level_difference: Enemy level - Party AVG level
    :param party_size: Size of the party
    :return: The experience that the enemy will yield
    """
    if level_difference < -4:
        return 0
    elif -4 >= level_difference < -3:
        return 10
    elif -3 >= level_difference < -2:
        return 15
    elif -2 >= level_difference < -1:
        return 20
    elif -1 >= level_difference < 0:
        return 30
    elif 0 >= level_difference < 1:
        return 40
    elif 1 >= level_difference < 2:
        return 60
    elif 2 >= level_difference < 3:
        return 80
    elif 3 >= level_difference < 4:
        return 120
    elif 4 >= level_difference < 5:
        return 160
    else:
        # just to avoid the level 1 party of 50 people destroying a lvl 20
        return 320 + (party_size - 4) * 60

    # Impossible = 320 with chara adjust 60, invented by me but what else can i do?
    # Pathfinder 2E thinks that a GM will only try out extreme encounter at maximum
    # I have to introduce a level for impossible things, Needs balancing Paizo help


def calculate_encounter_exp(party_levels: List[int], enemy_levels: List[int]) -> int:
    """
    Given a party and enemy party, it calculates the experience that the party
    will get from defeating the enemy
    :param party_levels:
    :param enemy_levels:
    :return: Experience of the whole encounter
    """
    party_avg = mean(party_levels)
    exp_sum = 0
    for enemy_lvl in enemy_levels:
        exp_sum += convert_level_difference_into_experience(
            level_difference=(enemy_lvl - party_avg),
            party_size=len(party_levels),
        )
    return exp_sum


def calculate_encounter_difficulty(
    encounter_exp: int, scaled_exp_levels: Dict[DifficultyEnum, int]
) -> DifficultyEnum:
    """
    Given the encounter total experience and the level of experience scaled with the party,
    It returns the difficulty of the encounter
    :param encounter_exp:
    :param scaled_exp_levels:
    :return:
    """
    enc_exp = encounter_exp
    exp = scaled_exp_levels
    if enc_exp < exp[DifficultyEnum.TRIVIAL]:
        return DifficultyEnum.TRIVIAL
    elif exp[DifficultyEnum.LOW] <= enc_exp < exp[DifficultyEnum.MODERATE]:
        return DifficultyEnum.LOW
    elif exp[DifficultyEnum.MODERATE] <= enc_exp < exp[DifficultyEnum.SEVERE]:
        return DifficultyEnum.MODERATE
    elif exp[DifficultyEnum.SEVERE] <= enc_exp < exp[DifficultyEnum.EXTREME]:
        return DifficultyEnum.SEVERE
    elif exp[DifficultyEnum.EXTREME] <= enc_exp < exp[DifficultyEnum.IMPOSSIBLE]:
        return DifficultyEnum.EXTREME
    else:
        return DifficultyEnum.IMPOSSIBLE


def calculate_encounter_scaling_difficulty(
    party_size: int,
) -> Dict[DifficultyEnum, int]:
    """
    Given the party size, it scales and calculates the thresholds for the various difficulty levels
    :param party_size:
    :return:
    """
    return {_enum: _scale_difficulty_exp(_enum, party_size) for _enum in DifficultyEnum}


def _scale_difficulty_exp(base_difficulty: DifficultyEnum, party_size: int) -> int:
    """
    Given the base difficulty and the party size, it scales the base difficulty.
    :param base_difficulty:
    :param party_size:
    :return:
    """
    party_deviation = party_size - 4
    return base_difficulty.get_base_xp_budget() + (
        party_deviation * base_difficulty.get_xp_adjustment()
    )
