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
    pub fn merge_trees(
        t1: Option<Rc<RefCell<TreeNode>>>,
        t2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match (t1, t2) {
            (Some(a), Some(b)) => Some(Rc::new(RefCell::new(TreeNode {
                val: a.borrow().val + b.borrow().val,
                left: Solution::merge_trees(a.borrow().left.clone(), b.borrow().left.clone()),
                right: Solution::merge_trees(a.borrow().right.clone(), b.borrow().right.clone()),
            }))),
            (Some(a), None) => Some(Rc::new(RefCell::new(TreeNode {
                val: a.borrow().val,
                left: a.borrow().left.clone(),
                right: a.borrow().right.clone(),
            }))),
            (None, Some(b)) => Some(Rc::new(RefCell::new(TreeNode {
                val: b.borrow().val,
                left: b.borrow().left.clone(),
                right: b.borrow().right.clone(),
            }))),
            (None, None) => None,
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
