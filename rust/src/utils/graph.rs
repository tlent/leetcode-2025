use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct Node {
    pub value: i32,
    pub neighbors: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(value: i32) -> Self {
        Self {
            value,
            neighbors: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Graph {
    pub node: Option<Rc<RefCell<Node>>>,
}

impl Graph {
    pub fn new(adjacency_lists: &[Vec<i32>]) -> Self {
        if adjacency_lists.is_empty() {
            return Self { node: None };
        }
        let nodes = (1..=adjacency_lists.len() as i32)
            .map(|value| Rc::new(RefCell::new(Node::new(value))))
            .collect::<Vec<_>>();
        for (i, list) in adjacency_lists.iter().enumerate() {
            nodes[i].borrow_mut().neighbors = list
                .iter()
                .map(|j| nodes[(j - 1) as usize].clone())
                .collect();
        }
        Self {
            node: Some(nodes[0].clone()),
        }
    }

    pub fn to_adjacency_lists(&self) -> Vec<Vec<i32>> {
        if self.node.is_none() {
            return vec![];
        }
        let mut max_value = 0;
        let mut result = vec![vec![]; 100];
        let mut visited = [false; 100];
        let mut stack = vec![self.node.as_ref().cloned().unwrap()];
        while let Some(node) = stack.pop() {
            let value = node.borrow().value;
            max_value = max_value.max(value);
            let adjacency_list: Vec<i32> = node
                .borrow()
                .neighbors
                .iter()
                .map(|n| n.borrow().value)
                .collect();
            result[(value - 1) as usize] = adjacency_list;
            for neighbor in &node.borrow().neighbors {
                let neighbor_index = (neighbor.borrow().value - 1) as usize;
                if !visited[neighbor_index] {
                    visited[neighbor_index] = true;
                    stack.push(neighbor.clone());
                }
            }
        }
        result.into_iter().take(max_value as usize).collect()
    }
}
