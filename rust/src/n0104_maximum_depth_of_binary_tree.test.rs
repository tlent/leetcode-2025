use crate::n0104_maximum_depth_of_binary_tree::*;
use crate::utils::tree::Tree;

#[test]
fn test_max_depth_example_one() {
    let tree = Tree::new([Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
    assert_eq!(max_depth(tree.root), 3);
}

#[test]
fn test_max_depth_example_two() {
    let tree = Tree::new([Some(1), None, Some(2)]);
    assert_eq!(max_depth(tree.root), 2);
}

#[test]
fn test_max_depth_iterative_example_one() {
    let tree = Tree::new([Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
    assert_eq!(max_depth_iterative(tree.root), 3);
}

#[test]
fn test_max_depth_iterative_example_two() {
    let tree = Tree::new([Some(1), None, Some(2)]);
    assert_eq!(max_depth_iterative(tree.root), 2);
}
