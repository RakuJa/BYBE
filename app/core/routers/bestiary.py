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
    prefix="/bestiary",
    tags=["bestiary"],
    responses={200: {"description": "success"}},
)


@router.get("/list/")
def get_bestiary(
    pagination_params: PaginationParams = Depends(),
    sort_field: CreatureFieldsEnum = CreatureFieldsEnum.ID,
    order: OrderEnum = OrderEnum.ASCENDING,
    name_filter: str | None = None,
    family_filter: str | None = None,
    rarity_filter: RarityEnum | None = None,
    size_filter: SizeEnum | None = None,
    alignment_filter: AlignmentEnum | None = None,
    min_hp_filter: int | None = None,
    max_hp_filter: int | None = None,
    min_level_filter: int | None = None,
    max_level_filter: int | None = None,
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
        min_hp_filter=min_hp_filter,
        max_hp_filter=max_hp_filter,
        min_level_filter=min_level_filter,
        max_level_filter=max_level_filter,
    )


@router.get("/families/")
def get_families_list() -> list[str]:
    return bestiary_service.get_families_list()


@router.get("/rarities/")
def get_rarities_list() -> list[str]:
    return bestiary_service.get_rarities_list()


@router.get("/sizes/")
def get_size_list() -> list[str]:
    return bestiary_service.get_size_list()


@router.get("/alignments/")
def get_alignment_list() -> list[str]:
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
