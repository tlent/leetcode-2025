use std::{cell::RefCell, rc::Rc};

use crate::utils::tree::TreeNode;

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

pub fn invert_binary_tree_recursive(
    root: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(node) = root.as_ref() {
        let left = node.borrow_mut().left.take();
        let right = node.borrow_mut().right.take();
        node.borrow_mut().left = invert_binary_tree_recursive(right);
        node.borrow_mut().right = invert_binary_tree_recursive(left);
    }
    root
}
