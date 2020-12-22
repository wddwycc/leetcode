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

use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;
impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::helper(root).1
    }

    /// Returns (max_depth, max_diameter)
    fn helper(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        match root {
            Some(a) => {
                let l = Self::helper(a.borrow().left.clone());
                let r = Self::helper(a.borrow().right.clone());
                let max_depth = std::cmp::max(l.0, r.0) + 1;
                let mut max_diameter = l.0 + r.0;
                max_diameter = max_diameter.max(l.1);
                max_diameter = max_diameter.max(r.1);
                (max_depth, max_diameter)
            }
            None => (0, 0),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
