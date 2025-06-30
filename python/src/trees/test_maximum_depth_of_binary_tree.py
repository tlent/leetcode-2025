from tree import Tree
from maximum_depth_of_binary_tree import max_depth


def test_max_depth_example_one() -> None:
    tree = Tree([3, 9, 20, None, None, 15, 7])
    assert max_depth(tree.root) == 3


def test_max_depth_example_two() -> None:
    tree = Tree([1, None, 2])
    assert max_depth(tree.root) == 2
