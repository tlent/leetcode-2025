from typing import Optional

from utils.tree import TreeNode


def diameter_of_binary_tree(root: Optional[TreeNode]) -> int:
    max_diameter: int = 0

    def depth(root: Optional[TreeNode]) -> int:
        nonlocal max_diameter
        if not root:
            return 0
        left = depth(root.left)
        right = depth(root.right)

        max_diameter = max(max_diameter, left + right)

        return 1 + max(left, right)

    depth(root)

    return max_diameter
