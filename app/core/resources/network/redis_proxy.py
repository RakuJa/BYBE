import asyncio
import logging
import time
from collections import defaultdict
from collections.abc import Iterable
from typing import TYPE_CHECKING

from returns.maybe import Maybe, Nothing

from app.core.resources.creature import Creature
from app.core.resources.network import redis_communicator
from app.core.resources.network.creature_cache import CreatureCache
from app.core.resources.schema.enum.alignment_enum import AlignmentEnum
from app.core.resources.schema.enum.creature_filter_enum import CreatureFilter
from app.core.resources.schema.enum.order_enum import OrderEnum
from app.core.resources.schema.enum.rarity_enum import RarityEnum
from app.core.resources.schema.enum.size_enum import SizeEnum
from app.core.resources.schema.enum.sort_enum import CreatureFieldsEnum

if TYPE_CHECKING:
    from enum import Enum

creatures_cache: CreatureCache | None = None
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
                melee_dict,
                ranged_dict,
                spell_caster_dict,
            ) = __create_enum_dicts(creatures_list)
            creatures_cache = CreatureCache(
                creatures_list=creatures_list,
                level_dict=level_dict,
                family_dict=family_dict,
                alignment_dict=alignment_dict,
                size_dict=size_dict,
                rarity_dict=rarity_dict,
                melee_dict=melee_dict,
                ranged_dict=ranged_dict,
                spell_caster_dict=spell_caster_dict,
            )

            last_cache_update_time = time.time()
        await asyncio.sleep(60)  # sleep for 60 seconds


