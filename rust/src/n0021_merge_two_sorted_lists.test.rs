use crate::n0021_merge_two_sorted_lists::*;
use crate::utils::linked_list::*;

#[test]
fn test_merge() {
    let mut a = List::new([1, 2, 4]);
    let b = List::new([1, 3, 4]);
    a.head = merge(a.head, b.head);
    assert_eq!(a.to_vec(), vec![1, 1, 2, 3, 4, 4]);
}
