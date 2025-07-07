use std::{cell::RefCell, rc::Rc};

use crate::utils::tree::TreeNode;

pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    check_balance(root).is_some()
}

fn check_balance(root: Option<Rc<RefCell<TreeNode>>>) -> Option<u32> {
    if let Some(node) = root {
        let left = check_balance(node.borrow().left.clone())?;
        let right = check_balance(node.borrow().right.clone())?;
        if left.abs_diff(right) > 1 {
            return None;
        }
        Some(1 + left.max(right))
    } else {
        Some(0)
    }
}
