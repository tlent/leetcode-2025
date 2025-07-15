from n0876_middle_of_the_linked_list import middle_node
from utils.linked_list import List


def test_middle_node() -> None:
    linked_list = List([1, 2, 3, 4, 5])
    mid = middle_node(linked_list.head)
    assert mid is not None
    assert mid.value == 3

    linked_list = List([1, 2, 3, 4, 5, 6])
    mid = middle_node(linked_list.head)
    assert mid is not None
    assert mid.value == 4
