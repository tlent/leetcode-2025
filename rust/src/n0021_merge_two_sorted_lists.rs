use crate::utils::linked_list::ListNode;

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
