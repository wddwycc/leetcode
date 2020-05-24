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

pub struct Solution {}
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut nodes: Vec<Rc<RefCell<TreeNode>>> = match root {
            Some(a) => vec![a],
            None => vec![],
        };
        let mut matrix = vec![];
        while nodes.len() > 0 {
            let mut row = vec![];
            let mut next_nodes: Vec<Rc<RefCell<TreeNode>>> = vec![];
            for node in &nodes {
                if let Some(l) = node.borrow().left.clone() {
                    next_nodes.push(l);
                }
                if let Some(r) = node.borrow().right.clone() {
                    next_nodes.push(r);
                }
                row.push(node.borrow().val.clone());
            }
            matrix.push(row);
            nodes = next_nodes;
        }

        matrix
    }
}

fn main() {
    println!("Hello, world!");
}
