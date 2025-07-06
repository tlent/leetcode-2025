use crate::{n0235_lowest_common_ancestor_of_a_binary_search_tree::*, utils::tree::Tree};

#[test]
fn test_lowest_common_ancestor_example_one() {
    let tree = Tree::new([
        Some(6),
        Some(2),
        Some(8),
        Some(0),
        Some(4),
        Some(7),
        Some(9),
        None,
        None,
        Some(3),
        Some(5),
    ]);
    let p = tree.nodes().find(|n| n.borrow().value == 2);
    let q = tree.nodes().find(|n| n.borrow().value == 8);

    let expected = tree.nodes().find(|n| n.borrow().value == 6);
    assert_eq!(lowest_common_ancestor(tree.root, p, q), expected);
}

#[test]
fn test_lowest_common_ancestor_example_two() {
    let tree = Tree::new([
        Some(6),
        Some(2),
        Some(8),
        Some(0),
        Some(4),
        Some(7),
        Some(9),
        None,
        None,
        Some(3),
        Some(5),
    ]);
    let p = tree.nodes().find(|n| n.borrow().value == 2);
    let q = tree.nodes().find(|n| n.borrow().value == 4);

    let expected = tree.nodes().find(|n| n.borrow().value == 2);
    assert_eq!(lowest_common_ancestor(tree.root, p, q), expected);
}

#[test]
fn test_lowest_common_ancestor_example_three() {
    let tree = Tree::new([Some(2), Some(1)]);
    let p = tree.nodes().find(|n| n.borrow().value == 2);
    let q = tree.nodes().find(|n| n.borrow().value == 1);

    let expected = tree.nodes().find(|n| n.borrow().value == 2);
    assert_eq!(lowest_common_ancestor(tree.root, p, q), expected);
}
