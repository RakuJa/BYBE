import random
from collections import Counter
from itertools import chain

from typing import Optional, Dict, Set, Iterable

from app.core.resources import redis_handler
from app.core.resources.schema.alignment_enum import AlignmentEnum
from app.core.resources.schema.difficulty_enum import DifficultyEnum
from app.core.resources.schema.encounter_params import EncounterParams
from app.core.resources.schema.rarity_enum import RarityEnum
from app.core.resources.schema.size_enum import SizeEnum
from app.core.services.encounter_handler.encounter_calculator import (
    calculate_encounter_exp,
    calculate_encounter_difficulty,
    calculate_encounter_scaling_difficulty,
    calculate_level_combination_for_encounter,
    merge_ids_with_dict_of_sets,
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


def _get_intersection_of_all_values_in_nested_dict(
    input_dict: Dict[str, Dict[str, Set[str]]]
) -> Set[str]:
    return set.intersection(
        *[
            # Must be a frozenset, set is mutable.
            {frozenset(id_set) for id_set in nested_dict.values()}
            for filters, nested_dict in input_dict.items()
        ]
    )


async def generate_random_encounter(
    encounter_params: EncounterParams,
    family: Optional[str] = None,
    rarity: Optional[RarityEnum] = None,
    size: Optional[SizeEnum] = None,
    alignment: Optional[AlignmentEnum] = None,
    encounter_difficulty: Optional[DifficultyEnum] = None,
):
    levels_combinations = calculate_level_combination_for_encounter(
        encounter_difficulty, encounter_params.party_levels
    )
    if not levels_combinations:
        raise ValueError("This encounter cannot be generated")
    filter_dict = build_filter_dict(family, rarity, size, alignment)
    creature_ids = await filters_creatures_ids_by_filters_and_levels(
        filter_dict, levels_combinations
    )
    creature_ids = choose_random_valid_ids(creature_ids, levels_combinations)
    return await redis_handler.get_creatures_by_id(
        list(chain.from_iterable(creature_ids))
    )


async def filters_creatures_ids_by_filters_and_levels(
    filter_dict: dict, levels_combinations: Iterable[Iterable[str]]
) -> Dict[int, Set[str]]:
    ids_dict = await redis_handler.fetch_creature_ids_passing_all_filters(filter_dict)
    id_set = get_intersection_of_all_values_in_nested_dict(ids_dict)
    unique_levels = set(chain.from_iterable(levels_combinations))
    level_id = await redis_handler.fetch_creature_ids_passing_filter(
        "level", unique_levels
    )
    # get dict made of level: {ids} and removes level with empty sets
    return {k: v for k, v in merge_ids_with_dict_of_sets(level_id, id_set).items() if v}


def choose_random_valid_ids(
    ids: Dict[int, Set[str]], levels_combinations: Iterable[Iterable[str]]
):
    valid_levels = filter_lists_by_id(lists=levels_combinations, ids=ids)
    random_encounter = random.sample(valid_levels, 1)[0]
    creature_count = Counter(random_encounter)
    return [
        random.sample([el for el in ids[key]], creature_count[key])
        for key in creature_count
    ]


def build_filter_dict(
    family: Optional[str],
    rarity: Optional[RarityEnum],
    size: Optional[SizeEnum],
    alignment: Optional[AlignmentEnum],
) -> dict:
    filter_dict = {}
    if family:
        filter_dict["family"] = [family]
    if rarity:
        filter_dict["rarity"] = [rarity.value]
    if size:
        filter_dict["size"] = [size.value]
    if alignment:
        filter_dict["alignment"] = [alignment.value]
    return filter_dict


def filter_lists_by_id(lists: Iterable[Iterable[str]], ids: Dict[int, Iterable[str]]):
    """
    Returns only iterables with elements contained in ids
    """
    return [
        element_list for element_list in lists if all(el in ids for el in element_list)
    ]


def get_intersection_of_all_values_in_nested_dict(input_dict: dict) -> set:
    return set.intersection(
        *[
            set(id_set)
            for nested_dict in input_dict.values()
            for id_set in nested_dict.values()
        ]
    )
