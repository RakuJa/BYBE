from typing import Annotated, List

from pydantic import BaseModel, conlist


class EncounterParams(BaseModel):
    party_levels: Annotated[List[int], conlist(int, min_items=1)]
    enemy_levels: Annotated[List[int], conlist(int, min_items=1)]
