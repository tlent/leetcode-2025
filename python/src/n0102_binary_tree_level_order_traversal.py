from collections import deque
from typing import Optional

from utils.tree import TreeNode


def level_order(root: Optional[TreeNode]) -> list[list[int]]:
    if not root:
        return []
    result: list[list[int]] = []
    queue = deque([root])
    while queue:
        length = len(queue)
        level: list[int] = []
        for _ in range(length):
            node = queue.popleft()
            level.append(node.value)
            if node.left:
                queue.append(node.left)
            if node.right:
                queue.append(node.right)
        result.append(level)
    return result
