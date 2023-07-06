import copy
from typing import List, Dict, Optional

from app.core.resources.creature import Creature
from app.core.resources.schema.enum.creature_filter_enum import CreatureFilter
from app.core.resources.schema.enum.order_enum import OrderEnum
from app.core.resources.schema.enum.sort_enum import CreatureFieldsEnum


class CreatureCache:
    def __init__(
        self,
        creatures_list: List[Creature],
        level_dict: Dict[str, List[Creature]],
        family_dict: Dict[str, List[Creature]],
        alignment_dict: Dict[str, List[Creature]],
        size_dict: Dict[str, List[Creature]],
        rarity_dict: Dict[str, List[Creature]],
        melee_dict: Dict[str, List[Creature]],
        ranged_dict: Dict[str, List[Creature]],
        spell_caster_dict: Dict[str, List[Creature]],
    ):
        self.unordered = creatures_list
        self.ordered_by_id_ascending: List[Creature] = sorted(
            creatures_list, key=lambda creature: int(creature.id)
        )
        self.ordered_by_id_descending: List[Creature] = sorted(
            creatures_list, key=lambda creature: int(creature.id), reverse=True
        )

        self.ordered_by_name_ascending: List[Creature] = sorted(
            creatures_list, key=lambda creature: creature.name
        )
        self.ordered_by_name_descending: List[Creature] = sorted(
            creatures_list, key=lambda creature: creature.name, reverse=True
        )

        self.ordered_by_hp_ascending: List[Creature] = sorted(
            creatures_list, key=lambda creature: creature.hp
        )
        self.ordered_by_hp_descending: List[Creature] = sorted(
            creatures_list, key=lambda creature: creature.hp, reverse=True
        )

        self.ordered_by_level_ascending: List[Creature] = sorted(
            creatures_list, key=lambda creature: creature.level
        )
        self.ordered_by_level_descending: List[Creature] = sorted(
            creatures_list, key=lambda creature: creature.level, reverse=True
        )

        self.ordered_by_family_ascending: List[Creature] = sorted(
            creatures_list, key=lambda creature: creature.family
        )
        self.ordered_by_family_descending: List[Creature] = sorted(
            creatures_list, key=lambda creature: creature.family, reverse=True
        )

        self.ordered_by_alignment_ascending: List[Creature] = sorted(
            creatures_list, key=lambda creature: creature.alignment.value
        )
        self.ordered_by_alignment_descending: List[Creature] = sorted(
            creatures_list, key=lambda creature: creature.alignment.value, reverse=True
        )

        self.ordered_by_size_ascending: List[Creature] = sorted(
            creatures_list, key=lambda creature: creature.size.value
        )
        self.ordered_by_size_descending: List[Creature] = sorted(
            creatures_list, key=lambda creature: creature.size.value, reverse=True
        )

        self.ordered_by_rarity_ascending: List[Creature] = sorted(
            creatures_list, key=lambda creature: creature.rarity.value
        )
        self.ordered_by_rarity_descending: List[Creature] = sorted(
            creatures_list, key=lambda creature: creature.rarity.value, reverse=True
        )

        self.id_filter: Dict[str, Creature] = {x.id: x for x in creatures_list}
        self.level_filter = level_dict
        self.family_filter = family_dict
        self.alignment_filter = alignment_dict
        self.size_filter = size_dict
        self.rarity_filter = rarity_dict
        self.melee_filter = melee_dict
        self.ranged_filter = ranged_dict
        self.spell_caster_filter = spell_caster_dict

    def get_creature_by_id(self, creature_id: str) -> Optional[Creature]:
        """
        Method used to fetch creatures, it will perform
        a deep copy before returning
        :param creature_id: id of the creature to fetch
        :return: deep copy of creature or None
        """
        return copy.deepcopy(self.id_filter.get(creature_id, None))

    def get_all_dictionaries(self) -> Dict[str, Dict[str, List[Creature]]]:
        return {
            CreatureFilter.SIZE.value: self.size_filter,
            CreatureFilter.RARITY.value: self.rarity_filter,
            CreatureFilter.FAMILY.value: self.family_filter,
            CreatureFilter.LEVEL.value: self.level_filter,
            CreatureFilter.ALIGNMENT.value: self.alignment_filter,
            CreatureFilter.MELEE.value: self.melee_filter,
            CreatureFilter.RANGED.value: self.ranged_filter,
            CreatureFilter.SPELL_CASTER.value: self.spell_caster_filter,
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
            case CreatureFilter.MELEE:
                return self.melee_filter
            case CreatureFilter.RANGED:
                return self.ranged_filter
            case CreatureFilter.SPELL_CASTER:
                return self.ranged_filter
            case _:
                raise SyntaxError("Enum value is not valid for this method")

    def get_all_lists(self) -> Dict[str, List[Creature]]:
        return {
            CreatureFieldsEnum.ID.value: self.ordered_by_id_ascending,
            CreatureFieldsEnum.SIZE.value: self.ordered_by_size_ascending,
            CreatureFieldsEnum.RARITY.value: self.ordered_by_rarity_ascending,
            CreatureFieldsEnum.HP.value: self.ordered_by_hp_ascending,
            CreatureFieldsEnum.LEVEL.value: self.ordered_by_level_ascending,
            CreatureFieldsEnum.NAME.value: self.ordered_by_name_ascending,
            CreatureFieldsEnum.ALIGNMENT.value: self.ordered_by_alignment_ascending,
            CreatureFieldsEnum.FAMILY.value: self.ordered_by_family_ascending,
        }

    def get_list(
        self,
        sort_filter: Optional[CreatureFieldsEnum],
        order: OrderEnum = OrderEnum.ASCENDING,
    ) -> List[Creature]:
        match (sort_filter, order):
            case (CreatureFieldsEnum.ID, OrderEnum.ASCENDING):
                return self.ordered_by_id_ascending
            case (CreatureFieldsEnum.ID, OrderEnum.DESCENDING):
                return self.ordered_by_id_descending

            case (CreatureFieldsEnum.HP, OrderEnum.ASCENDING):
                return self.ordered_by_hp_ascending
            case (CreatureFieldsEnum.HP, OrderEnum.DESCENDING):
                return self.ordered_by_hp_descending

            case (CreatureFieldsEnum.FAMILY, OrderEnum.ASCENDING):
                return self.ordered_by_family_ascending
            case (CreatureFieldsEnum.FAMILY, OrderEnum.DESCENDING):
                return self.ordered_by_family_descending

            case (CreatureFieldsEnum.ALIGNMENT, OrderEnum.ASCENDING):
                return self.ordered_by_alignment_ascending
            case (CreatureFieldsEnum.ALIGNMENT, OrderEnum.DESCENDING):
                return self.ordered_by_alignment_descending

            case (CreatureFieldsEnum.LEVEL, OrderEnum.ASCENDING):
                return self.ordered_by_level_ascending
            case (CreatureFieldsEnum.LEVEL, OrderEnum.DESCENDING):
                return self.ordered_by_level_descending

            case (CreatureFieldsEnum.NAME, OrderEnum.ASCENDING):
                return self.ordered_by_name_ascending
            case (CreatureFieldsEnum.NAME, OrderEnum.DESCENDING):
                return self.ordered_by_name_descending

            case (CreatureFieldsEnum.RARITY, OrderEnum.ASCENDING):
                return self.ordered_by_rarity_ascending
            case (CreatureFieldsEnum.RARITY, OrderEnum.DESCENDING):
                return self.ordered_by_rarity_descending

            case (CreatureFieldsEnum.SIZE, OrderEnum.ASCENDING):
                return self.ordered_by_size_ascending
            case (CreatureFieldsEnum.SIZE, OrderEnum.DESCENDING):
                return self.ordered_by_size_descending
            case _:
                return self.unordered
