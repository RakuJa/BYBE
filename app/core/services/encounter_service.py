import random
from collections import Counter
from collections.abc import Iterable
from itertools import chain

from typing import Optional, Dict, Set, List, Annotated

from pydantic import conlist

from app.core.resources.network import redis_proxy
from app.core.resources.schema.alignment_enum import AlignmentEnum
from app.core.resources.schema.creature_filter import CreatureFilter
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


def get_encounter_info(encounter_params: EncounterParams) -> dict:
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


async def generate_random_encounter(
    party_levels: Annotated[List[int], conlist(int, min_items=1)],
    encounter_difficulty: DifficultyEnum,
    family: Optional[str] = None,
    rarity: Optional[RarityEnum] = None,
    size: Optional[SizeEnum] = None,
    alignment: Optional[AlignmentEnum] = None,
) -> dict:
    exp, levels_combinations = calculate_level_combination_for_encounter(
        encounter_difficulty, party_levels
    )
    if not levels_combinations:
        raise ValueError("This encounter cannot be generated")
    filter_dict = build_filter_dict(family, rarity, size, alignment)
    creature_ids_dict: Dict[
        str, Set[str]
    ] = await filters_creatures_ids_by_filters_and_levels(
        filter_dict, levels_combinations
    )
    try:
        creature_ids_list = choose_random_valid_ids(
            creature_ids_dict, levels_combinations
        )
        encounter = await redis_proxy.get_creatures_by_ids(
            list(chain.from_iterable(creature_ids_list))
        )
    except ValueError:
        encounter = []
    scaled_exp_levels = calculate_encounter_scaling_difficulty(
        party_size=len(party_levels)
    )

    return {
        "results": encounter,
        "count": len(encounter),
        "experience": exp if encounter else 0,
        "difficulty": encounter_difficulty.value,
        "levels": scaled_exp_levels,
    }


async def filters_creatures_ids_by_filters_and_levels(
    filter_dict: dict, levels_combinations: List[List[str]]
) -> Dict[str, Set[str]]:
    id_set = None
    if filter_dict:
        # Fetch creature IDs passing all filters
        ids_dict = await redis_proxy.fetch_creature_ids_passing_all_filters(filter_dict)
        id_set = get_intersection_of_all_values_in_nested_dict(ids_dict)
        if not id_set:
            # Empty id set, abort. (Encounter could not be generated)
            return dict()

    # Fetch creature IDs passing level filter
    unique_levels = set(chain.from_iterable(levels_combinations))
    level_id = await redis_proxy.fetch_creature_ids_passing_filter(
        CreatureFilter.LEVEL, unique_levels
    )
    # Merge IDs with dict of sets and remove levels with empty sets
    # If id_set is empty THEN no filter were passed and you don't have to merge
    result = merge_ids_with_dict_of_sets(level_id, id_set) if id_set else level_id
    return {k: v for k, v in result.items() if v}


def choose_random_valid_ids(
    ids: Dict[str, Set[str]], levels_combinations: Iterable[Iterable[str]]
) -> List[List[str]]:
    valid_levels = filter_lists_by_id(lists=levels_combinations, ids=ids)
    random_encounter = random.sample(valid_levels, 1)[0]
    creature_count = Counter(random_encounter)
    new_ids_dict: Dict[str, List[str]] = dict()
    for key, value in creature_count.items():
        if len(ids[key]) < value:
            # fill ids[key] until len(ids[key]) = value. the filler values
            # are already presents inside the ids dictionary
            # Example: ids{0:{1,2}} creature_count{0:5} => ids{0:{1,2,2,1,2}
            filler_values = list(ids[key]) * (value // len(ids[key]))
            filler_values += list(ids[key])[: value % len(ids[key])]
            new_ids_dict[key] = filler_values
    return [
        random.sample([el for el in new_ids_dict[str(key)]], creature_count[key])
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
        filter_dict[CreatureFilter.FAMILY] = [family]
    if rarity:
        filter_dict[CreatureFilter.RARITY] = [rarity.value]
    if size:
        filter_dict[CreatureFilter.SIZE] = [size.value]
    if alignment:
        filter_dict[CreatureFilter.ALIGNMENT] = [alignment.value]
    return filter_dict


def filter_lists_by_id(
    lists: Iterable[Iterable[str]], ids: Dict[str, Set[str]]
) -> List[Iterable[str]]:
    """
    Returns only iterables with elements contained in ids
    """
    return [
        element_list for element_list in lists if all(el in ids for el in element_list)
    ]


def get_intersection_of_all_values_in_nested_dict(input_dict: dict) -> set:
    if not input_dict:
        # Otherwise set.intersection raise exception
        return set()
    return set.intersection(
        *[
            set(id_set)
            for nested_dict in input_dict.values()
            for id_set in nested_dict.values()
        ]
    )
