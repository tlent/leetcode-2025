use crate::utils::linked_list::ListNode;

pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut slow = head.clone();
    let mut fast = head;

    while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
        slow = slow.unwrap().next;
        fast = fast.unwrap().next.unwrap().next;
    }

    slow
}