def __create_enum_dicts(
    creatures_list: list[Creature],
) -> tuple[
    dict[str, list[Creature]],
    dict[str, list[Creature]],
    dict[str, list[Creature]],
    dict[str, list[Creature]],
    dict[str, list[Creature]],
    dict[str, list[Creature]],
    dict[str, list[Creature]],
    dict[str, list[Creature]],
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
    melee_dict = defaultdict(list)
    ranged_dict = defaultdict(list)
    spell_caster_dict = defaultdict(list)

    for curr_creature in creatures_list:
        level_dict[str(curr_creature.level)].append(curr_creature)
        family_dict[curr_creature.family].append(curr_creature)
        alignment_dict[curr_creature.alignment.value].append(curr_creature)
        size_dict[curr_creature.size.value].append(curr_creature)
        rarity_dict[curr_creature.rarity.value if curr_creature.rarity else "-"].append(
            curr_creature,
        )
        melee_dict[str(curr_creature.is_melee)].append(curr_creature)
        ranged_dict[str(curr_creature.is_ranged)].append(curr_creature)
        spell_caster_dict[str(curr_creature.is_spell_caster)].append(curr_creature)

    return (
        dict(level_dict),
        dict(family_dict),
        dict(alignment_dict),
        dict(size_dict),
        dict(rarity_dict),
        dict(melee_dict),
        dict(ranged_dict),
        dict(spell_caster_dict),
    )


def fetch_data_from_database() -> list[Creature]:
    return redis_communicator.get_creatures_by_id(
        redis_communicator.fetch_and_parse_all_keys(pattern="creature:*"),
    )


def get_paginated_creatures(
    cursor: int,
    page_size: int,
    sort_field: CreatureFieldsEnum,
    order: OrderEnum,
    name_filter: str | None,
    family_filter: str | None,
    rarity_filter: RarityEnum | None,
    size_filter: SizeEnum | None,
    alignment_filter: AlignmentEnum | None,
    min_hp_filter: int | None,
    max_hp_filter: int | None,
    min_level_filter: int | None,
    max_level_filter: int | None,
    is_melee_filter: bool | None,
    is_ranged_filter: bool | None,
    is_spell_caster_filter: bool | None,
) -> Maybe[tuple[int, list[Creature]]]:
    if creatures_cache:
        ordered_values: list[Creature] = creatures_cache.get_list(sort_field, order)
        filtered_values: list[Creature] = list(
            filter(
                lambda creature: check_element_pass_filters(
                    creature,
                    name_filter=name_filter,
                    family_filter=family_filter,
                    rarity_filter=rarity_filter,
                    size_filter=size_filter,
                    alignment_filter=alignment_filter,
                    min_hp_filter=min_hp_filter,
                    max_hp_filter=max_hp_filter,
                    min_level_filter=min_level_filter,
                    max_level_filter=max_level_filter,
                    is_melee_filter=is_melee_filter,
                    is_ranged_filter=is_ranged_filter,
                    is_spell_caster_filter=is_spell_caster_filter,
                ),
                ordered_values,
            ),
        )

        next_cursor = (
            cursor + page_size
            if len(filtered_values) > cursor + page_size
            else len(filtered_values)
        )
        return Maybe.from_value((next_cursor, filtered_values[cursor:next_cursor]))

    # We should have a direct call like we had in the past
    # But this would increment code complexity for
    # a non-existent case. (handling filter, orders, etc..)
    # cache is empty only on startup, later on it is never emptied but always
    # overwritten.
    return Nothing


def check_element_pass_filters(
    element: Creature,
    name_filter: str | None,
    family_filter: str | None,
    rarity_filter: RarityEnum | None,
    size_filter: SizeEnum | None,
    alignment_filter: AlignmentEnum | None,
    min_hp_filter: int | None,
    max_hp_filter: int | None,
    min_level_filter: int | None,
    max_level_filter: int | None,
    is_melee_filter: bool | None,
    is_ranged_filter: bool | None,
    is_spell_caster_filter: bool | None,
) -> bool:
    return (
        _check_element_pass_equals_filters(
            element=element,
            name_filter=name_filter,
            family_filter=family_filter,
            rarity_filter=rarity_filter,
            size_filter=size_filter,
            alignment_filter=alignment_filter,
            is_melee_filter=is_melee_filter,
            is_ranged_filter=is_ranged_filter,
            is_spell_caster_filter=is_spell_caster_filter,
        )
        and _check_element_pass_greater_filters(
            element=element,
            max_hp_filter=max_hp_filter,
            max_level_filter=max_level_filter,
        )
        and _check_element_pass_lesser_filters(
            element=element,
            min_hp_filter=min_hp_filter,
            min_level_filter=min_level_filter,
        )
    )


def _check_element_pass_lesser_filters(
    element: Creature,
    min_hp_filter: int | None,
    min_level_filter: int | None,
) -> bool:
    lesser_filters = [
        (min_hp_filter, element.hp),
        (min_level_filter, element.level),
    ]
    for filter_value, element_value in lesser_filters:
        if filter_value is not None and element_value < filter_value:
            return False
    return True


def _check_element_pass_greater_filters(
    element: Creature,
    max_hp_filter: int | None,
    max_level_filter: int | None,
) -> bool:
    greater_filters = [
        (max_hp_filter, element.hp),
        (max_level_filter, element.level),
    ]
    for filter_value, element_value in greater_filters:
        if filter_value is not None and element_value > filter_value:
            return False
    return True


def _check_element_pass_equals_filters(
    element: Creature,
    name_filter: str | None,
    family_filter: str | None,
    rarity_filter: RarityEnum | None,
    size_filter: SizeEnum | None,
    alignment_filter: AlignmentEnum | None,
    is_melee_filter: bool | None,
    is_ranged_filter: bool | None,
    is_spell_caster_filter: bool | None,
) -> bool:
    enum_filters: list[tuple[Enum | None, Enum]] = [
        (rarity_filter, element.rarity),
        (size_filter, element.size),
        (alignment_filter, element.alignment),
    ]

    string_filters: list[tuple[str | None, str]] = [
        (name_filter, element.name.lower()),
        (family_filter, element.family.lower()),
    ]

    boolean_filters: list[tuple[bool | None, bool]] = [
        (is_melee_filter, element.is_melee),
        (is_ranged_filter, element.is_ranged),
        (is_spell_caster_filter, element.is_spell_caster),
    ]

    for filter_str_value, str_element_value in string_filters:
        if (
            filter_str_value is not None
            and filter_str_value.lower() != str_element_value
        ):
            return False

    for filter_bool_value, bool_element_value in boolean_filters:
        if filter_bool_value is not None and filter_bool_value != bool_element_value:
            return False

    for filter_enum_value, enum_element_value in enum_filters:
        if filter_enum_value is not None and filter_enum_value != enum_element_value:
            return False
    return True


def get_keys(creature_filter: CreatureFilter) -> list[str]:
    if creatures_cache:
        return sorted(creatures_cache.get_dictionary(creature_filter).keys())

    return sorted(
        redis_communicator.fetch_and_parse_all_keys(
            creature_filter.value.lower() + "*",
        ),
    )


def get_creatures_by_ids(id_list: list[str]) -> list[Creature]:
    creatures_list: list[Creature] = []
    for _id in id_list:
        curr_creature = get_creature_by_id(_id)
        if curr_creature:
            creatures_list.append(curr_creature)
    return creatures_list


def get_creature_by_id(creature_id: str) -> Creature | None:
    if creatures_cache:
        return creatures_cache.get_creature_by_id(creature_id)
    return redis_communicator.get_creature_by_id(creature_id)


def fetch_creature_ids_passing_all_filters(
    key_value_filters: dict,
) -> dict[str, dict[str, set[str]]]:
    if creatures_cache:
        ids_passing_filter: dict[str, dict[str, set[str]]] = {}
        for key, value in key_value_filters.items():
            curr_dict = fetch_creature_ids_passing_filter(key, filter_list=value)
            if not curr_dict:
                return {}
            ids_passing_filter[key] = curr_dict
        return ids_passing_filter

    # We should have a direct call like we had in the past
    # But this would increment code complexity for a
    # non-existent case. (handling filter, orders, etc..)
    # cache is empty only on startup, later on it is never emptied but always
    # overwritten.
    return {}


def fetch_creature_ids_passing_filter(
    creature_filter: CreatureFilter,
    filter_list: Iterable[str],
) -> dict[str, set[str]]:
    if creatures_cache:
        ids_passing_filter: dict[str, set[str]] = {}
        for curr_value in filter_list:
            curr_set = {
                creature.id
                for creature in creatures_cache.get_dictionary(creature_filter)[
                    curr_value
                ]
            }
            if curr_set:
                ids_passing_filter[curr_value] = curr_set
            else:
                error_string: str = (
                    f"No keys found for {creature_filter} with value {curr_value}"
                )
                logger.debug(error_string)
        return ids_passing_filter

    return {}
