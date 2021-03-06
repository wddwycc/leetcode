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

pub struct Solution;
impl Solution {
    pub fn iteratively(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let root = match root {
            Some(a) => a,
            None => return vec![],
        };

        let mut res = vec![];
        let mut stack = vec![(root, false)];
        while let Some((ref node, left_seen)) = stack.pop() {
            if left_seen {
                res.push(node.borrow().val);
                if let Some(ref r) = node.borrow().right {
                    stack.push((r.clone(), false));
                };
            } else {
                stack.push((node.clone(), true));
                if let Some(ref l) = node.borrow().left {
                    stack.push((l.clone(), false));
                };
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
        res.append(&mut Self::recursively(root.borrow().left.clone()));
        res.push(root.borrow().val);
        res.append(&mut Self::recursively(root.borrow().right.clone()));
        res
    }
}

fn main() {
    println!("Hello, world!");
}
