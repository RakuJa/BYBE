from typing import Optional, List

from fastapi import Depends

from app.core.resources.api_router import APIRouter
from app.core.resources.schema.enum.alignment_enum import AlignmentEnum
from app.core.resources.schema.enum.order_enum import OrderEnum
from app.core.resources.schema.enum.rarity_enum import RarityEnum
from app.core.resources.schema.enum.size_enum import SizeEnum
from app.core.resources.schema.enum.sort_enum import CreatureFieldsEnum
from app.core.resources.schema.models.pagination_params import PaginationParams
from app.core.services import bestiary_service

router = APIRouter(
    prefix="/bestiary", tags=["bestiary"], responses={200: {"description": "success"}}
)


@router.get("/list/")
def get_bestiary(
    pagination_params: PaginationParams = Depends(),
    sort_field: CreatureFieldsEnum = CreatureFieldsEnum.ID,
    order: OrderEnum = OrderEnum.ASCENDING,
    name_filter: Optional[str] = None,
    family_filter: Optional[str] = None,
    rarity_filter: Optional[RarityEnum] = None,
    size_filter: Optional[SizeEnum] = None,
    alignment_filter: Optional[AlignmentEnum] = None,
) -> dict:
    return bestiary_service.get_bestiary(
        pagination_params=pagination_params,
        sort_field=sort_field,
        order=order,
        name_filter=name_filter,
        family_filter=family_filter,
        rarity_filter=rarity_filter,
        size_filter=size_filter,
        alignment_filter=alignment_filter,
    )


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
