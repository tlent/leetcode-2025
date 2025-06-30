use std::{cell::RefCell, rc::Rc};

use crate::trees::tree::TreeNode;

pub fn invert_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if root.is_none() {
        return root;
    }
    let mut stack = vec![root.clone().unwrap()];
    while let Some(node) = stack.pop() {
        let left = node.borrow_mut().left.take();
        let right = node.borrow_mut().right.take();
        node.borrow_mut().left = right;
        node.borrow_mut().right = left;

        if let Some(left) = &node.borrow().left {
            stack.push(left.clone());
        }
        if let Some(right) = &node.borrow().right {
            stack.push(right.clone());
        }
    }
    root
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::trees::tree::Tree;

    #[test]
    fn test_invert_binary_tree_example_one() {
        let mut tree = Tree::new(&[4, 2, 7, 1, 3, 6, 9]);
        tree.0 = invert_binary_tree(tree.0);
        assert_eq!(tree.to_vec(), [4, 7, 2, 9, 6, 3, 1]);
    }

    #[test]
    fn test_invert_binary_tree_example_two() {
        let mut tree = Tree::new(&[2, 1, 3]);
        tree.0 = invert_binary_tree(tree.0);
        assert_eq!(tree.to_vec(), [2, 3, 1]);
    }

    #[test]
    fn test_invert_binary_tree_example_three() {
        let mut tree = Tree::new(&[]);
        tree.0 = invert_binary_tree(tree.0);
        assert_eq!(tree.to_vec(), []);
    }
}
