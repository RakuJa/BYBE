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
        if self == DifficultyEnum.TRIVIAL:
            return 40
        if self == DifficultyEnum.LOW:
            return 60
        if self == DifficultyEnum.MODERATE:
            return 80
        if self == DifficultyEnum.SEVERE:
            return 120
        if self == DifficultyEnum.EXTREME:
            return 160
        if self == DifficultyEnum.IMPOSSIBLE:
            return 320

        msg = (
            f"This enumerator: {self.value} has not been implemented yet,"
            f" contact the developer"
        )
        raise NotImplementedError(
            msg,
        )

    def get_xp_adjustment(self: Self) -> int:
        if self == DifficultyEnum.TRIVIAL:
            return 10
        if self == DifficultyEnum.LOW:
            return 15
        if self == DifficultyEnum.MODERATE:
            return 20
        if self == DifficultyEnum.SEVERE:
            return 30
        if self == DifficultyEnum.EXTREME:
            return 40
        if self == DifficultyEnum.IMPOSSIBLE:
            return 60

        msg = (
            f"This enumerator: {self.value} has not been implemented yet,"
            f" contact the developer"
        )
        raise NotImplementedError(
            msg,
        )
