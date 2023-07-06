from typing import List, Optional

from app.core.resources.network import redis_proxy
from app.core.resources.creature import Creature
from app.core.resources.schema.enum.alignment_enum import AlignmentEnum
from app.core.resources.schema.enum.creature_filter_enum import CreatureFilter
from app.core.resources.schema.enum.order_enum import OrderEnum
from app.core.resources.schema.enum.rarity_enum import RarityEnum
from app.core.resources.schema.enum.size_enum import SizeEnum
from app.core.resources.schema.enum.sort_enum import CreatureFieldsEnum
from app.core.resources.schema.models.pagination_params import PaginationParams


def get_bestiary(
    pagination_params: PaginationParams,
    sort_field: CreatureFieldsEnum,
    order: OrderEnum,
    name_filter: Optional[str],
    family_filter: Optional[str],
    rarity_filter: Optional[RarityEnum],
    size_filter: Optional[SizeEnum],
    alignment_filter: Optional[AlignmentEnum],
    min_hp_filter: Optional[int],
    max_hp_filter: Optional[int],
    min_level_filter: Optional[int],
    max_level_filter: Optional[int],
    is_melee_filter: Optional[bool],
    is_ranged_filter: Optional[bool],
    is_spell_caster_filter: Optional[bool],
) -> dict:
    paginated_result = redis_proxy.get_paginated_creatures(
        cursor=pagination_params.cursor,
        page_size=pagination_params.page_size,
        sort_field=sort_field,
        order=order,
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
    )

    next_cursor, list_of_creatures = paginated_result.value_or((0, []))

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


def get_families_list() -> List[str]:
    return redis_proxy.get_keys(CreatureFilter.FAMILY)


def get_rarities_list() -> List[str]:
    return redis_proxy.get_keys(CreatureFilter.RARITY)


def get_size_list() -> List[str]:
    return redis_proxy.get_keys(CreatureFilter.SIZE)


def get_alignment_list() -> List[str]:
    return redis_proxy.get_keys(CreatureFilter.ALIGNMENT)


def get_creature(creature_id: str) -> dict:
    return {"results": redis_proxy.get_creature_by_id(creature_id)}


def get_elite_version(creature_id: str) -> dict:
    hp_increase = {1: 10, 2: 15, 5: 20, 20: 30}
    return {"results": __update_creature(creature_id, hp_increase, 1)}


def get_weak_version(creature_id: str) -> dict:
    hp_increase = {1: -10, 2: -15, 5: -20, 20: -30}
    return {"results": __update_creature(creature_id, hp_increase, -1)}


def __update_creature(
    creature_id: str,
    hp_increase: dict,
    level_delta: int,
) -> Optional[Creature]:
    creature: Optional[Creature] = redis_proxy.get_creature_by_id(creature_id)
    if not creature:
        return None
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
