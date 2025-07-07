from typing import Optional

from utils.tree import TreeNode


def is_balanced(root: Optional[TreeNode]) -> bool:
    return check_balance(root) is not None


def check_balance(root: Optional[TreeNode]) -> Optional[int]:
    if not root:
        return 0
    left = check_balance(root.left)
    if left is None:
        return None
    right = check_balance(root.right)
    if right is None or abs(left - right) > 1:
        return None
    return 1 + max(left, right)
