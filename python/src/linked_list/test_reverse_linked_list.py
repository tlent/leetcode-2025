from reverse_linked_list import reverse_list, reverse_list_recursive
from list import ListNode


def test_reverse_list() -> None:
    result = reverse_list(ListNode.from_values(range(5)))
    assert result is not None
    assert list(result.values()) == list(reversed(range(5)))


def test_reverse_list_recursive() -> None:
    result = reverse_list_recursive(ListNode.from_values(range(5)))
    assert result is not None
    assert list(result.values()) == list(reversed(range(5)))
