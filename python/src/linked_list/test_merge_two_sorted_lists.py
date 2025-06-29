from merge_two_sorted_lists import merge
from list import ListNode


def test_merge() -> None:
    a = ListNode.from_values([1, 2, 4])
    b = ListNode.from_values([1, 3, 4])
    result = merge(a, b)
    assert result is not None
    assert list(result.values()) == [1, 1, 2, 3, 4, 4]