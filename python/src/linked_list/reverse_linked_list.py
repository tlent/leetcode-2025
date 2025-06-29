from list import List


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
