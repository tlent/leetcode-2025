from n0235_lowest_common_ancestor_of_a_binary_search_tree import lowest_common_ancestor
from utils.tree import Tree


def test_lowest_common_ancestor_example_one() -> None:
    tree = Tree([6, 2, 8, 0, 4, 7, 9, None, None, 3, 5])

    p = next(filter(lambda node: node.value == 2, tree.nodes()))
    q = next(filter(lambda node: node.value == 8, tree.nodes()))
    expected = next(filter(lambda node: node.value == 6, tree.nodes()))

    assert lowest_common_ancestor(tree.root, p, q) == expected


def test_lowest_common_ancestor_example_two() -> None:
    tree = Tree([6, 2, 8, 0, 4, 7, 9, None, None, 3, 5])

    p = next(filter(lambda node: node.value == 2, tree.nodes()))
    q = next(filter(lambda node: node.value == 4, tree.nodes()))
    expected = next(filter(lambda node: node.value == 2, tree.nodes()))

    assert lowest_common_ancestor(tree.root, p, q) == expected


def test_lowest_common_ancestor_example_three() -> None:
    tree = Tree([2, 1])

    p = next(filter(lambda node: node.value == 2, tree.nodes()))
    q = next(filter(lambda node: node.value == 1, tree.nodes()))
    expected = next(filter(lambda node: node.value == 2, tree.nodes()))

    assert lowest_common_ancestor(tree.root, p, q) == expected
