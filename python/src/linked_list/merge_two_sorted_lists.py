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


