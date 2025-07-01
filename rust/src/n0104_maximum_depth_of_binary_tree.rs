use std::{cell::RefCell, rc::Rc};

use crate::utils::tree::TreeNode;

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> usize {
    match root {
        Some(node) => {
            let left = node.borrow().left.clone();
            let right = node.borrow().right.clone();
            max_depth(left).max(max_depth(right)) + 1
        }
        None => 0,
    }
}

pub fn max_depth_iterative(root: Option<Rc<RefCell<TreeNode>>>) -> usize {
    let mut max_depth = 0;
    if let Some(r) = root {
        let mut stack = vec![(r, 1)];
        while let Some((node, depth)) = stack.pop() {
            max_depth = max_depth.max(depth);
            if let Some(l) = node.borrow().left.clone() {
                stack.push((l, depth + 1));
            }
            if let Some(r) = node.borrow().right.clone() {
                stack.push((r, depth + 1));
            }
        }
    }
    max_depth
}
