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

struct Solution;
impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        match root {
            None => return vec![],
            Some(root) => {
                let mut result = vec![];
                let mut stack = vec![root];
                while let Some(node) = stack.pop() {
                    result.push(node.borrow().val);
                    if let Some(right) = node.borrow().right.clone() {
                        stack.push(right)
                    }
                    if let Some(left) = node.borrow().left.clone() {
                        stack.push(left)
                    }
                }
                return result;
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
