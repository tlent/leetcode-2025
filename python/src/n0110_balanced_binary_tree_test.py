from n0110_balanced_binary_tree import is_balanced
from utils.tree import Tree


def test_is_balanced_example_one() -> None:
    tree = Tree([1, 2, 2, 3, None, None, 3, 4, None, None, 4])
    assert not is_balanced(tree.root)


def test_is_balanced_example_two() -> None:
    tree = Tree([1, 2, 2, 3, 3, None, None, 4, 4])
    assert not is_balanced(tree.root)


def test_is_balanced_example_three() -> None:
    tree = Tree([])
    assert is_balanced(tree.root)
