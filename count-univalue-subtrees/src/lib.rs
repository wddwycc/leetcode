use std::cell::RefCell;
use std::rc::Rc;

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
    // Two ways to be a universal subtree
    // 1. be the leaf
    // 2. has all children and itself with same values
    fn is_unival_subtree(node: &Rc<RefCell<TreeNode>>) -> bool {
        match (node.borrow().left.as_ref(), node.borrow().right.as_ref()) {
            (Some(l), Some(r)) => {
                Self::is_unival_subtree(l)
                    && Self::is_unival_subtree(r)
                    && l.borrow().val == node.borrow().val
                    && r.borrow().val == node.borrow().val
            }
            (Some(l), None) => Self::is_unival_subtree(l) && l.borrow().val == node.borrow().val,
            (None, Some(r)) => Self::is_unival_subtree(r) && r.borrow().val == node.borrow().val,
            (None, None) => true,
        }
    }

    pub fn count_unival_subtrees(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let root = match root {
            Some(a) => a,
            None => return 0,
        };
        let mut res = 0;
        let mut stack = vec![];
        stack.push(root);
        while let Some(node) = stack.pop() {
            if Self::is_unival_subtree(&node) {
                res += 1;
            }
            if let Some(l) = node.borrow().left.as_ref() {
                stack.push(l.clone());
            }
            if let Some(r) = node.borrow().right.as_ref() {
                stack.push(r.clone());
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
