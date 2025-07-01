from typing import Optional
from utils.linked_list import ListNode


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
