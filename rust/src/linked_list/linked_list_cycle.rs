use std::{cell::RefCell, rc::Rc};

use crate::linked_list::list::SharedListNode;

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

#[cfg(test)]
mod tests {
    use crate::linked_list::list::SharedList;

    use super::*;

    #[test]
    fn test_example_one() {
        let list = SharedList::new([3, 2, 0, -4]);
        let nodes = list.nodes().collect::<Vec<_>>();
        nodes[nodes.len() - 1].borrow_mut().next = Some(nodes[1].clone());
        assert!(has_cycle(list.head));
    }

    #[test]
    fn test_example_two() {
        let list = SharedList::new([1, 2]);
        let nodes = list.nodes().collect::<Vec<_>>();
        nodes[nodes.len() - 1].borrow_mut().next = Some(nodes[0].clone());
        assert!(has_cycle(list.head));
    }

    #[test]
    fn test_example_three() {
        let list = SharedList::new([1]);
        assert!(!has_cycle(list.head));
    }
}
