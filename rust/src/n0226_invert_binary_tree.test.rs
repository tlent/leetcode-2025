use crate::n0226_invert_binary_tree::*;
use crate::utils::tree::Tree;

#[test]
fn test_invert_binary_tree_example_one() {
    let mut tree = Tree::new([
        Some(4),
        Some(2),
        Some(7),
        Some(1),
        Some(3),
        Some(6),
        Some(9),
    ]);
    tree.root = invert_binary_tree(tree.root);
    assert_eq!(tree.to_vec(), [4, 7, 2, 9, 6, 3, 1]);
}

#[test]
fn test_invert_binary_tree_example_two() {
    let mut tree = Tree::new([Some(2), Some(1), Some(3)]);
    tree.root = invert_binary_tree(tree.root);
    assert_eq!(tree.to_vec(), [2, 3, 1]);
}

#[test]
fn test_invert_binary_tree_example_three() {
    let mut tree = Tree::new([]);
    tree.root = invert_binary_tree(tree.root);
    assert_eq!(tree.to_vec(), []);
}

#[test]
fn test_invert_binary_tree_recursive_example_one() {
    let mut tree = Tree::new([
        Some(4),
        Some(2),
        Some(7),
        Some(1),
        Some(3),
        Some(6),
        Some(9),
    ]);
    tree.root = invert_binary_tree_recursive(tree.root);
    assert_eq!(tree.to_vec(), [4, 7, 2, 9, 6, 3, 1]);
}

#[test]
fn test_invert_binary_tree_recursive_example_two() {
    let mut tree = Tree::new([Some(2), Some(1), Some(3)]);
    tree.root = invert_binary_tree_recursive(tree.root);
    assert_eq!(tree.to_vec(), [2, 3, 1]);
}

#[test]
fn test_invert_binary_tree_recursive_example_three() {
    let mut tree = Tree::new([]);
    tree.root = invert_binary_tree_recursive(tree.root);
    assert_eq!(tree.to_vec(), []);
}
