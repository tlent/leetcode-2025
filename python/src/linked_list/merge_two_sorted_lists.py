from list import ListNode, List


def merge(a: List, b: List) -> List:
    head = ListNode()
    cursor = head
    while a and b:
        if a.val <= b.val:
            cursor.next = a
            a = a.next
        else:
            cursor.next = b
            b = b.next
        cursor = cursor.next
    cursor.next = a if a else b
    return head.next


def test_merge() -> None:
    a = ListNode.from_iterable([1, 2, 4])
    b = ListNode.from_iterable([1, 3, 4])
    result = merge(a, b)
    assert result is not None
    assert list(result) == [1, 1, 2, 3, 4, 4]
