from __future__ import annotations
from typing import Iterable, Iterator, Optional


class ListNode:
    def __init__(self, value: int = 0, next: List = None):
        self.value = value
        self.next = next

    @classmethod
    def from_values(cls, values: Iterable[int]) -> List:
        head = None
        cursor = None
        for value in values:
            node = cls(value)
            if cursor is None:
                head = node
            else:
                cursor.next = node
            cursor = node
        return head

    def nodes(self) -> Iterator[ListNode]:
        cursor: List = self
        while cursor:
            yield cursor
            cursor = cursor.next

    def values(self) -> Iterator[int]:
        return map(lambda node: node.value, self.nodes())


List = Optional[ListNode]
