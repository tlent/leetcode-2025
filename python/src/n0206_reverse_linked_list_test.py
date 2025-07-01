from n0206_reverse_linked_list import reverse_list, reverse_list_recursive
from utils.linked_list import List


def test_reverse_list() -> None:
    linked_list = List([1, 2, 3, 4, 5])
    linked_list.head = reverse_list(linked_list.head)
    assert linked_list.to_list() == [5, 4, 3, 2, 1]


def test_reverse_list_recursive() -> None:
    linked_list = List([1, 2, 3, 4, 5])
    linked_list.head = reverse_list_recursive(linked_list.head)
    assert linked_list.to_list() == [5, 4, 3, 2, 1]
