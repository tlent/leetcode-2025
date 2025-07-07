use crate::{n0110_balanced_binary_tree::is_balanced, utils::tree::Tree};

#[test]
fn test_is_balanced_example_one() {
    let tree = Tree::new([Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
    assert!(is_balanced(tree.root));
}

#[test]
fn test_is_balanced_example_two() {
    let tree = Tree::new([
        Some(1),
        Some(2),
        Some(2),
        Some(3),
        Some(3),
        None,
        None,
        Some(4),
        Some(4),
    ]);
    assert!(!is_balanced(tree.root));
}

#[test]
fn test_is_balanced_example_three() {
    let tree = Tree::new([]);
    assert!(is_balanced(tree.root));
}
