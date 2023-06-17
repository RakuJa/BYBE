import asyncio
import logging
import time
from collections import defaultdict
from typing import List, Tuple, Dict, Set, Iterable, Optional

from returns.maybe import Maybe, Nothing

from app.core.resources.network import redis_communicator
from app.core.resources.network.creature_cache import CreatureCache
from app.core.resources.creature import Creature
from app.core.resources.schema.enum.alignment_enum import AlignmentEnum
from app.core.resources.schema.enum.creature_filter_enum import CreatureFilter
from app.core.resources.schema.enum.order_enum import OrderEnum
from app.core.resources.schema.enum.rarity_enum import RarityEnum
from app.core.resources.schema.enum.size_enum import SizeEnum
from app.core.resources.schema.enum.sort_enum import CreatureFieldsEnum

creatures_cache: Optional[CreatureCache] = None
cache_expiration: int = 3600  # cache expires after one hour
last_cache_update_time: float = 0
logger = logging.getLogger(__name__)


async def update_cache() -> None:
    global last_cache_update_time, creatures_cache
    while True:
        if time.time() - last_cache_update_time >= cache_expiration:
            # fetch data from database and update cache
            creatures_list = fetch_data_from_database()
            (
                level_dict,
                family_dict,
                alignment_dict,
                size_dict,
                rarity_dict,
            ) = __create_enum_dicts(creatures_list)
            creatures_cache = CreatureCache(
                creatures_list=creatures_list,
                level_dict=level_dict,
                family_dict=family_dict,
                alignment_dict=alignment_dict,
                size_dict=size_dict,
                rarity_dict=rarity_dict,
            )

            last_cache_update_time = time.time()
        await asyncio.sleep(60)  # sleep for 60 seconds


def __create_enum_dicts(
    creatures_list: List[Creature],
) -> Tuple[
    Dict[str, List[Creature]],
    Dict[str, List[Creature]],
    Dict[str, List[Creature]],
    Dict[str, List[Creature]],
    Dict[str, List[Creature]],
]:
    """

    :param creatures_list:
    :return: A Tuple (level_dict, family_dict, alignment_dict, size_dict, rarity_dict)
    """
    level_dict = defaultdict(list)
    family_dict = defaultdict(list)
    alignment_dict = defaultdict(list)
    size_dict = defaultdict(list)
    rarity_dict = defaultdict(list)

    for curr_creature in creatures_list:
        level_dict[str(curr_creature.level)].append(curr_creature)
        family_dict[curr_creature.family].append(curr_creature)
        alignment_dict[curr_creature.alignment.value].append(curr_creature)
        size_dict[curr_creature.size.value].append(curr_creature)
        rarity_dict[curr_creature.rarity.value if curr_creature.rarity else "-"].append(
            curr_creature
        )

    return (
        dict(level_dict),
        dict(family_dict),
        dict(alignment_dict),
        dict(size_dict),
        dict(rarity_dict),
    )


def fetch_data_from_database() -> List[Creature]:
    return redis_communicator.get_creatures_by_id(
        redis_communicator.fetch_and_parse_all_keys(pattern="creature:*")
    )


def get_paginated_creatures(
    cursor: int,
    page_size: int,
    sort_field: CreatureFieldsEnum,
    order: OrderEnum,
    name_filter: Optional[str],
    family_filter: Optional[str],
    rarity_filter: Optional[RarityEnum],
    size_filter: Optional[SizeEnum],
    alignment_filter: Optional[AlignmentEnum],
) -> Maybe[Tuple[int, List[Creature]]]:
    if creatures_cache:
        ordered_values: List[Creature] = creatures_cache.get_list(sort_field, order)
        filtered_values: List[Creature] = list(
            filter(
                lambda creature: check_element_pass_filters(
                    creature,
                    name_filter,
                    family_filter,
                    rarity_filter,
                    size_filter,
                    alignment_filter,
                ),
                ordered_values,
            )
        )

        next_cursor = (
            cursor + page_size
            if len(filtered_values) > cursor + page_size
            else len(filtered_values)
        )
        return Maybe.from_value((next_cursor, filtered_values[cursor:next_cursor]))
    else:
        # We should have a direct call like we had in the past
        # redis_communicator.get_paginated_creatures(cursor, page_size)
        # But this would increment code complexity for
        # a non-existent case. (handling filter, orders, etc..)
        # cache is empty only on startup, later on it is never emptied but always
        # overwritten.
        return Nothing


def check_element_pass_filters(
    element: Creature,
    name_filter: Optional[str],
    family_filter: Optional[str],
    rarity_filter: Optional[RarityEnum],
    size_filter: Optional[SizeEnum],
    alignment_filter: Optional[AlignmentEnum],
) -> bool:
    if name_filter is not None and name_filter.lower() not in element.name.lower():
        return False
    if (
        family_filter is not None
        and family_filter.lower() not in element.family.lower()
    ):
        return False
    if rarity_filter is not None and element.rarity != rarity_filter:
        return False
    if size_filter is not None and element.size != size_filter:
        return False
    if alignment_filter is not None and element.alignment != alignment_filter:
        return False
    return True


def get_keys(creature_filter: CreatureFilter) -> List[str]:
    if creatures_cache:
        return sorted(list(creatures_cache.get_dictionary(creature_filter).keys()))
    else:
        return sorted(
            redis_communicator.fetch_and_parse_all_keys(
                creature_filter.value.lower() + "*"
            )
        )


def get_creatures_by_ids(id_list: List[str]) -> List[Creature]:
    creatures_list: List[Creature] = []
    for _id in id_list:
        curr_creature = get_creature_by_id(_id)
        if curr_creature:
            creatures_list.append(curr_creature)
    return creatures_list


def get_creature_by_id(creature_id: str) -> Optional[Creature]:
    if creatures_cache:
        return creatures_cache.id_filter.get(creature_id, None)
    return redis_communicator.get_creature_by_id(creature_id)


def fetch_creature_ids_passing_all_filters(
    key_value_filters: dict,
) -> Dict[str, Dict[str, Set[str]]]:
    if creatures_cache:
        ids_passing_filter: Dict[str, Dict[str, Set[str]]] = dict()
        for key, value in key_value_filters.items():
            curr_dict = fetch_creature_ids_passing_filter(key, filter_list=value)
            if not curr_dict:
                return {}
            ids_passing_filter[key] = curr_dict
        return ids_passing_filter
    else:
        # We should have a direct call like we had in the past
        # redis_communicator.get_paginated_creatures(cursor, page_size)
        # But this would increment code complexity for a
        # non-existent case. (handling filter, orders, etc..)
        # cache is empty only on startup, later on it is never emptied but always
        # overwritten.
        return {}


def fetch_creature_ids_passing_filter(
    creature_filter: CreatureFilter, filter_list: Iterable[str]
) -> Dict[str, Set[str]]:
    if creatures_cache:
        ids_passing_filter: Dict[str, Set[str]] = dict()
        for curr_value in filter_list:
            curr_set = set(
                creature.id
                for creature in creatures_cache.get_dictionary(creature_filter)[
                    curr_value
                ]
            )
            if curr_set:
                ids_passing_filter[curr_value] = curr_set
            else:
                logger.debug(
                    f"No keys found for {creature_filter} with value {curr_value}"
                )
        return ids_passing_filter
    else:
        return {}
