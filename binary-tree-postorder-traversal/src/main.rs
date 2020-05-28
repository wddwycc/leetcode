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
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        match root {
            None => return vec![],
            Some(root) => {
                let mut stack1 = vec![root];
                let mut stack2 = vec![];
                while let Some(node) = stack1.pop() {
                    // TODO
                    if let Some(left) = node.borrow().left.clone() {
                        stack1.push(left)
                    }
                    if let Some(right) = node.borrow().right.clone() {
                        stack1.push(right)
                    }
                    stack2.push(node);
                }
                let mut result = vec![];
                while let Some(node) = stack2.pop() {
                    result.push(node.borrow().val);
                }
                return result;
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
