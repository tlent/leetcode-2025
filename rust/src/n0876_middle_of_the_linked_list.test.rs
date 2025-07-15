use crate::n0876_middle_of_the_linked_list::middle_node;
use crate::utils::linked_list::List;

#[test]
fn test_middle_node() {
    let list = List::new([1, 2, 3, 4, 5]);
    let result = middle_node(list.head);
    assert_eq!(result.unwrap().value, 3);

    let list = List::new([1, 2, 3, 4, 5, 6]);
    let result = middle_node(list.head);
    assert_eq!(result.unwrap().value, 4);
}
