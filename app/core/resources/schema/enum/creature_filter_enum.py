from enum import Enum


class CreatureFilter(Enum):
    ID: str = "ID"
    LEVEL: str = "LEVEL"
    FAMILY: str = "FAMILY"
    ALIGNMENT: str = "ALIGNMENT"
    SIZE: str = "SIZE"
    RARITY: str = "RARITY"
    MELEE: str = "MELEE"
    RANGED: str = "RANGED"
    SPELL_CASTER: str = "SPELL_CASTER"
