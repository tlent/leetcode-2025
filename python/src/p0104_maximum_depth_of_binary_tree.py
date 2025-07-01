from typing import Optional
from utils.tree import TreeNode


def max_depth(root: Optional[TreeNode]) -> int:
    if not root:
        return 0
    return max(max_depth(root.left), max_depth(root.right)) + 1
