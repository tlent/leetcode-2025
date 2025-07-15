from n0543_diameter_of_binary_tree import diameter_of_binary_tree
from utils.tree import Tree


def test_diameter_of_binary_tree() -> None:
    tree = Tree([1, 2, 3, 4, 5])
    assert diameter_of_binary_tree(tree.root) == 3
    tree = Tree([1, 2])
    assert diameter_of_binary_tree(tree.root) == 1
