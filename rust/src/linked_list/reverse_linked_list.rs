#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32, next: Option<Box<ListNode>>) -> Self {
        ListNode { next, val }
    }
}

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
mod test {
    use super::*;

    #[test]
    fn test_reverse_list() {
        let list = Some(Box::new(ListNode::new(
            1,
            Some(Box::new(ListNode::new(
                2,
                Some(Box::new(ListNode::new(
                    3,
                    Some(Box::new(ListNode::new(
                        4,
                        Some(Box::new(ListNode::new(5, None))),
                    ))),
                ))),
            ))),
        )));

        let expected = Some(Box::new(ListNode::new(
            5,
            Some(Box::new(ListNode::new(
                4,
                Some(Box::new(ListNode::new(
                    3,
                    Some(Box::new(ListNode::new(
                        2,
                        Some(Box::new(ListNode::new(1, None))),
                    ))),
                ))),
            ))),
        )));
        let mut expected_cursor = expected.as_ref();
        let actual = reverse_list(list);
        let mut actual_cursor = actual.as_ref();

        while let (Some(e), Some(a)) = (expected_cursor, actual_cursor) {
            assert_eq!(e.val, a.val);
            expected_cursor = e.next.as_ref();
            actual_cursor = a.next.as_ref();
        }

        assert!(expected_cursor.is_none());
        assert!(actual_cursor.is_none());
    }

    #[test]
    fn test_reverse_list_recursive() {
        let list = Some(Box::new(ListNode::new(
            1,
            Some(Box::new(ListNode::new(
                2,
                Some(Box::new(ListNode::new(
                    3,
                    Some(Box::new(ListNode::new(
                        4,
                        Some(Box::new(ListNode::new(5, None))),
                    ))),
                ))),
            ))),
        )));

        let expected = Some(Box::new(ListNode::new(
            5,
            Some(Box::new(ListNode::new(
                4,
                Some(Box::new(ListNode::new(
                    3,
                    Some(Box::new(ListNode::new(
                        2,
                        Some(Box::new(ListNode::new(1, None))),
                    ))),
                ))),
            ))),
        )));
        let mut expected_cursor = expected.as_ref();
        let actual = reverse_list_recursive(list);
        let mut actual_cursor = actual.as_ref();

        while let (Some(e), Some(a)) = (expected_cursor, actual_cursor) {
            assert_eq!(e.val, a.val);
            expected_cursor = e.next.as_ref();
            actual_cursor = a.next.as_ref();
        }

        assert!(expected_cursor.is_none());
        assert!(actual_cursor.is_none());
    }
}
