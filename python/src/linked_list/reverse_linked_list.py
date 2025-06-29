from list import ListNode, List


def reverse_list(head: List) -> List:
    prev = None
    cursor = head
    while cursor:
        next = cursor.next
        cursor.next = prev
        prev = cursor
        cursor = next
    return prev


def reverse_list_recursive(head: List) -> List:
    if not head or not head.next:
        return head

    new_head = reverse_list_recursive(head.next)
    head.next.next = head
    head.next = None
    return new_head


def test_reverse_list() -> None:
    result = reverse_list(ListNode.from_values(range(5)))
    assert result is not None
    assert list(result.values()) == list(reversed(range(5)))


def test_reverse_list_recursive() -> None:
    result = reverse_list_recursive(ListNode.from_values(range(5)))
    assert result is not None
    assert list(result.values()) == list(reversed(range(5)))
