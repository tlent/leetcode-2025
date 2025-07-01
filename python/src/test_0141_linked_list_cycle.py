from p0141_linked_list_cycle import has_cycle
from utils.linked_list import List


def test_has_cycle_example_one() -> None:
    linked_list = List([3, 2, 0, -4])
    nodes = list(linked_list.nodes())
    nodes[-1].next = nodes[1]
    assert has_cycle(linked_list.head)


def test_has_cycle_example_two() -> None:
    linked_list = List([1, 2])
    nodes = list(linked_list.nodes())
    nodes[-1].next = nodes[0]
    assert has_cycle(linked_list.head)


def test_has_cycle_example_three() -> None:
    linked_list = List([1])
    assert not has_cycle(linked_list.head)
