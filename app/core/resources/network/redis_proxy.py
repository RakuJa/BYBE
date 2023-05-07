import asyncio
import logging
import time
from collections import defaultdict
from typing import List, Tuple, Dict, Set, Iterable

from app.core.resources.network import redis_communicator
from app.core.resources.schema.creature import Creature
from app.core.resources.schema.creature_filter import CreatureFilter
from app.core.resources.schema.order_enum import OrderEnum

creatures_cache: Dict[str, dict] = {}
cache_expiration = 3600  # cache expires after one hour
last_cache_update_time = 0
logger = logging.getLogger(__name__)


async def update_cache():
    global last_cache_update_time, creatures_cache
    while True:
        if time.time() - last_cache_update_time >= cache_expiration:
            # fetch data from database and update cache
            creatures_list = await fetch_data_from_database()
            (
                level_dict,
                family_dict,
                alignment_dict,
                size_dict,
                rarity_dict,
            ) = __create_enum_dicts(creatures_list)
            creatures_cache = {
                "lists": {
                    OrderEnum.UNORDERED: creatures_list,
                    OrderEnum.ORDERED_BY_ID: sorted(
                        creatures_list, key=lambda creature: creature.id
                    ),
                    OrderEnum.ORDERED_BY_NAME: sorted(
                        creatures_list, key=lambda creature: creature.name
                    ),
                    OrderEnum.ORDERED_BY_HP: sorted(
                        creatures_list, key=lambda creature: creature.hp
                    ),
                    OrderEnum.ORDERED_BY_LEVEL: sorted(
                        creatures_list, key=lambda creature: creature.level
                    ),
                    OrderEnum.ORDERED_BY_FAMILY: sorted(
                        creatures_list, key=lambda creature: creature.family
                    ),
                    OrderEnum.ORDERED_BY_ALIGNMENT: sorted(
                        creatures_list, key=lambda creature: creature.alignment
                    ),
                    OrderEnum.ORDERED_BY_SIZE: sorted(
                        creatures_list, key=lambda creature: creature.size
                    ),
                    OrderEnum.ORDERED_BY_RARITY: sorted(
                        creatures_list, key=lambda creature: creature.rarity
                    ),
                },
                "dicts": {
                    CreatureFilter.ID: {x.id: x for x in creatures_list},
                    CreatureFilter.LEVEL: level_dict,
                    CreatureFilter.FAMILY: family_dict,
                    CreatureFilter.ALIGNMENT: alignment_dict,
                    CreatureFilter.SIZE: size_dict,
                    CreatureFilter.RARITY: rarity_dict,
                },
            }

            last_cache_update_time = time.time()
        await asyncio.sleep(60)  # sleep for 60 seconds


def __create_enum_dicts(
    creatures_list: List[Creature],
) -> Tuple[dict, dict, dict, dict, dict]:
    """

    :param creatures_list:
    :return: A Tuple (level_dict, family_dict, alignment_dict, size_dict, rarity_dict)
    """
    level_dict = defaultdict(list)
    family_dict = defaultdict(list)
    alignment_dict = defaultdict(list)
    size_dict = defaultdict(list)
    rarity_dict = defaultdict(list)

    for creature in creatures_list:
        level_dict[str(creature.level)].append(creature)
        family_dict[creature.family].append(creature)
        alignment_dict[creature.alignment].append(creature)
        size_dict[creature.size].append(creature)
        rarity_dict[creature.rarity].append(creature)

    return (
        dict(level_dict),
        dict(family_dict),
        dict(alignment_dict),
        dict(size_dict),
        dict(rarity_dict),
    )


async def fetch_data_from_database() -> List[Creature]:
    return await redis_communicator.get_creatures_by_id(
        await redis_communicator.fetch_and_parse_all_keys(pattern="creature:*")
    )


async def get_paginated_creatures(
    cursor: int, page_size: int, order: OrderEnum
) -> Tuple[int, List[Creature]]:
    if creatures_cache:
        ordered_values = creatures_cache["lists"][order]
        next_cursor = (
            cursor + page_size
            if len(ordered_values) > cursor + page_size
            else len(creatures_cache)
        )
        return next_cursor, ordered_values[cursor:next_cursor]
    else:
        return await redis_communicator.get_paginated_creatures(cursor, page_size)


async def get_keys(creature_filter: CreatureFilter):
    if creatures_cache:
        return sorted(list(creatures_cache["dicts"][creature_filter].keys()))
    else:
        return sorted(
            await redis_communicator.fetch_and_parse_all_keys(
                creature_filter.value.lower() + "*"
            )
        )


async def get_creatures_by_ids(id_list: List[str]) -> List[Creature]:
    return [await get_creature_by_id(_id) for _id in id_list]


async def get_creature_by_id(creature_id: str) -> Creature:
    if creatures_cache:
        return creatures_cache["dicts"][CreatureFilter.ID][creature_id]
    else:
        return await redis_communicator.get_creature_by_id(creature_id)


async def fetch_creature_ids_passing_all_filters(
    key_value_filters: dict,
) -> Dict[str, Dict[str, Set[str]]]:
    if creatures_cache:
        ids_passing_filter: Dict[str, Dict[str, Set[str]]] = dict()
        for key, value in key_value_filters.items():
            curr_dict = await fetch_creature_ids_passing_filter(key, filter_list=value)
            if not curr_dict:
                return {}
            ids_passing_filter[key] = curr_dict
        return ids_passing_filter
    else:
        return await redis_communicator.fetch_creature_ids_passing_all_filters(
            key_value_filters
        )


async def fetch_creature_ids_passing_filter(
    creature_filter: CreatureFilter, filter_list: Iterable[str]
) -> Dict[str, Set[str]]:
    if creatures_cache:
        ids_passing_filter: Dict[str, Set[str]] = dict()
        for curr_value in filter_list:
            curr_set = set(
                creature.id
                for creature in creatures_cache["dicts"][creature_filter][curr_value]
            )
            if curr_set:
                ids_passing_filter[curr_value] = curr_set
            else:
                logger.debug(
                    f"No keys found for {creature_filter} with value {curr_value}"
                )
        return ids_passing_filter
    else:
        return await redis_communicator.fetch_creature_ids_passing_filter(
            creature_filter.value.lower(), filter_list
        )
