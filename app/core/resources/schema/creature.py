import json
from typing import Optional, Self

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
        self.id = identifier
        self.name = name.strip()
        self.hp = hp
        self.level = level
        self.alignment = alignment
        self.size = size
        self.family = family
        self.rarity = rarity
        self.archive_link = f"https://2e.aonprd.com/Monsters.aspx?ID={self.id}"

    def serialize_to_json(self) -> str:
        return json.dumps(self.serialize_to_dict())

    def serialize_to_dict(self) -> dict:
        return {
            "id": self.id,
            "name": self.name,
            "hp": self.hp,
            "level": self.level,
            "family": self.family,
            "alignment": self.alignment.value,
            "size": self.size.value,
            # "rarity": self.rarity.value,
            "archive_link": self.archive_link,
        }

    @classmethod
    def from_dict(cls, creature_dict: dict, _id: str) -> Self:
        return cls(
            identifier=_id,
            name=creature_dict["name"],
            hp=creature_dict["hp"],
            level=creature_dict["level"],
            family=creature_dict["family"],
            alignment=creature_dict["alignment"],
            size=creature_dict["size"],
            rarity=creature_dict["rarity"],
        )

    @classmethod
    def from_json_string(cls, json_str: str, _id: str) -> Self:
        return cls.from_dict(creature_dict=json.loads(json_str), _id=_id)

    def __str__(self) -> str:
        return self.serialize_to_json()
