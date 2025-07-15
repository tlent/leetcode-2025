use std::{cell::RefCell, rc::Rc};

use crate::utils::tree::TreeNode;

pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut max_diameter = 0;

    fn depth(root: Option<Rc<RefCell<TreeNode>>>, max: &mut i32) -> i32 {
        if let Some(node) = root {
            let left = depth(node.borrow().left.clone(), max);
            let right = depth(node.borrow().right.clone(), max);

            *max = (*max).max(left + right);

            1 + left.max(right)
        } else {
            0
        }
    }
    depth(root, &mut max_diameter);

    max_diameter
}
