import logging
from typing import Tuple, List

import redis

from app.core.resources.app_config import config
from app.core.resources.schema.pagination_params import PaginationParams

logger = logging.getLogger(__name__)

r = redis.StrictRedis(
    host=config.service_ip, port=config.service_port, db=config.redis_db
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
    return next_cursor, await get_creatures_by_ids(parsed_keys)


async def get_creatures_by_ids(id_list: List[str]) -> List[str]:
    return [r.json().get(_id) for _id in id_list]


async def fetch_creature_by_id_and_filter():
    result = r.hgetall("level:10")
    for hash_key, json_str in result.items():
        # gets the obj
        r.json().get(int(json_str))
