from __future__ import annotations
from typing import Optional


# Definition for singly-linked list.
class ListNode:
    def __init__(self, val: int = 0, next: Optional[ListNode] = None):
        self.val = val
        self.next = next


def reverse_list(head: Optional[ListNode]) -> Optional[ListNode]:
    prev = None
    cursor = head
    while cursor:
        next = cursor.next
        cursor.next = prev
        prev = cursor
        cursor = next
    return prev


def reverse_list_recursive(head: Optional[ListNode]) -> Optional[ListNode]:
    if not head or not head.next:
        return head

    new_head = reverse_list_recursive(head.next)
    head.next.next = head
    head.next = None
    return new_head


def test_reverse_list() -> None:
    list = ListNode(1, ListNode(2, ListNode(3, ListNode(4, ListNode(5)))))

    expected: Optional[ListNode] = ListNode(
        5, ListNode(4, ListNode(3, ListNode(2, ListNode(1))))
    )
    actual = reverse_list(list)

    while expected and actual:
        assert expected.val == actual.val
        expected = expected.next
        actual = actual.next
    assert expected is None
    assert actual is None


def test_reverse_list_recursive() -> None:
    list = ListNode(1, ListNode(2, ListNode(3, ListNode(4, ListNode(5)))))

    expected: Optional[ListNode] = ListNode(
        5, ListNode(4, ListNode(3, ListNode(2, ListNode(1))))
    )
    actual = reverse_list_recursive(list)

    while expected and actual:
        assert expected.val == actual.val
        expected = expected.next
        actual = actual.next
    assert expected is None
    assert actual is None
