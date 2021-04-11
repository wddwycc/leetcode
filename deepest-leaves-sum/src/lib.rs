use std::cell::RefCell;
use std::collections::VecDeque;
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
    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let root = match root {
            Some(a) => a,
            None => return 0,
        };

        let mut queue = VecDeque::new();
        queue.push_back(root);

        while queue.len() > 0 {
            let mut sum = 0;
            for _ in 0..queue.len() {
                let node = queue.pop_front().unwrap();
                sum += node.borrow().val;
                if let Some(ref l) = node.borrow().left {
                    queue.push_back(l.clone());
                };
                if let Some(ref r) = node.borrow().right {
                    queue.push_back(r.clone());
                };
            }
            if queue.len() == 0 {
                return sum;
            }
        }
        panic!();
    }
}
