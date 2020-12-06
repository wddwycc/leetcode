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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let root = match root {
            None => return vec![],
            Some(a) => a,
        };
        let mut stack = vec![(root, false)];
        let mut result = vec![];
        while let Some((node, left_seen)) = stack.pop() {
            if left_seen {
                result.push(node.borrow().val);
                if let Some(right) = node.borrow().right.clone() {
                    stack.push((right, false))
                }
            } else {
                stack.push((node.clone(), true));
                if let Some(left) = node.borrow().left.clone() {
                    stack.push((left, false))
                }
            }
        }
        result
    }

    pub fn recursively(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let root = match root {
            Some(a) => a,
            None => return vec![],
        };

        let mut res = vec![];
        res.append(&mut Self::recursively(root.borrow().left.clone()));
        res.push(root.borrow().val);
        res.append(&mut Self::recursively(root.borrow().right.clone()));
        res
    }
}

fn main() {
    println!("Hello, world!");
}
