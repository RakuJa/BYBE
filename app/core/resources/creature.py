import json
from typing import Self

from app.core.resources.schema.enum.alignment_enum import AlignmentEnum
from app.core.resources.schema.enum.rarity_enum import RarityEnum
from app.core.resources.schema.enum.size_enum import SizeEnum


class Creature:
    def __init__(
        self: Self,
        identifier: str,
        name: str,
        hp: int,
        level: int,
        alignment: str,
        size: str,
        family: str | None,
        rarity: str | None,
    ) -> None:
        self.id: str = identifier
        self.name: str = name.strip()
        self.hp: int = hp
        self.level: int = level
        self.alignment: AlignmentEnum = AlignmentEnum(alignment)
        self.size: SizeEnum = SizeEnum(size)
        self.family: str = family if family else "-"
        self.rarity: RarityEnum = RarityEnum(rarity) if rarity else RarityEnum.COMMON
        self.archive_link: str = f"https://2e.aonprd.com/Monsters.aspx?ID={self.id}"

    def serialize_to_json(self: Self) -> str:
        return json.dumps(self.serialize_to_dict())

    def get_id(self: Self) -> str:
        return self.id

    def serialize_to_dict(self: Self) -> dict:
        return {
            "id": self.id,
            "name": self.name,
            "hp": self.hp,
            "level": self.level,
            "family": self.family,
            "alignment": self.alignment.value,
            "size": self.size.value,
            "archive_link": self.archive_link,
        }

    @classmethod
    def from_dict(cls: type["Creature"], creature_dict: dict, _id: str) -> "Creature":
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
    def from_json_string(cls: type["Creature"], json_str: str, _id: str) -> "Creature":
        return cls.from_dict(creature_dict=json.loads(json_str), _id=_id)

    def __str__(self: Self) -> str:
        return self.serialize_to_json()
