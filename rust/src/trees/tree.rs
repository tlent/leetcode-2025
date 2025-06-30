use std::{cell::RefCell, collections::VecDeque, rc::Rc};

pub type Tree = Option<Rc<RefCell<TreeNode>>>;

#[derive(Debug, PartialEq, Eq, Default)]
pub struct TreeNode {
    value: i32,
    left: Tree,
    right: Tree,
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

    pub fn from_array<T: IntoIterator<Item = i32>>(values: T) -> Tree {
        let mut nodes: Vec<_> = values
            .into_iter()
            .map(|value| Rc::new(RefCell::new(TreeNode::new(value))))
            .collect();
        if nodes.is_empty() {
            return None;
        }
        for (i, node) in nodes.iter().enumerate() {
            let left_index = i * 2 + 1;
            if left_index < nodes.len() {
                node.borrow_mut().left = Some(nodes[left_index].clone());
            }
            let right_index = left_index + 1;
            if right_index < nodes.len() {
                node.borrow_mut().right = Some(nodes[right_index].clone());
            }
        }
        Some(nodes.remove(0))
    }

    pub fn values(root: &Tree) -> impl Iterator<Item = i32> {
        Self::nodes(root).map(|node| node.borrow().value)
    }

    pub fn nodes(root: &Tree) -> impl Iterator<Item = Rc<RefCell<TreeNode>>> {
        let mut queue = VecDeque::new();
        if let Some(root_node) = root {
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
}
