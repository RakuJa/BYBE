import ast
import json
from typing import Self

from app.core.resources.schema.enum.alignment_enum import AlignmentEnum
from app.core.resources.schema.enum.boolean_enum import BooleanEnum
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
        is_melee: BooleanEnum,
        is_ranged: BooleanEnum,
        is_spell_caster: BooleanEnum,
        source: list[str],
    ) -> None:
        self.id: str = identifier
        self.name: str = name.strip()
        self.hp: int = hp
        self.level: int = level
        self.alignment: AlignmentEnum = AlignmentEnum(alignment)
        self.size: SizeEnum = SizeEnum(size)
        self.family: str = family if family else "-"
        self.rarity: RarityEnum = RarityEnum(rarity) if rarity else RarityEnum.COMMON
        self.is_melee: bool = bool(is_melee)
        self.is_ranged: bool = bool(is_ranged)
        self.is_spell_caster: bool = bool(is_spell_caster)
        self.sources: list[str] = source
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
            "rarity": self.rarity.value,
            "archive_link": self.archive_link,
            "is_melee": self.is_melee,
            "is_ranged": self.is_ranged,
            "is_spell_caster": self.is_spell_caster,
            "sources": self.sources,
        }

    @classmethod
    def from_dict(cls: type["Creature"], creature_dict: dict, _id: str) -> "Creature":
        try:
            source_list: list[str] = ast.literal_eval(creature_dict["sources"])
        except ValueError:
            source_list = []
        return cls(
            identifier=_id,
            name=creature_dict["name"],
            hp=creature_dict["hp"],
            level=creature_dict["level"],
            family=creature_dict["family"],
            alignment=creature_dict["alignment"],
            size=creature_dict["size"],
            rarity=creature_dict["rarity"],
            is_melee=BooleanEnum(creature_dict["is_melee"]),
            is_ranged=BooleanEnum(creature_dict["is_ranged"]),
            is_spell_caster=BooleanEnum(creature_dict["is_spell_caster"]),
            source=source_list,
        )

    @classmethod
    def from_json_string(cls: type["Creature"], json_str: str, _id: str) -> "Creature":
        return cls.from_dict(creature_dict=json.loads(json_str), _id=_id)

    def __str__(self: Self) -> str:
        return self.serialize_to_json()
