from typing import Annotated

from pydantic import BaseModel, conlist


class EncounterParams(BaseModel):
    party_levels: Annotated[list[int], conlist(int, min_items=1)]
    enemy_levels: Annotated[list[int], conlist(int, min_items=1)]
