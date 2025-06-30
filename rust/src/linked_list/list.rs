use std::cell::RefCell;
use std::rc::Rc;

pub type BoxList = Option<Box<BoxListNode>>;
pub type RcList = Option<Rc<RefCell<RcListNode>>>;

#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub struct BoxListNode {
    pub value: i32,
    pub next: BoxList,
}

impl BoxListNode {
    #[inline]
    pub fn new(value: i32) -> Self {
        BoxListNode { next: None, value }
    }

    /// Create a linked list from an iterator
    pub fn from_values<T: IntoIterator<Item = i32>>(values: T) -> BoxList {
        let mut head = BoxListNode::default();
        let mut cursor = &mut head;
        for value in values {
            cursor.next = Some(Box::new(BoxListNode::new(value)));
            cursor = cursor.next.as_mut().unwrap();
        }
        head.next
    }

    pub fn nodes(&self) -> impl Iterator<Item = &Self> {
        std::iter::successors(Some(self), |node| node.next.as_deref())
    }

    pub fn values(&self) -> impl Iterator<Item = i32> {
        self.nodes().map(|node| node.value)
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub struct RcListNode {
    pub value: i32,
    pub next: RcList,
}

impl RcListNode {
    pub fn new(value: i32) -> Self {
        RcListNode { value, next: None }
    }

    /// Create a linked list from an iterator
    pub fn from_values<T: IntoIterator<Item = i32>>(iter: T) -> RcList {
        let nodes: Vec<Rc<RefCell<RcListNode>>> = iter
            .into_iter()
            .map(|val| Rc::new(RefCell::new(RcListNode::new(val))))
            .collect();
        if nodes.is_empty() {
            return None;
        }

        // Link the nodes
        for i in 0..nodes.len() - 1 {
            nodes[i].borrow_mut().next = Some(nodes[i + 1].clone());
        }

        Some(nodes[0].clone())
    }

    pub fn nodes(head: &RcList) -> impl Iterator<Item = Rc<RefCell<RcListNode>>> {
        let mut current = head.as_ref().map(Rc::clone);
        std::iter::from_fn(move || {
            current.take().inspect(|node| {
                current = node.borrow().next.clone();
            })
        })
    }

    pub fn values(head: &RcList) -> impl Iterator<Item = i32> {
        Self::nodes(head).map(|node| node.borrow().value)
    }
}

#[cfg(test)]
pub mod test_utils {
    use super::*;

    /// Create a cycle by connecting tail to the node at given position
    /// This is a test utility for creating cyclic linked lists
    pub fn create_cycle(head: &Rc<RefCell<RcListNode>>, pos: usize) {
        let mut nodes = Vec::new();
        let mut current = Some(head.clone());

        // Collect all nodes
        while let Some(node) = current {
            let next = node.borrow().next.clone();
            nodes.push(node);
            current = next;
        }

        if pos < nodes.len() && !nodes.is_empty() {
            // Connect last node to node at position
            nodes[nodes.len() - 1].borrow_mut().next = Some(nodes[pos].clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box_list_from_values() {
        let list = BoxListNode::from_values([1, 2, 3]);
        assert!(list.is_some());
        let values: Vec<i32> = list.unwrap().values().collect();
        assert_eq!(values, vec![1, 2, 3]);
    }

    #[test]
    fn test_box_list_empty() {
        let list = BoxListNode::from_values([]);
        assert!(list.is_none());
    }

    #[test]
    fn test_box_list_nodes_iterator() {
        let list = BoxListNode::from_values([1, 2, 3]).unwrap();
        let node_count = list.nodes().count();
        assert_eq!(node_count, 3);
    }

    #[test]
    fn test_rc_list_from_values() {
        let list = RcListNode::from_values([1, 2, 3]);
        assert!(list.is_some());
        let values: Vec<i32> = RcListNode::values(&list).collect();
        assert_eq!(values, vec![1, 2, 3]);
    }

    #[test]
    fn test_rc_list_empty() {
        let list = RcListNode::from_values([]);
        assert!(list.is_none());
    }

    #[test]
    fn test_rc_list_nodes_iterator() {
        let list = RcListNode::from_values([1, 2, 3]);
        let node_count = RcListNode::nodes(&list).count();
        assert_eq!(node_count, 3);
    }
}
