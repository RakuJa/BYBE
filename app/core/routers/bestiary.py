from fastapi import APIRouter, Depends

from app.core.resources.schema.pagination_params import PaginationParams
from app.core.services import bestiary_service


router = APIRouter(
    prefix="/bestiary", tags=["creature"], responses={200: {"description": "success"}}
)


@router.get("/list/", responses={404: {"description": "item not found"}})
async def get_bestiary(pagination_params: PaginationParams = Depends()):
    return await bestiary_service.get_bestiary(pagination_params)
