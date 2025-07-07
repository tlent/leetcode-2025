from n0232_implement_queue_using_stacks import MyQueue


def test_myqueue() -> None:
    queue = MyQueue()
    queue.push(1)
    queue.push(2)
    assert queue.peek() == 1
    assert queue.pop() == 1
    assert not queue.empty()
