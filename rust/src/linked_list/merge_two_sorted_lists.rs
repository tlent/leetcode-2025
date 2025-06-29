use crate::linked_list::list::{List, ListNode};

pub fn merge(mut a: List, mut b: List) -> List {
    let mut head = ListNode::default();
    let mut cursor = &mut head;

    while a.is_some() && b.is_some() {
        if a.as_ref().unwrap().val <= b.as_ref().unwrap().val {
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
mod test {
    use super::*;

    #[test]
    fn test_merge() {
        let a = ListNode::from_iter([1, 2, 4]);
        let b = ListNode::from_iter([1, 3, 4]);
        let result = merge(Some(Box::new(a)), Some(Box::new(b)));
        assert_eq!(
            result.unwrap().iter().collect::<Vec<_>>(),
            vec![1, 1, 2, 3, 4, 4]
        );
    }
}
