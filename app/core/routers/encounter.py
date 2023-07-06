import random
from typing import Annotated

from pydantic import conlist

from app.core.resources.api_router import APIRouter
from app.core.resources.schema.enum.alignment_enum import AlignmentEnum
from app.core.resources.schema.enum.difficulty_enum import DifficultyEnum
from app.core.resources.schema.enum.rarity_enum import RarityEnum
from app.core.resources.schema.enum.size_enum import SizeEnum
from app.core.resources.schema.models.encounter_params import EncounterParams
from app.core.services import encounter_service

router = APIRouter(
    prefix="/encounter",
    tags=["encounter"],
    responses={200: {"description": "success"}},
)


@router.post("/info/")
def get_encounter_info(encounter_params: EncounterParams) -> dict:
    return encounter_service.get_encounter_info(encounter_params)


@router.post("/generator/")
def generate_random_encounter(
    party_levels: Annotated[list[int], conlist(int, min_items=1)],
    family: str | None = None,
    rarity: RarityEnum | None = None,
    size: SizeEnum | None = None,
    alignment: AlignmentEnum | None = None,
    encounter_difficulty: DifficultyEnum | None = None,
) -> dict:
    if not encounter_difficulty:
        encounter_difficulty = random.choice(list(DifficultyEnum))  # noqa: S311
    try:
        return encounter_service.generate_random_encounter(
            party_levels=party_levels,
            family=family,
            rarity=rarity,
            size=size,
            alignment=alignment,
            encounter_difficulty=encounter_difficulty,
        )
    except ValueError:
        return {
            "results": [],
            "count": 0,
            "experience": 0,
            "difficulty": DifficultyEnum.TRIVIAL,
            "levels": {
                "TRIVIAL": 10,
                "LOW": 15,
                "MODERATE": 20,
                "SEVERE": 30,
                "EXTREME": 40,
                "IMPOSSIBLE": 140,
            },
        }
