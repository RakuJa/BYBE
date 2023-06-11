from enum import Enum


class OrderEnum(Enum):
    UNORDERED: str = "UNORDERED"
    ORDERED_BY_ID: str = "ID"
    ORDERED_BY_NAME: str = "NAME"
    ORDERED_BY_HP: str = "HP"
    ORDERED_BY_LEVEL: str = "LEVEL"
    ORDERED_BY_FAMILY: str = "FAMILY"
    ORDERED_BY_ALIGNMENT: str = "ALIGNMENT"
    ORDERED_BY_SIZE: str = "SIZE"
    ORDERED_BY_RARITY: str = "RARITY"
