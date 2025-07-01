from n0226_invert_binary_tree import invert_tree, invert_tree_recursive
from utils.tree import Tree


def test_invert_binary_tree_example_one() -> None:
    tree = Tree([4, 2, 7, 1, 3, 6, 9])
    tree.root = invert_tree(tree.root)
    assert tree.to_list() == [4, 7, 2, 9, 6, 3, 1]


def test_invert_binary_tree_example_two() -> None:
    tree = Tree([2, 1, 3])
    tree.root = invert_tree(tree.root)
    assert tree.to_list() == [2, 3, 1]


def test_invert_binary_tree_example_three() -> None:
    tree = Tree([])
    assert tree.root is None
    tree.root = invert_tree(tree.root)
    assert tree.root is None


def test_invert_binary_tree_recursive_example_one() -> None:
    tree = Tree([4, 2, 7, 1, 3, 6, 9])
    tree.root = invert_tree_recursive(tree.root)
    assert tree.to_list() == [4, 7, 2, 9, 6, 3, 1]


def test_invert_binary_tree_recursive_example_two() -> None:
    tree = Tree([2, 1, 3])
    tree.root = invert_tree_recursive(tree.root)
    assert tree.to_list() == [2, 3, 1]


def test_invert_binary_tree_recursive_example_three() -> None:
    tree = Tree([])
    assert tree.root is None
    tree.root = invert_tree_recursive(tree.root)
    assert tree.root is None
