from typing import List, Dict, Optional

from app.core.resources.schema.creature import Creature
from app.core.resources.schema.creature_filter import CreatureFilter
from app.core.resources.schema.order_enum import OrderEnum


class CreatureCache:
    def __init__(
        self,
        creatures_list: List[Creature],
        level_dict: Dict[str, List[Creature]],
        family_dict: Dict[str, List[Creature]],
        alignment_dict: Dict[str, List[Creature]],
        size_dict: Dict[str, List[Creature]],
        rarity_dict: Dict[str, List[Creature]],
    ):
        self.unordered = creatures_list
        self.ordered_by_id: List[Creature] = sorted(
            creatures_list, key=lambda creature: creature.id
        )
        self.ordered_by_name: List[Creature] = sorted(
            creatures_list, key=lambda creature: creature.name
        )
        self.ordered_by_hp: List[Creature] = sorted(
            creatures_list, key=lambda creature: creature.hp
        )
        self.ordered_by_level: List[Creature] = sorted(
            creatures_list, key=lambda creature: creature.level
        )
        self.ordered_by_family: List[Creature] = sorted(
            creatures_list, key=lambda creature: creature.family
        )
        self.ordered_by_alignment: List[Creature] = sorted(
            creatures_list, key=lambda creature: creature.alignment.value
        )
        self.ordered_by_size: List[Creature] = sorted(
            creatures_list, key=lambda creature: creature.size.value
        )
        self.ordered_by_rarity: List[Creature] = sorted(
            creatures_list, key=lambda creature: creature.rarity.value
        )

        self.id_filter: Dict[str, Creature] = {x.id: x for x in creatures_list}
        self.level_filter = level_dict
        self.family_filter = family_dict
        self.alignment_filter = alignment_dict
        self.size_filter = size_dict
        self.rarity_filter = rarity_dict

    def get_all_dictionaries(self) -> Dict[str, Dict[str, List[Creature]]]:
        return {
            CreatureFilter.SIZE.value: self.size_filter,
            CreatureFilter.RARITY.value: self.rarity_filter,
            CreatureFilter.FAMILY.value: self.family_filter,
            CreatureFilter.LEVEL.value: self.level_filter,
            CreatureFilter.ALIGNMENT.value: self.alignment_filter,
        }

    def get_dictionary(
        self, creature_filter: CreatureFilter
    ) -> Dict[str, List[Creature]]:
        match creature_filter:
            # Filter dicts
            case CreatureFilter.SIZE:
                return self.size_filter
            case CreatureFilter.FAMILY:
                return self.family_filter
            case CreatureFilter.RARITY:
                return self.rarity_filter
            case CreatureFilter.ALIGNMENT:
                return self.alignment_filter
            case CreatureFilter.LEVEL:
                return self.level_filter
            case _:
                raise SyntaxError("Enum value is not valid for this method")

    def get_all_lists(self) -> Dict[str, List[Creature]]:
        return {
            OrderEnum.ORDERED_BY_ID.value: self.ordered_by_id,
            OrderEnum.ORDERED_BY_SIZE.value: self.ordered_by_size,
            OrderEnum.ORDERED_BY_RARITY.value: self.ordered_by_rarity,
            OrderEnum.ORDERED_BY_HP.value: self.ordered_by_hp,
            OrderEnum.ORDERED_BY_LEVEL.value: self.ordered_by_level,
            OrderEnum.ORDERED_BY_NAME.value: self.ordered_by_name,
            OrderEnum.ORDERED_BY_ALIGNMENT.value: self.ordered_by_alignment,
            OrderEnum.ORDERED_BY_FAMILY.value: self.ordered_by_family,
        }

    def get_list(self, ordered_filter: Optional[OrderEnum] = None) -> List[Creature]:
        match ordered_filter:
            case OrderEnum.ORDERED_BY_ID:
                return self.ordered_by_id
            case OrderEnum.ORDERED_BY_HP:
                return self.ordered_by_hp
            case OrderEnum.ORDERED_BY_FAMILY:
                return self.ordered_by_family
            case OrderEnum.ORDERED_BY_ALIGNMENT:
                return self.ordered_by_alignment
            case OrderEnum.ORDERED_BY_LEVEL:
                return self.ordered_by_level
            case OrderEnum.ORDERED_BY_NAME:
                return self.ordered_by_name
            case OrderEnum.ORDERED_BY_RARITY:
                return self.ordered_by_rarity
            case OrderEnum.ORDERED_BY_SIZE:
                return self.ordered_by_size
            case _:
                return self.unordered
