use std::{cell::RefCell, rc::Rc};

use crate::utils::tree::TreeNode;

pub fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    let p_value = p.as_ref()?.borrow().value;
    let q_value = q.as_ref()?.borrow().value;
    let min = p_value.min(q_value);
    let max = p_value.max(q_value);
    let mut cursor = root?;
    loop {
        let cursor_value = cursor.borrow().value;
        cursor = if max < cursor_value {
            cursor.borrow().left.clone()?
        } else if min > cursor_value {
            cursor.borrow().right.clone()?
        } else {
            return Some(cursor);
        }
    }
}
