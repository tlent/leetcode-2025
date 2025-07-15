from typing import Optional, cast

from utils.linked_list import ListNode


def middle_node(head: Optional[ListNode]) -> Optional[ListNode]:
    fast = head
    slow = head

    while fast and fast.next:
        fast = fast.next.next
        slow = cast(ListNode, slow).next

    return slow
