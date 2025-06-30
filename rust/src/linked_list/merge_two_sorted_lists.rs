use crate::linked_list::list::{BoxList, BoxListNode};

pub fn merge(mut a: BoxList, mut b: BoxList) -> BoxList {
    let mut head = BoxListNode::default();
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
    use super::*;

    #[test]
    fn test_merge() {
        let a = BoxListNode::from_values([1, 2, 4]);
        let b = BoxListNode::from_values([1, 3, 4]);
        let result = merge(a, b);
        assert_eq!(
            result.unwrap().values().collect::<Vec<_>>(),
            vec![1, 1, 2, 3, 4, 4]
        );
    }
}
