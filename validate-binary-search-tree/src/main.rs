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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::helper(root, None, None)
    }

    fn helper(
        root: Option<Rc<RefCell<TreeNode>>>,
        l_bound: Option<i32>,
        r_bound: Option<i32>,
    ) -> bool {
        let root = match root {
            Some(a) => a,
            None => return true,
        };
        let val = root.borrow().val;
        if let Some(l) = l_bound {
            if val <= l {
                return false;
            }
        }
        if let Some(r) = r_bound {
            if val >= r {
                return false;
            }
        }
        if !Self::helper(root.borrow().left.clone(), l_bound, Some(val)) {
            return false;
        }
        if !Self::helper(root.borrow().right.clone(), Some(val), r_bound) {
            return false;
        }
        true
    }
}
fn main() {
    println!("Hello, world!");
}
