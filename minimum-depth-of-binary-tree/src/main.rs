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
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let root = match root {
            Some(a) => a,
            None => return 0,
        };
        let mut depth = 1;
        let mut deque = std::collections::VecDeque::new();
        deque.push_back(root);
        while !deque.is_empty() {
            for _ in 0..deque.len() {
                let node = deque.pop_front().unwrap();
                match (
                    node.clone().borrow().left.clone(),
                    node.clone().borrow().right.clone(),
                ) {
                    (Some(l), Some(r)) => {
                        deque.push_back(l);
                        deque.push_back(r);
                    }
                    (Some(l), _) => {
                        deque.push_back(l);
                    }
                    (_, Some(r)) => {
                        deque.push_back(r);
                    }
                    (None, None) => return depth,
                }
            }
            depth += 1
        }
        depth
    }
}

fn main() {
    let tree = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: None,
    })));
    assert_eq!(Solution::min_depth(tree), 2)
}
