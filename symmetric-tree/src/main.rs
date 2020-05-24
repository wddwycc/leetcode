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
    fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let (Some(p), Some(q)) = (&p, &q) {
            if p.borrow().val == q.borrow().val {
                return Self::is_same_tree(p.borrow().left.clone(), q.borrow().right.clone())
                    && Self::is_same_tree(p.borrow().right.clone(), q.borrow().left.clone());
            } else {
                return false;
            }
        } else if p.is_none() && q.is_none() {
            return true;
        }
        false
    }

    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(root) = root {
            return Self::is_same_tree(root.borrow().left.clone(), root.borrow().right.clone());
        }
        true
    }
}

fn main() {
    println!("Hello, world!");
}
