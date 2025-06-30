from __future__ import annotations
from typing import Iterator, Optional


class ListNode:
    def __init__(self, value: int = 0, next: Optional[ListNode] = None):
        self.value = value
        self.next = next


class List:
    def __init__(self, values: list[int]):
        head = None
        cursor = None
        for value in values:
            node = ListNode(value)
            if cursor is None:
                head = node
            else:
                cursor.next = node
            cursor = node
        self.head = head

    def nodes(self) -> Iterator[ListNode]:
        cursor = self.head
        while cursor:
            yield cursor
            cursor = cursor.next

    def values(self) -> Iterator[int]:
        return map(lambda node: node.value, self.nodes())

    def to_list(self) -> list[int]:
        return list(self.values())
