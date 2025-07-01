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
pub struct List {
    pub head: Option<Box<ListNode>>,
}

impl List {
    pub fn new<V: IntoIterator<Item = i32>>(values: V) -> Self {
        let mut head = ListNode::default();
        let mut cursor = &mut head;
        for value in values {
            cursor.next = Some(Box::new(ListNode::new(value)));
            cursor = cursor.next.as_mut().unwrap();
        }
        List { head: head.next }
    }

    pub fn nodes(&self) -> impl Iterator<Item = &ListNode> {
        let mut current = self.head.as_deref();
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
pub struct SharedList {
    pub head: Option<Rc<RefCell<SharedListNode>>>,
}

impl SharedList {
    pub fn new<V: IntoIterator<Item = i32>>(values: V) -> Self {
        let nodes: Vec<Rc<RefCell<SharedListNode>>> = values
            .into_iter()
            .map(|val| Rc::new(RefCell::new(SharedListNode::new(val))))
            .collect();
        if nodes.is_empty() {
            return SharedList { head: None };
        }

        // Link the nodes
        for i in 0..nodes.len() - 1 {
            nodes[i].borrow_mut().next = Some(nodes[i + 1].clone());
        }

        SharedList {
            head: Some(nodes[0].clone()),
        }
    }

    pub fn nodes(&self) -> impl Iterator<Item = Rc<RefCell<SharedListNode>>> {
        let mut current = self.head.as_ref().map(Rc::clone);
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
}
