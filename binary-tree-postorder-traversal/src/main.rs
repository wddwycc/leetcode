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

pub enum NodeState {
    NoAsk,
    LeftSeen,
    RightSeen,
}

struct Solution;
impl Solution {
    pub fn iteratively(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let root = match root {
            Some(root) => root,
            None => return vec![],
        };

        let mut res = vec![];
        let mut stack = vec![(root, NodeState::NoAsk)];
        while let Some((node, state)) = stack.pop().as_ref() {
            match state {
                NodeState::NoAsk => {
                    stack.push((node.clone(), NodeState::LeftSeen));
                    if let Some(ref l) = node.borrow().left {
                        stack.push((l.clone(), NodeState::NoAsk));
                    }
                }
                NodeState::LeftSeen => {
                    stack.push((node.clone(), NodeState::RightSeen));
                    if let Some(ref r) = node.borrow().right {
                        stack.push((r.clone(), NodeState::NoAsk));
                    }
                }
                NodeState::RightSeen => {
                    res.push(node.borrow().val);
                }
            }
        }
        res
    }

    pub fn recursively(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let root = match root {
            Some(root) => root,
            None => return vec![],
        };

        let mut res = vec![];
        res.append(&mut Self::recursively(root.borrow().left.clone()));
        res.append(&mut Self::recursively(root.borrow().right.clone()));
        res.push(root.borrow().val);
        res
    }
}

fn main() {
    println!("Hello, world!");
}
