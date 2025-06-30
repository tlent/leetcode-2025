use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub struct ListNode {
    pub value: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(value: i32) -> Self {
        ListNode { next: None, value }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct List(Option<Box<ListNode>>);

impl List {
    pub fn new(values: &[i32]) -> Self {
        let mut head = ListNode::default();
        let mut cursor = &mut head;
        for &value in values {
            cursor.next = Some(Box::new(ListNode::new(value)));
            cursor = cursor.next.as_mut().unwrap();
        }
        List(head.next)
    }

    pub fn nodes(&self) -> impl Iterator<Item = &ListNode> {
        let mut current = self.0.as_deref();
        std::iter::from_fn(move || {
            current.take().inspect(|node| {
                current = node.next.as_deref();
            })
        })
    }

    pub fn values(&self) -> impl Iterator<Item = i32> {
        self.nodes().map(|node| node.value)
    }

    pub fn to_vec(&self) -> Vec<i32> {
        self.values().collect()
    }

    pub fn head(&self) -> Option<Box<ListNode>> {
        self.0.clone()
    }

    pub fn head_mut(&mut self) -> &mut Option<Box<ListNode>> {
        &mut self.0
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub struct SharedListNode {
    pub value: i32,
    pub next: Option<Rc<RefCell<SharedListNode>>>,
}

impl SharedListNode {
    pub fn new(value: i32) -> Self {
        SharedListNode { value, next: None }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SharedList(Option<Rc<RefCell<SharedListNode>>>);

impl SharedList {
    pub fn new(values: &[i32]) -> Self {
        let nodes: Vec<Rc<RefCell<SharedListNode>>> = values
            .iter()
            .map(|&val| Rc::new(RefCell::new(SharedListNode::new(val))))
            .collect();
        if nodes.is_empty() {
            return SharedList(None);
        }

        // Link the nodes
        for i in 0..nodes.len() - 1 {
            nodes[i].borrow_mut().next = Some(nodes[i + 1].clone());
        }

        SharedList(Some(nodes[0].clone()))
    }

    pub fn nodes(&self) -> impl Iterator<Item = Rc<RefCell<SharedListNode>>> {
        let mut current = self.0.as_ref().map(Rc::clone);
        std::iter::from_fn(move || {
            current.take().inspect(|node| {
                current = node.borrow().next.clone();
            })
        })
    }

    pub fn values(&self) -> impl Iterator<Item = i32> {
        self.nodes().map(|node| node.borrow().value)
    }

    pub fn to_vec(&self) -> Vec<i32> {
        self.values().collect()
    }

    pub fn head(&self) -> Option<Rc<RefCell<SharedListNode>>> {
        self.0.clone()
    }

    pub fn head_mut(&mut self) -> &mut Option<Rc<RefCell<SharedListNode>>> {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_new() {
        let list = List::new(&[1, 2, 3]);
        assert_eq!(list.to_vec(), vec![1, 2, 3]);
    }

    #[test]
    fn test_list_empty() {
        let list = List::new(&[]);
        assert_eq!(list.to_vec(), Vec::<i32>::new());
    }

    #[test]
    fn test_list_nodes_iterator() {
        let list = List::new(&[1, 2, 3]);
        let node_count = list.nodes().count();
        assert_eq!(node_count, 3);
    }

    #[test]
    fn test_shared_list_new() {
        let list = SharedList::new(&[1, 2, 3]);
        assert_eq!(list.to_vec(), vec![1, 2, 3]);
    }

    #[test]
    fn test_shared_list_empty() {
        let list = SharedList::new(&[]);
        assert_eq!(list.to_vec(), Vec::<i32>::new());
    }

    #[test]
    fn test_shared_list_nodes_iterator() {
        let list = SharedList::new(&[1, 2, 3]);
        let node_count = list.nodes().count();
        assert_eq!(node_count, 3);
    }
}
