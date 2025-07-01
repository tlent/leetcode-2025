use crate::n0141_linked_list_cycle::*;
use crate::utils::linked_list::SharedList;

#[test]
fn test_example_one() {
    let list = SharedList::new([3, 2, 0, -4]);
    let nodes = list.nodes().collect::<Vec<_>>();
    nodes[nodes.len() - 1].borrow_mut().next = Some(nodes[1].clone());
    assert!(has_cycle(list.head));
}

#[test]
fn test_example_two() {
    let list = SharedList::new([1, 2]);
    let nodes = list.nodes().collect::<Vec<_>>();
    nodes[nodes.len() - 1].borrow_mut().next = Some(nodes[0].clone());
    assert!(has_cycle(list.head));
}

#[test]
fn test_example_three() {
    let list = SharedList::new([1]);
    assert!(!has_cycle(list.head));
}
