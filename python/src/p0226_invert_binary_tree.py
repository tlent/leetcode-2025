from typing import Optional
from utils.tree import TreeNode


def invert_tree(root: Optional[TreeNode]) -> Optional[TreeNode]:
    if not root:
        return None
    stack = [root]
    while stack:
        node = stack.pop()
        node.left, node.right = node.right, node.left
        if node.left:
            stack.append(node.left)
        if node.right:
            stack.append(node.right)
    return root


def invert_tree_recursive(root: Optional[TreeNode]) -> Optional[TreeNode]:
    if not root:
        return None
    root.left, root.right = (
        invert_tree_recursive(root.right),
        invert_tree_recursive(root.left),
    )
    return root
