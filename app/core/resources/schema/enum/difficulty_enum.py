from enum import Enum
from typing import Self


class DifficultyEnum(Enum):
    TRIVIAL = "TRIVIAL"
    LOW = "LOW"
    MODERATE = "MODERATE"
    SEVERE = "SEVERE"
    EXTREME = "EXTREME"
    IMPOSSIBLE = "IMPOSSIBLE"

    # Impossible = 320 with chara adjust 60, invented by me but what else can I do?
    # Pathfinder 2E thinks that a GM will only try out extreme encounter at maximum
    # I have to introduce a level for impossible things, Needs balancing Paizo help

    def get_base_xp_budget(self: Self) -> int:
        match self:
            case DifficultyEnum.TRIVIAL:
                return 40
            case DifficultyEnum.LOW:
                return 60
            case DifficultyEnum.MODERATE:
                return 80
            case DifficultyEnum.SEVERE:
                return 120
            case DifficultyEnum.EXTREME:
                return 160
            case DifficultyEnum.IMPOSSIBLE:
                return 320
            case _:
                msg = (
                    f"This enumerator: {self.value} has not been implemented yet,"
                    f" contact the developer"
                )
                raise NotImplementedError(
                    msg,
                )

    def get_xp_adjustment(self: Self) -> int:
        match self:
            case DifficultyEnum.TRIVIAL:
                return 10
            case DifficultyEnum.LOW:
                return 15
            case DifficultyEnum.MODERATE:
                return 20
            case DifficultyEnum.SEVERE:
                return 30
            case DifficultyEnum.EXTREME:
                return 40
            case DifficultyEnum.IMPOSSIBLE:
                return 60
            case _:
                msg = (
                    f"This enumerator: {self.value} has not been implemented yet,"
                    f" contact the developer"
                )
                raise NotImplementedError(
                    msg,
                )
