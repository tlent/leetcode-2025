use std::{cell::RefCell, collections::VecDeque, rc::Rc};
#[derive(Debug, PartialEq, Eq, Default)]
pub struct TreeNode {
    pub value: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(value: i32) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Tree {
    pub root: Option<Rc<RefCell<TreeNode>>>,
}

impl Tree {
    pub fn new<V: IntoIterator<Item = Option<i32>>>(values: V) -> Self {
        let mut nodes: Vec<_> = values
            .into_iter()
            .map(|value| value.map(|v| Rc::new(RefCell::new(TreeNode::new(v)))))
            .collect();
        if nodes.is_empty() {
            return Tree { root: None };
        }
        for (i, node) in nodes.iter().enumerate() {
            if let Some(n) = node {
                let left_index = i * 2 + 1;
                if left_index < nodes.len() {
                    n.borrow_mut().left = nodes[left_index].clone();
                }
                let right_index = left_index + 1;
                if right_index < nodes.len() {
                    n.borrow_mut().right = nodes[right_index].clone();
                }
            }
        }
        Tree {
            root: nodes.remove(0),
        }
    }

    pub fn nodes(&self) -> impl Iterator<Item = Rc<RefCell<TreeNode>>> {
        let mut queue = VecDeque::new();
        if let Some(root_node) = &self.root {
            queue.push_back(root_node.clone());
        }
        std::iter::from_fn(move || match queue.pop_front() {
            None => None,
            Some(node) => {
                if let Some(l) = node.borrow().left.as_ref() {
                    queue.push_back(l.clone());
                }
                if let Some(r) = node.borrow().right.as_ref() {
                    queue.push_back(r.clone());
                }
                Some(node)
            }
        })
    }

    pub fn values(&self) -> impl Iterator<Item = i32> {
        self.nodes().map(|node| node.borrow().value)
    }

    pub fn to_vec(&self) -> Vec<i32> {
        self.values().collect()
    }
}
