from __future__ import annotations
from typing import Iterator, Optional, Iterable
from collections import deque


class TreeNode:
    def __init__(
        self,
        value: int = 0,
        left: Optional[TreeNode] = None,
        right: Optional[TreeNode] = None,
    ):
        self.value = value
        self.left = left
        self.right = right


class Tree:
    def __init__(self, values: Iterable[int]):
        nodes = [TreeNode(value) for value in values]
        if len(nodes) == 0:
            self.root = None
        else:
            for i, node in enumerate(nodes):
                left_index = 2 * i + 1
                if left_index < len(nodes):
                    node.left = nodes[left_index]
                right_index = left_index + 1
                if right_index < len(nodes):
                    node.right = nodes[right_index]
            self.root = nodes[0]

    def nodes(self) -> Iterator[TreeNode]:
        if not self.root:
            return
        queue: deque[TreeNode] = deque([self.root])
        while queue:
            node = queue.popleft()
            yield node
            if node.left:
                queue.append(node.left)
            if node.right:
                queue.append(node.right)

    def values(self) -> Iterator[int]:
        return map(lambda node: node.value, self.nodes())

    def to_list(self) -> list[int]:
        return list(self.values())
