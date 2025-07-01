use crate::n0206_reverse_linked_list::*;
use crate::utils::linked_list::*;

#[test]
fn test_reverse_list() {
    let mut list = List::new([1, 2, 3, 4, 5]);
    list.head = reverse_list(list.head);
    assert_eq!(list.to_vec(), [5, 4, 3, 2, 1]);
}

#[test]
fn test_reverse_list_recursive() {
    let mut list = List::new([1, 2, 3, 4, 5]);
    list.head = reverse_list_recursive(list.head);
    assert_eq!(list.to_vec(), [5, 4, 3, 2, 1]);
}
