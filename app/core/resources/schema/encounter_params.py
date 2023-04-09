from typing import List

from pydantic import BaseModel


class EncounterParams(BaseModel):
    party_levels: List[int]
    enemy_levels: List[int]
