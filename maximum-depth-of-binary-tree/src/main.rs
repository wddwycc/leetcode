use std::cell::{Ref, RefCell};
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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(a) => {
                let l = Solution::max_depth(a.borrow().left.clone());
                let r = Solution::max_depth(a.borrow().right.clone());
                return std::cmp::max(l, r) + 1;
            }
            None => return 0,
        }
    }
}

fn main() {
    println!("Hello, world!");
}
