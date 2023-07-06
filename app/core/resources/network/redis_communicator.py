import logging
import os
from collections.abc import Iterable

import redis

from app.core.resources.app_config import config
from app.core.resources.creature import Creature

logger = logging.getLogger(__name__)

r = redis.StrictRedis(
    host=config.redis_ip,
    port=int(config.redis_port),
    password=os.environ.get("REDIS_KEY"),
)


def is_redis_up() -> bool:
    try:
        return bool(r.info())
    except Exception as e:
        error_string: str = (
            f"Exception encountered while connecting to the redis DB: {e}"
        )
        logger.warning(error_string)
        return False


def fetch_keys(cursor: int, page_size: int, pattern: str) -> tuple[int, list[bytes]]:
    keys = r.scan_iter(match=pattern)
    key_list: list[bytes] = list(keys)

    next_cursor = (
        cursor + page_size if len(key_list) > cursor + page_size else len(key_list)
    )
    return next_cursor, key_list[cursor:next_cursor]


def fetch_and_parse_all_keys(pattern: str) -> list[str]:
    parse_pattern = pattern[:-1] if pattern.endswith("*") else pattern
    return [
        key.decode("utf-8").replace(parse_pattern, "")
        for key in r.scan_iter(match=pattern)
    ]


def fetch_and_parse_keys(
    cursor: int,
    page_size: int,
    pattern: str,
) -> tuple[int, list[str]]:
    cursor, raw_key_list = fetch_keys(
        cursor=cursor,
        page_size=page_size,
        pattern=pattern,
    )
    if pattern.endswith("*"):
        pattern = pattern[:-1]
    return cursor, [key.decode("utf-8").replace(pattern, "") for key in raw_key_list]


def get_paginated_creatures(cursor: int, page_size: int) -> tuple[int, list]:
    next_cursor, keys = fetch_and_parse_keys(
        cursor=cursor,
        page_size=page_size,
        pattern="creature:*",
    )
    return next_cursor, get_creatures_by_id(keys)


def get_creatures_by_id(id_list: list[str]) -> list[Creature]:
    """
    Gets the creatures associated with the given ids
    :param id_list: list of ids to fetch
    :return: dict containing all the data of the
    """
    return [
        Creature.from_json_string(json_str=el, _id=_id)
        for _id in id_list
        for el in r.json().get(_id, "$")
    ]


def get_creature_by_id(creature_id: str) -> Creature:
    try:
        return Creature.from_json_string(
            json_str=r.json().get(creature_id, "$")[0],
            _id=creature_id,
        )
    except Exception as e:
        error_string: str = (
            f"Error encountered while fetching json with id {creature_id}: {e}"
        )
        logger.debug(error_string)
        raise


# DEPRECATED AND NOT USED
def fetch_creature_ids_passing_all_filters(
    key_value_filters: dict,
) -> dict[str, dict[str, set[str]]]:
    ids_passing_filter: dict[str, dict[str, set[str]]] = {}
    for key, value in key_value_filters.items():
        curr_dict = fetch_creature_ids_passing_filter(key, filter_list=value)
        if not curr_dict:
            return {}
        ids_passing_filter[key] = curr_dict
    return ids_passing_filter


# DEPRECATED AND NOT USED
def fetch_creature_ids_passing_filter(
    filter_name: str,
    filter_list: Iterable[str],
) -> dict[str, set[str]]:
    ids_passing_filter: dict[str, set[str]] = {}
    for curr_value in filter_list:
        curr_set = {
            key.decode("utf-8").replace("creature:", "")
            for key in r.hgetall(f"{filter_name}:{curr_value}")
        }
        if curr_set:
            ids_passing_filter[curr_value] = curr_set
        else:
            error_string: str = (
                f"No keys found for {filter_name} with value {curr_value}"
            )
            logger.debug(error_string)
    return ids_passing_filter
