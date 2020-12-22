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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::helper(root).0
    }

    fn helper(root: Option<Rc<RefCell<TreeNode>>>) -> (bool, i32) {
        let root = match root {
            Some(a) => a,
            None => return (true, 0),
        };
        let (l_valid, l_height) = Self::helper(root.borrow().left.clone());
        let (r_valid, r_height) = Self::helper(root.borrow().right.clone());
        let height = std::cmp::max(l_height, r_height) + 1;
        let valid = l_valid && r_valid && (l_height - r_height).abs() <= 1;
        (valid, height)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
