from fastapi import APIRouter

from app.core.resources.schema.encounter_params import EncounterParams
from app.core.services import encounter_service

router = APIRouter(
    prefix="/encounter",
    tags=["encounter"],
    responses={200: {"description": "success"}},
)


@router.post("/info/")
async def get_encounter_info(encounter_params: EncounterParams):
    return encounter_service.get_encounter_info(encounter_params)
