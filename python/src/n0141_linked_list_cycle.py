from typing import Optional
from utils.linked_list import ListNode


def has_cycle(head: Optional[ListNode]) -> bool:
    slow = head
    fast = head
    while slow and fast and fast.next:
        slow = slow.next
        fast = fast.next.next
        if slow == fast:
            return True
    return False
