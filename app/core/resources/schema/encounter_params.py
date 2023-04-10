from pydantic import BaseModel, conlist


class EncounterParams(BaseModel):
    party_levels: conlist(int, min_items=1)
    enemy_levels: conlist(int, min_items=0)
