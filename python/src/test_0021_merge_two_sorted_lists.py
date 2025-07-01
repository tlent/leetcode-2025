from p0021_merge_two_sorted_lists import merge
from utils.linked_list import List


def test_merge() -> None:
    a = List([1, 2, 4])
    b = List([1, 3, 4])
    a.head = merge(a.head, b.head)
    assert a.to_list() == [1, 1, 2, 3, 4, 4]
