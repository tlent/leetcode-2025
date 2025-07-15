use crate::{n0543_diameter_of_binary_tree::diameter_of_binary_tree, utils::tree::Tree};

#[test]
fn test_diameter_of_binary_tree() {
    let tree = Tree::new([Some(1), Some(2), Some(3), Some(4), Some(5)]);
    assert_eq!(diameter_of_binary_tree(tree.root), 3);
    let tree = Tree::new([Some(1), Some(2)]);
    assert_eq!(diameter_of_binary_tree(tree.root), 1);
}
