use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::utils::tree::TreeNode;

pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    if root.is_none() {
        return vec![];
    }
    let mut result = vec![];
    let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::from([root.unwrap()]);
    while !queue.is_empty() {
        let len = queue.len();
        let mut level = Vec::with_capacity(len);
        for _ in 0..len {
            let node = queue.pop_front().unwrap();
            level.push(node.borrow().value);
            if let Some(l) = node.borrow().left.clone() {
                queue.push_back(l);
            }
            if let Some(r) = node.borrow().right.clone() {
                queue.push_back(r);
            }
        }
        result.push(level);
    }
    result
}
