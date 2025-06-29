use crate::linked_list::list::BoxList;

pub fn reverse_list(head: BoxList) -> BoxList {
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
pub fn reverse_list_recursive(head: BoxList) -> BoxList {
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
    use crate::linked_list::list::BoxListNode;

    use super::*;

    #[test]
    fn test_reverse_list() {
        let list = BoxListNode::from_values(1..=5);
        let expected = BoxListNode::from_values((1..=5).rev());
        assert_eq!(reverse_list(list), expected);
    }

    #[test]
    fn test_reverse_list_recursive() {
        let list = BoxListNode::from_values(1..=5);
        let expected = BoxListNode::from_values((1..=5).rev());
        assert_eq!(reverse_list_recursive(list), expected);
    }
}
