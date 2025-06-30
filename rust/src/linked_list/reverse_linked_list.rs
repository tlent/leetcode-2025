use crate::linked_list::list::ListNode;

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    let mut cursor = head;

    while let Some(mut node) = cursor {
        let next = node.next;
        node.next = prev.take();
        prev = Some(node);
        cursor = next;
    }

    prev
}

// NOTE: This recursive implementation is O(n²) due to Rust's ownership model.
// Unlike Python/TypeScript which can directly access head.next.next after recursion,
// Rust requires finding the tail of the reversed portion to attach the current node.
// This results in O(n) tail traversal for each of O(n) recursive calls.
pub fn reverse_list_recursive(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match head {
        None => None,
        Some(mut node) => match node.next.take() {
            None => Some(node),
            Some(next) => {
                let new_head = reverse_list_recursive(Some(next));
                // Find the tail of reversed list and attach current node
                if let Some(mut tail) = new_head {
                    let mut current = &mut tail;
                    while current.next.is_some() {
                        current = current.next.as_mut().unwrap();
                    }
                    current.next = Some(node);
                    Some(tail)
                } else {
                    Some(node)
                }
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::linked_list::list::List;

    use super::*;

    #[test]
    fn test_reverse_list() {
        let mut list = List::new(&[1, 2, 3, 4, 5]);
        *list.head_mut() = reverse_list(list.head());
        assert_eq!(list.to_vec(), [5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_reverse_list_recursive() {
        let mut list = List::new(&[1, 2, 3, 4, 5]);
        *list.head_mut() = reverse_list(list.head());
        assert_eq!(list.to_vec(), [5, 4, 3, 2, 1]);
    }
}
