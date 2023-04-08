import json
import logging
import redis
import os
from typing import Tuple, List

from app.core.resources.app_config import config
from app.core.resources.schema.pagination_params import PaginationParams

logger = logging.getLogger(__name__)

r = redis.StrictRedis(
    host=config.redis_ip,
    port=config.redis_port,
    password=os.environ.get('REDIS_KEY')
)


async def is_redis_up() -> bool:
    try:
        return True if r.info() else False
    except Exception as e:
        logger.warning(f"Exception encountered while connecting to the redis DB: {e}")


async def fetch_keys(
        cursor: int, page_size: int, pattern: str
) -> Tuple[int, List[bytes]]:
    keys = r.scan_iter(match=pattern)
    keys_list: List[bytes] = list(keys)

    next_cursor = (
        cursor + page_size if len(keys_list) > cursor + page_size else len(keys_list)
    )
    return next_cursor, keys_list[cursor:next_cursor]


async def fetch_keys_dep(
        page_number: int, page_size: int, pattern: str = "creature:*"
) -> Tuple[int, List[int]]:
    cursor = page_number * page_size
    cursor, keys = r.scan(cursor=cursor, count=page_size, match=pattern)
    return cursor, keys


async def get_paginated_creatures(
        pagination_params: PaginationParams,
) -> Tuple[int, List]:
    next_cursor, keys = await fetch_keys(
        cursor=pagination_params.cursor,
        page_size=pagination_params.page_size,
        pattern="creature:*",
    )
    parsed_keys = [key.decode("utf-8").replace("creature:", "") for key in keys]
    return next_cursor, await _json_get_all_elements_of_list(parsed_keys)


async def _json_get_all_elements_of_list(id_list: List[str]) -> List[dict]:
    json_list: List[dict] = []
    for _id in id_list:
        try:
            for el in r.json().get(_id, "$"):
                json_list.append(json.loads(el))
        except Exception as e:
            logger.debug(f"Error encountered while fetching json with id {_id}: {e}")
            raise
    return json_list


async def fetch_creature_by_id_and_filter():
    result = r.hgetall("level:10")
    for hash_key, json_str in result.items():
        # gets the obj
        r.json().get(int(json_str))
