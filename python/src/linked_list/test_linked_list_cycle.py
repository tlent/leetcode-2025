from linked_list_cycle import has_cycle
from list import ListNode


def test_has_cycle_example_one() -> None:
    values = ListNode.from_values([3, 2, 0, -4])
    assert values is not None
    nodes = list(values.nodes())
    nodes[-1].next = nodes[1]
    assert has_cycle(values)


def test_has_cycle_example_two() -> None:
    values = ListNode.from_values([1, 2])
    assert values is not None
    nodes = list(values.nodes())
    nodes[-1].next = nodes[0]
    assert has_cycle(values)


def test_has_cycle_example_three() -> None:
    values = ListNode.from_values([1])
    assert values is not None
    assert not has_cycle(values)
