// Definition for a binary tree node.
use std::cell::RefCell;
use std::rc::Rc;

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

/// Ref: https://leetcode.com/problems/binary-tree-preorder-traversal/discuss/657035/Rust-DFS-iteratively-recursively
pub struct Solution;
impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        Self::recursively(root)
    }

    fn iteratively(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let root = match root {
            Some(a) => a,
            None => return vec![],
        };

        let mut res = vec![];
        let mut stack = vec![root];
        while let Some(node) = stack.pop() {
            res.push(node.borrow().val);
            if let Some(ref r) = node.borrow().right {
                stack.push(r.clone());
            }
            if let Some(ref l) = node.borrow().left {
                stack.push(l.clone());
            }
        }
        res
    }

    pub fn recursively(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let root = match root {
            Some(a) => a,
            None => return vec![],
        };

        let mut res = vec![];
        res.push(root.borrow().val);
        res.append(&mut Self::preorder_traversal(root.borrow().left.clone()));
        res.append(&mut Self::preorder_traversal(root.borrow().right.clone()));
        res
    }
}

fn main() {
    println!("Hello, world!");
}
