use crate::linked_list::list::ListNode;

pub fn merge(mut a: Option<Box<ListNode>>, mut b: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = ListNode::default();
    let mut cursor = &mut head;

    while a.is_some() && b.is_some() {
        if a.as_ref().unwrap().value <= b.as_ref().unwrap().value {
            cursor.next = a.take();
            cursor = cursor.next.as_mut().unwrap();
            a = cursor.next.take();
        } else {
            cursor.next = b.take();
            cursor = cursor.next.as_mut().unwrap();
            b = cursor.next.take();
        }
    }

    cursor.next = a.or(b);
    head.next
}

#[cfg(test)]
mod tests {
    use crate::linked_list::list::List;

    use super::*;

    #[test]
    fn test_merge() {
        let mut a = List::new(&[1, 2, 4]);
        let b = List::new(&[1, 3, 4]);
        a.0 = merge(a.0, b.0);
        assert_eq!(a.to_vec(), vec![1, 1, 2, 3, 4, 4]);
    }
}
