from __future__ import annotations
from typing import Iterable, Iterator, Optional
from collections import deque


class TreeNode:
    def __init__(
        self,
        value: int = 0,
        left: Tree = None,
        right: Tree = None,
    ):
        self.value = value
        self.left = left
        self.right = right

    @classmethod
    def from_array(cls, values: Iterable[int]) -> Tree:
        nodes = [cls(value) for value in values]
        if len(nodes) == 0:
            return None
        for i, node in enumerate(nodes):
            left_index = 2 * i + 1
            if left_index < len(nodes):
                node.left = nodes[left_index]
            right_index = left_index + 1
            if right_index < len(nodes):
                node.right = nodes[right_index]

        return nodes[0]

    def nodes(self) -> Iterator[TreeNode]:
        queue: deque[TreeNode] = deque([self])
        while queue:
            node = queue.popleft()
            yield node
            if node.left:
                queue.append(node.left)
            if node.right:
                queue.append(node.right)

    def values(self) -> Iterator[int]:
        return map(lambda node: node.value, self.nodes())


Tree = Optional[TreeNode]
