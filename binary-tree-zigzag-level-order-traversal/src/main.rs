use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution;
impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let root = match root {
            Some(a) => a,
            None => return result,
        };

        let mut reversed = false;
        let mut nodes = vec![root];
        while nodes.len() > 0 {
            let mut values = vec![];
            let mut next_nodes = vec![];
            for node in &nodes {
                values.push(node.borrow().val);
                if reversed {
                    if let Some(r) = node.borrow().right.clone() {
                        next_nodes.push(r)
                    }
                    if let Some(l) = node.borrow().left.clone() {
                        next_nodes.push(l)
                    }
                } else {
                    if let Some(l) = node.borrow().left.clone() {
                        next_nodes.push(l)
                    }
                    if let Some(r) = node.borrow().right.clone() {
                        next_nodes.push(r)
                    }
                }
            }
            next_nodes.reverse();
            nodes = next_nodes;
            result.push(values);
            reversed = !reversed;
        }

        result
    }
}

fn main() {
    // TODO
}
