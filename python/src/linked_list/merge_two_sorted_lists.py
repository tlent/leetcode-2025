from typing import Optional
from linked_list import ListNode


def merge(a: Optional[ListNode], b: Optional[ListNode]) -> Optional[ListNode]:
    head = ListNode()
    cursor = head
    while a and b:
        if a.value <= b.value:
            cursor.next = a
            a = a.next
        else:
            cursor.next = b
            b = b.next
        cursor = cursor.next
    cursor.next = a if a else b
    return head.next
