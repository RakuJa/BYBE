from typing import List

from app.core.resources import redis_handler
from app.core.resources.schema.alignment_enum import AlignmentEnum
from app.core.resources.schema.pagination_params import PaginationParams
from app.core.resources.schema.rarity_enum import RarityEnum
from app.core.resources.schema.size_enum import SizeEnum


async def get_bestiary(pagination_params: PaginationParams) -> dict:
    next_cursor, list_of_creatures = await redis_handler.get_paginated_creatures(
        pagination_params
    )
    return {
        "results": list_of_creatures,
        "count": len(list_of_creatures),
        "next": f"https://bybe.fly.dev/bestiary/list/"
        f"?cursor={next_cursor}&page_size=100",
    }


async def get_families_list() -> List[str]:
    next_cursor, keys = await redis_handler.fetch_and_parse_keys(
        cursor=0,
        page_size=-1,
        pattern="family:*",
    )
    return keys


def get_rarities_list() -> List[str]:
    return [el.value for el in RarityEnum]


def get_size_list() -> List[str]:
    return [el.value for el in SizeEnum]


def get_alignment_list() -> List[str]:
    return [el.value for el in AlignmentEnum]
