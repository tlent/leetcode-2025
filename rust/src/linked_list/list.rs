use std::cell::RefCell;
use std::rc::Rc;

pub type BoxList = Option<Box<BoxListNode>>;
pub type RcList = Option<Rc<RefCell<RcListNode>>>;

#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub struct BoxListNode {
    pub val: i32,
    pub next: BoxList,
}

impl BoxListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        BoxListNode { next: None, val }
    }

    pub fn iter_nodes(&self) -> impl Iterator<Item = &Self> {
        std::iter::successors(Some(self), |node| node.next.as_deref())
    }

    pub fn values(&self) -> impl Iterator<Item = i32> {
        self.iter_nodes().map(|node| node.val)
    }

    /// Create a linked list from an iterator
    pub fn from_values<T: IntoIterator<Item = i32>>(iter: T) -> BoxList {
        let mut head = BoxListNode::default();
        let mut cursor = &mut head;
        for val in iter {
            cursor.next = Some(Box::new(BoxListNode::new(val)));
            cursor = cursor.next.as_mut().unwrap();
        }
        head.next
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub struct RcListNode {
    pub val: i32,
    pub next: RcList,
}

impl RcListNode {
    pub fn new(val: i32) -> Self {
        RcListNode { val, next: None }
    }

    pub fn iter_nodes(head: &Rc<RefCell<Self>>) -> impl Iterator<Item = Rc<RefCell<RcListNode>>> {
        let mut current = Some(head.clone());
        std::iter::from_fn(move || {
            current.take().inspect(|node| {
                current = node.borrow().next.clone();
            })
        })
    }

    pub fn values(head: &Rc<RefCell<Self>>) -> impl Iterator<Item = i32> {
        Self::iter_nodes(head).map(|node| node.borrow().val)
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
