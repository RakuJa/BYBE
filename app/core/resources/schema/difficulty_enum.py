from enum import Enum


class DifficultyEnum(Enum):
    TRIVIAL = "TRIVIAL"
    LOW = "LOW"
    MODERATE = "MODERATE"
    SEVERE = "SEVERE"
    EXTREME = "EXTREME"
    IMPOSSIBLE = "IMPOSSIBLE"

    def get_base_xp_budget(self):
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
            raise NotImplemented(
                f"This enumerator: {self.value} has not been implemented yet, contact the developer"
            )

    def get_xp_adjustment(self):
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
            raise NotImplemented(
                f"This enumerator: {self.value} has not been implemented yet, contact the developer"
            )
