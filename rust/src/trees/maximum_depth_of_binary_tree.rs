use std::{cell::RefCell, rc::Rc};

use crate::trees::tree::TreeNode;

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> usize {
    match root {
        Some(node) => {
            let left = node.borrow().left.clone();
            let right = node.borrow().right.clone();
            max_depth(left).max(max_depth(right)) + 1
        }
        None => 0,
    }
}

pub fn max_depth_iterative(root: Option<Rc<RefCell<TreeNode>>>) -> usize {
    let mut max_depth = 0;
    if let Some(r) = root {
        let mut stack = vec![(r, 1)];
        while let Some((node, depth)) = stack.pop() {
            max_depth = max_depth.max(depth);
            if let Some(l) = node.borrow().left.clone() {
                stack.push((l, depth + 1));
            }
            if let Some(r) = node.borrow().right.clone() {
                stack.push((r, depth + 1));
            }
        }
    }
    max_depth
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::trees::tree::Tree;

    #[test]
    fn test_max_depth_example_one() {
        let tree = Tree::new([Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
        assert_eq!(max_depth(tree.root), 3);
    }

    #[test]
    fn test_max_depth_example_two() {
        let tree = Tree::new([Some(1), None, Some(2)]);
        assert_eq!(max_depth(tree.root), 2);
    }

    #[test]
    fn test_max_depth_iterative_example_one() {
        let tree = Tree::new([Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
        assert_eq!(max_depth_iterative(tree.root), 3);
    }

    #[test]
    fn test_max_depth_iterative_example_two() {
        let tree = Tree::new([Some(1), None, Some(2)]);
        assert_eq!(max_depth_iterative(tree.root), 2);
    }
}
