from typing import Optional
from utils.tree import TreeNode


def lowest_common_ancestor(
    root: Optional[TreeNode], p: Optional[TreeNode], q: Optional[TreeNode]
) -> Optional[TreeNode]:
    if not root or not p or not q:
        return None
    min_value = min(p.value, q.value)
    max_value = max(p.value, q.value)
    cursor: Optional[TreeNode] = root
    while cursor:
        if max_value < cursor.value:
            cursor = cursor.left
        elif min_value > cursor.value:
            cursor = cursor.right
        else:
            break
    return cursor
