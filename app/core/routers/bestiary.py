from typing import List, Optional

from fastapi import Depends

from app.core.resources.api_router import APIRouter
from app.core.resources.schema.order_enum import OrderEnum
from app.core.resources.schema.pagination_params import PaginationParams
from app.core.services import bestiary_service

router = APIRouter(
    prefix="/bestiary", tags=["bestiary"], responses={200: {"description": "success"}}
)


@router.get("/list/")
def get_bestiary(
    pagination_params: PaginationParams = Depends(),
    order: OrderEnum = OrderEnum.ORDERED_BY_ID,
    name_filter: Optional[str] = None,
) -> dict:
    return bestiary_service.get_bestiary(pagination_params, order, name_filter)


@router.get("/families/")
def get_families_list() -> List[str]:
    return bestiary_service.get_families_list()


@router.get("/rarities/")
def get_rarities_list() -> List[str]:
    return bestiary_service.get_rarities_list()


@router.get("/sizes/")
def get_size_list() -> List[str]:
    return bestiary_service.get_size_list()


@router.get("/alignments/")
def get_alignment_list() -> List[str]:
    return bestiary_service.get_alignment_list()


@router.get("/")
def get_creature(creature_id: str) -> dict:
    return bestiary_service.get_creature(creature_id)


@router.get("/elite/")
def get_elite_version(creature_id: str) -> dict:
    return bestiary_service.get_elite_version(creature_id)


@router.get("/weak/")
def get_weak_version(creature_id: str) -> dict:
    return bestiary_service.get_weak_version(creature_id)
