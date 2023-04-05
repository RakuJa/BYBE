import json
from typing import Optional, List

from app.core.resources.schema.alignment_enum import AlignmentEnum
from app.core.resources.schema.rarity_enum import RarityEnum
from app.core.resources.schema.size_enum import SizeEnum


class Creature:
    def __init__(
        self,
        identifier: str,
        name: str,
        hp: int,
        level: int,
        alignment: AlignmentEnum,
        size: SizeEnum,
        family: Optional[str],
        rarity: Optional[RarityEnum],
    ):
        self._id = identifier
        self._name = name.strip()
        self._hp = hp
        self._level = level
        self._alignment = alignment
        self._size = size
        self._family = family
        self._rarity = rarity

    def serialize_to_json(self) -> str:
        return json.dumps(self.serialize_to_dict())

    def serialize_to_dict(self) -> dict:
        return {
            "name": self._name,
            "hp": self._hp,
            "level": self._level,
            "family": self._family if self._family else "-",
            "alignment": self._alignment.value,
            "size": self._size.value,
            "rarity": self._rarity.value if self._rarity else "-",
        }

    def get_id(self) -> str:
        return self._id

    def __str__(self):
        return self.serialize_to_json()
