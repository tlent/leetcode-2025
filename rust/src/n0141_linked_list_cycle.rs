use std::{cell::RefCell, rc::Rc};

use crate::utils::linked_list::SharedListNode;

pub fn has_cycle(head: Option<Rc<RefCell<SharedListNode>>>) -> bool {
    let mut slow = head.as_ref().map(Rc::clone);
    let mut fast = head.as_ref().map(Rc::clone);

    loop {
        // Check if we can advance both pointers
        let (slow_next, fast_next) = match (&slow, &fast) {
            (Some(s), Some(f)) => {
                let slow_next = s.borrow().next.clone();
                let fast_first = f.borrow().next.clone();
                let fast_next = match &fast_first {
                    Some(f1) => f1.borrow().next.clone(),
                    None => return false,
                };
                (slow_next, fast_next)
            }
            _ => return false,
        };

        slow = slow_next;
        fast = fast_next;

        // Check for cycle
        match (&slow, &fast) {
            (Some(s), Some(f)) if Rc::ptr_eq(s, f) => return true,
            (None, _) | (_, None) => return false,
            _ => continue,
        }
    }
}
