use crate::n0232_implement_queue_using_stacks::MyQueue;

#[test]
fn test_queue() {
    let mut queue = MyQueue::new();
    queue.push(1);
    queue.push(2);
    assert_eq!(queue.peek(), 1);
    assert_eq!(queue.pop(), 1);
    assert!(!queue.empty())
}
