use std::{cell::RefCell, rc::Rc};

use crate::utils::graph::Node;

pub fn clone_graph(node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
    let mut new_nodes = vec![None; 100];
    let mut adjacency_lists: Vec<Vec<usize>> = vec![vec![]; 100];
    let mut stack = vec![node?];
    let mut seen = [false; 100];
    seen[0] = true;
    while let Some(node) = stack.pop() {
        let value = node.borrow().value;
        let index = (value - 1) as usize;
        new_nodes[index] = Some(Rc::new(RefCell::new(Node::new(value))));
        adjacency_lists[index] = node
            .borrow()
            .neighbors
            .iter()
            .map(|n| (n.borrow().value - 1) as usize)
            .collect();
        for neighbor in &node.borrow().neighbors {
            let neighbor_value = neighbor.borrow().value;
            let neighbor_index = (neighbor_value - 1) as usize;
            let neighbor_seen = &mut seen[neighbor_index];
            if !*neighbor_seen {
                *neighbor_seen = true;
                stack.push(neighbor.clone());
            }
        }
    }
    for i in 0..100 {
        if let Some(node) = new_nodes[i].as_ref() {
            node.borrow_mut().neighbors = adjacency_lists[i]
                .iter()
                .map(|&j| new_nodes[j].as_ref().cloned().unwrap())
                .collect()
        }
    }
    new_nodes[0].take()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::graph::Graph;

    #[test]
    fn test_example_1() {
        let adjacency_lists = vec![vec![2, 4], vec![1, 3], vec![2, 4], vec![1, 3]];
        let graph = Graph::new(&adjacency_lists);
        let cloned = Graph {
            node: clone_graph(graph.node),
        };
        assert_eq!(cloned.to_adjacency_lists(), adjacency_lists);
    }

    #[test]
    fn test_example_2() {
        let adjacency_lists = vec![vec![]];
        let graph = Graph::new(&adjacency_lists);
        let cloned = Graph {
            node: clone_graph(graph.node),
        };
        assert_eq!(cloned.to_adjacency_lists(), adjacency_lists);
    }

    #[test]
    fn test_example_3() {
        let adjacency_lists: Vec<Vec<i32>> = vec![];
        let graph = Graph::new(&adjacency_lists);
        let cloned = Graph {
            node: clone_graph(graph.node),
        };
        assert_eq!(cloned.to_adjacency_lists(), adjacency_lists);
    }
}
