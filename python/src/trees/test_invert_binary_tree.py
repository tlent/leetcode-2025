from invert_binary_tree import invert_tree, invert_tree_recursive
from tree import TreeNode


def test_invert_binary_tree_example_one() -> None:
    root = TreeNode.from_level_order([4, 2, 7, 1, 3, 6, 9])
    assert root is not None
    inverted = invert_tree(root)
    assert inverted is not None
    assert list(inverted.values()) == [4, 7, 2, 9, 6, 3, 1]


def test_invert_binary_tree_example_two() -> None:
    root = TreeNode.from_level_order([2, 1, 3])
    assert root is not None
    inverted = invert_tree(root)
    assert inverted is not None
    assert list(inverted.values()) == [2, 3, 1]


def test_invert_binary_tree_example_three() -> None:
    root = TreeNode.from_level_order([])
    assert root is None
    inverted = invert_tree(root)
    assert inverted is None


def test_invert_binary_tree_recursive_example_one() -> None:
    root = TreeNode.from_level_order([4, 2, 7, 1, 3, 6, 9])
    assert root is not None
    inverted = invert_tree_recursive(root)
    assert inverted is not None
    assert list(inverted.values()) == [4, 7, 2, 9, 6, 3, 1]


def test_invert_binary_tree_recursive_example_two() -> None:
    root = TreeNode.from_level_order([2, 1, 3])
    assert root is not None
    inverted = invert_tree_recursive(root)
    assert inverted is not None
    assert list(inverted.values()) == [2, 3, 1]


def test_invert_binary_tree_recursive_example_three() -> None:
    root = TreeNode.from_level_order([])
    assert root is None
    inverted = invert_tree_recursive(root)
    assert inverted is None
