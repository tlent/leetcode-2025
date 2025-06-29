use std::rc::Rc;

use crate::linked_list::list::RcList;

pub fn has_cycle(head: RcList) -> bool {
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

#[cfg(test)]
mod test {
    use crate::linked_list::list::{RcListNode, test_utils};

    use super::*;

    #[test]
    fn test_example_one() {
        let head = RcListNode::from_values([3, 2, 0, -4]).unwrap();
        test_utils::create_cycle(&head, 1);
        assert!(has_cycle(Some(head)));
    }

    #[test]
    fn test_example_two() {
        let head = RcListNode::from_values([1, 2]).unwrap();
        test_utils::create_cycle(&head, 0);
        assert!(has_cycle(Some(head)));
    }

    #[test]
    fn test_example_three() {
        let head = RcListNode::from_values([1]).unwrap();
        // No cycle
        assert!(!has_cycle(Some(head)));
    }
}
