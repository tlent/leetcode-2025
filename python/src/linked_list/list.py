from __future__ import annotations
from typing import Iterable, Iterator, Optional


class ListNode:
    def __init__(self, val: int = 0, next: List = None):
        self.val = val
        self.next = next

    @classmethod
    def from_iterable(cls, iterable: Iterable[int]) -> List:
        head = None
        cursor = None
        for val in iterable:
            node = cls(val)
            if cursor is None:
                head = node
            else:
                cursor.next = node
            cursor = node
        return head

    def __iter__(self) -> Iterator[int]:
        cursor: List = self
        while cursor:
            yield cursor.val
            cursor = cursor.next


List = Optional[ListNode]
