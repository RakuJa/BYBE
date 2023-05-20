from typing import List

from app.core.resources.network import redis_proxy
from app.core.resources.schema.creature import Creature
from app.core.resources.schema.creature_filter import CreatureFilter
from app.core.resources.schema.order_enum import OrderEnum
from app.core.resources.schema.pagination_params import PaginationParams


async def get_bestiary(
    pagination_params: PaginationParams,
    order: OrderEnum,
    name_filter: str,
) -> dict:
    next_cursor, list_of_creatures = await redis_proxy.get_paginated_creatures(
        cursor=pagination_params.cursor,
        page_size=pagination_params.page_size,
        order=order,
        name_filter=name_filter,
    )
    end_of_next_field = ""
    if name_filter:
        end_of_next_field = f"&name_filter={name_filter}"

    return {
        "results": list_of_creatures,
        "count": len(list_of_creatures),
        "next": f"https://bybe.fly.dev/bestiary/list/"
        f"?order={order.value}&cursor={next_cursor}"
        f"&page_size={pagination_params.page_size}"
        f"{end_of_next_field}"
        if len(list_of_creatures) >= pagination_params.page_size
        else None,
    }


async def get_families_list() -> List[str]:
    return await redis_proxy.get_keys(CreatureFilter.FAMILY)


async def get_rarities_list() -> List[str]:
    return await redis_proxy.get_keys(CreatureFilter.RARITY)


async def get_size_list() -> List[str]:
    return await redis_proxy.get_keys(CreatureFilter.SIZE)


async def get_alignment_list() -> List[str]:
    return await redis_proxy.get_keys(CreatureFilter.ALIGNMENT)


async def get_creature(creature_id: str) -> dict:
    return {"results": await redis_proxy.get_creature_by_id(creature_id)}


async def get_elite_version(creature_id: str) -> dict:
    hp_increase = {1: 10, 2: 15, 5: 20, 20: 30}
    return {"results": await __update_creature(creature_id, hp_increase, 1)}


async def get_weak_version(creature_id: str) -> dict:
    hp_increase = {1: -10, 2: -15, 5: -20, 20: -30}
    return {"results": await __update_creature(creature_id, hp_increase, -1)}


async def __update_creature(
    creature_id: str,
    hp_increase: dict,
    level_delta: int,
) -> Creature:
    creature = await redis_proxy.get_creature_by_id(creature_id)
    # finds the bigger key in hp_increase where the creature's level
    # is greater than or equal to the key.
    creature.hp += hp_increase.get(
        max(
            (key for key in hp_increase.keys() if creature.level >= key),
            default=next(iter(hp_increase)),
        ),
        0,
    )

    creature.hp = creature.hp if creature.hp > 0 else 1

    creature.level += level_delta
    archive_query = "Elite" if level_delta >= 1 else "Weak"
    creature.archive_link = f"{creature.archive_link}&{archive_query}=true"
    return creature
