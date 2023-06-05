from enum import Enum


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

    def get_base_xp_budget(self) -> int:
        if self == DifficultyEnum.TRIVIAL:
            return 40
        elif self == DifficultyEnum.LOW:
            return 60
        elif self == DifficultyEnum.MODERATE:
            return 80
        elif self == DifficultyEnum.SEVERE:
            return 120
        elif self == DifficultyEnum.EXTREME:
            return 160
        elif self == DifficultyEnum.IMPOSSIBLE:
            return 320
        else:
            raise NotImplementedError(
                f"This enumerator: {self.value} has not been implemented yet,"
                f" contact the developer"
            )

    def get_xp_adjustment(self) -> int:
        if self == DifficultyEnum.TRIVIAL:
            return 10
        elif self == DifficultyEnum.LOW:
            return 15
        elif self == DifficultyEnum.MODERATE:
            return 20
        elif self == DifficultyEnum.SEVERE:
            return 30
        elif self == DifficultyEnum.EXTREME:
            return 40
        elif self == DifficultyEnum.IMPOSSIBLE:
            return 60
        else:
            raise NotImplementedError(
                f"This enumerator: {self.value} has not been implemented yet,"
                f" contact the developer"
            )
