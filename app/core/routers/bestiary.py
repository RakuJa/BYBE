from fastapi import APIRouter, Depends

from app.core.resources.schema.pagination_params import PaginationParams
from app.core.services import bestiary_service

router = APIRouter(
    prefix="/bestiary", tags=["bestiary"], responses={200: {"description": "success"}}
)


@router.get("/list/")
async def get_bestiary(pagination_params: PaginationParams = Depends()):
    return await bestiary_service.get_bestiary(pagination_params)


@router.get("/families/")
async def get_families_list():
    return await bestiary_service.get_families_list()


@router.get("/rarities/")
async def get_rarities_list():
    return bestiary_service.get_rarities_list()


@router.get("/sizes/")
async def get_size_list():
    return bestiary_service.get_size_list()


@router.get("/alignments/")
async def get_alignment_list():
    return bestiary_service.get_alignment_list()
