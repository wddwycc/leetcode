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

pub enum Limit {
    Min(i32),
    Max(i32),
    None,
}

pub struct Solution {}
impl Solution {
    pub fn is_valid_bst_recursion(
        root: Option<Rc<RefCell<TreeNode>>>,
        min_bound: Option<i32>,
        max_bound: Option<i32>,
    ) -> bool {
        if let Some(tree) = root {
            let tree = tree.borrow();
            let val = tree.val;
            let l_root = tree.left.clone();
            let r_root = tree.right.clone();

            if let Some(l) = &l_root {
                let l_val = l.borrow().val;
                if l_val >= val {
                    return false;
                }
                if let Some(a) = min_bound {
                    if l_val <= a {
                        return false;
                    }
                }
            }
            if let Some(r) = &r_root {
                let r_val = r.borrow().val;
                if r_val <= val {
                    return false;
                }
                if let Some(a) = max_bound {
                    if r_val >= a {
                        return false;
                    }
                }
            }
            if !Solution::is_valid_bst_recursion(l_root, min_bound, Some(val)) {
                return false;
            }
            if !Solution::is_valid_bst_recursion(r_root, Some(val), max_bound) {
                return false;
            }
            return true;
        } else {
            return true;
        }
    }

    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Solution::is_valid_bst_recursion(root, None, None)
    }
}

fn main() {
    println!("Hello, world!");
}
