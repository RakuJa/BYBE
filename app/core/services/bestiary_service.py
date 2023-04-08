from app.core.resources import redis_handler
from app.core.resources.app_config import config
from app.core.resources.schema.pagination_params import PaginationParams


async def get_bestiary(pagination_params: PaginationParams) -> dict:
    next_cursor, list_of_creatures = await redis_handler.get_paginated_creatures(
        pagination_params
    )
    return {
        "results": list_of_creatures,
        "count": len(list_of_creatures),
        "next": f"{config.service_ip}:{config.service_port}"
        f"/bestiary/get_bestiary/"
        f"?cursor={next_cursor}&page_size=100",
    }
