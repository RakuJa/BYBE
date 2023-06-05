from statistics import mean
from typing import List, Dict, Set, Tuple
from math import floor, dist

from app.core.resources.schema.difficulty_enum import DifficultyEnum


def get_lvl_and_exp_dict() -> Dict[int, int]:
    return {
        -4: 10,
        -3: 15,
        -2: 20,
        -1: 30,
        00: 40,
        +1: 60,
        +2: 80,
        +3: 120,
        +4: 160,
    }


def convert_level_difference_into_experience(
    level_difference: float, party_size: int
) -> int:
    """
    Given the level difference and party size, it calculates the exp for the encounter
    :param level_difference: Enemy level - Party AVG level
    :param party_size: Size of the party
    :return: The experience that the enemy will yield
    """
    lvl_diff_rounded_down = floor(level_difference)
    lvl_and_exp_dict: Dict[int, int] = get_lvl_and_exp_dict()
    if lvl_diff_rounded_down in lvl_and_exp_dict:
        return lvl_and_exp_dict[lvl_diff_rounded_down]
    elif lvl_diff_rounded_down < -4:
        return 0
    else:
        # just to avoid the level 1 party of 50 people destroying a lvl 20
        return _scale_difficulty_exp(DifficultyEnum.IMPOSSIBLE, party_size=party_size)


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
        if enemy_lvl < 0 and enemy_lvl < party_avg:
            lvl_diff = -dist((party_avg,), (enemy_lvl,))
        else:
            lvl_diff = enemy_lvl - party_avg
        exp_sum += convert_level_difference_into_experience(
            level_difference=lvl_diff,
            party_size=len(party_levels),
        )
    return exp_sum


def calculate_encounter_difficulty(
    encounter_exp: int, scaled_exp_levels: Dict[DifficultyEnum, int]
) -> DifficultyEnum:
    """
    Given the encounter total experience and the level
    of experience scaled with the party,
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
    Given the party size, it scales and calculates
    the thresholds for the various difficulty levels
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


def calculate_level_combination_for_encounter(
    difficulty: DifficultyEnum, party_levels: List[int]
) -> Tuple[List[List[int]], int]:
    """
    Given an encounter difficulty it calculates all possible encounter permutations
    :param difficulty:
    :param party_levels:
    :return: a Tuple of [enc_lvl_combinations, encounter_exp]
    """
    encounter_exp: int = _scale_difficulty_exp(
        base_difficulty=difficulty, party_size=len(party_levels)
    )
    return (
        calculate_level_combinations_for_given_exp(
            encounter_exp,
            party_lvl=floor(mean(party_levels)),
        ),
        encounter_exp,
    )


def calculate_level_combinations_for_given_exp(
    experience: int, party_lvl: int
) -> List[List[int]]:
    """
    Given a encounter experience it calculates all possible encounter permutations
    :param experience:
    :param party_lvl:
    :return:
    """
    encounters_lvl: List[List[int]] = []
    exp_list: List[int] = [exp for lvl, exp in get_lvl_and_exp_dict().items()]
    for el in find_combinations(candidates=exp_list, target=experience):
        encounters_lvl.append(
            [
                party_lvl + convert_exp_to_lvl_diff(curr_exp)
                for curr_exp in el
                if party_lvl + convert_exp_to_lvl_diff(curr_exp) >= -1
            ]
        )
    return encounters_lvl


def convert_exp_to_lvl_diff(experience: int) -> int:
    for lvl, exp in get_lvl_and_exp_dict().items():
        if experience == exp:
            return lvl
    raise Exception()


def merge_ids_with_dict_of_sets(
    dict_of_sets: Dict[str, Set[str]], ids: Set[str]
) -> Dict[str, Set[str]]:
    """
    foreach key in the dictionary, merges the set of id with the current key ids
    :param dict_of_sets: Dictionary made up by [key, set of ids]
    :param ids:
    :return:
    """
    return {
        key: set.intersection(curr_ids, ids) for key, curr_ids in dict_of_sets.items()
    }


def find_combinations(candidates: List[int], target: int) -> List[List[int]]:
    """
    Find all combinations of numbers in the candidates list that sum up to the target.
    """

    def backtrack(start: int, target: int, path: List[int]) -> None:
        # If target is reached, add the current path to results list
        if target == 0:
            result.append(path)
            return
        # If target is negative, no need to continue as
        # adding more numbers will exceed the target
        elif target < 0:
            return
        # Iterate through the candidates starting from the given index
        for i in range(start, len(candidates)):
            # Make a recursive call to backtrack with updated target and path
            backtrack(i, target - candidates[i], path + [candidates[i]])

    result: List[List] = []  # List to store all combinations
    candidates.sort()  # Sort the candidates list for optimization
    backtrack(0, target, [])  # Start the backtracking from the first index
    return result
