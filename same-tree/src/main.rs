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

pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (p, q) {
        (Some(p), Some(q)) => {
            return p.borrow().val == q.borrow().val
                && is_same_tree(p.borrow().left.clone(), q.borrow().left.clone())
                && is_same_tree(p.borrow().right.clone(), q.borrow().right.clone())
        }
        (None, None) => true,
        _ => false,
    }
}

fn main() {
    println!("Hello, world!");
}
