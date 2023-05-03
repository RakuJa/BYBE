import random

from typing import Optional
from pydantic import conlist

from app.core.resources.api_router import APIRouter
from app.core.resources.schema.alignment_enum import AlignmentEnum
from app.core.resources.schema.difficulty_enum import DifficultyEnum
from app.core.resources.schema.encounter_params import EncounterParams
from app.core.resources.schema.rarity_enum import RarityEnum
from app.core.resources.schema.size_enum import SizeEnum
from app.core.services import encounter_service

router = APIRouter(
    prefix="/encounter",
    tags=["encounter"],
    responses={200: {"description": "success"}},
)


@router.post("/info/")
async def get_encounter_info(encounter_params: EncounterParams):
    return encounter_service.get_encounter_info(encounter_params)


@router.post("/generator/")
async def generate_random_encounter(
    party_levels: conlist(int, min_items=1),
    family: Optional[str] = None,
    rarity: Optional[RarityEnum] = None,
    size: Optional[SizeEnum] = None,
    alignment: Optional[AlignmentEnum] = None,
    encounter_difficulty: Optional[DifficultyEnum] = None,
):
    if not encounter_difficulty:
        encounter_difficulty = random.choice(list(DifficultyEnum))  # nosec
    return await encounter_service.generate_random_encounter(
        party_levels=party_levels,
        family=family,
        rarity=rarity,
        size=size,
        alignment=alignment,
        encounter_difficulty=encounter_difficulty,
    )
