from typing import List

from fastapi import Depends

from app.core.resources.api_router import APIRouter
from app.core.resources.schema.order_enum import OrderEnum
from app.core.resources.schema.pagination_params import PaginationParams
from app.core.services import bestiary_service

router = APIRouter(
    prefix="/bestiary", tags=["bestiary"], responses={200: {"description": "success"}}
)


@router.get("/list/")
async def get_bestiary(
    pagination_params: PaginationParams = Depends(),
    order: OrderEnum = OrderEnum.ORDERED_BY_ID,
) -> dict:
    return await bestiary_service.get_bestiary(pagination_params, order)


@router.get("/families/")
async def get_families_list() -> List[str]:
    return await bestiary_service.get_families_list()


@router.get("/rarities/")
async def get_rarities_list() -> List[str]:
    return await bestiary_service.get_rarities_list()


@router.get("/sizes/")
async def get_size_list() -> List[str]:
    return await bestiary_service.get_size_list()


@router.get("/alignments/")
async def get_alignment_list() -> List[str]:
    return await bestiary_service.get_alignment_list()


@router.get("/elite/")
async def get_elite_version(creature_id: str) -> dict:
    return await bestiary_service.get_elite_version(creature_id)


@router.get("/weak/")
async def get_weak_version(creature_id: str) -> dict:
    return await bestiary_service.get_weak_version(creature_id)
