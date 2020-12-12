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

pub struct TreeNodeAndDepth(Option<Rc<RefCell<TreeNode>>>, usize);

pub struct Solution;
impl Solution {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> TreeNodeAndDepth {
        let root = match root {
            Some(a) => a,
            None => return TreeNodeAndDepth(None, 0),
        };
        let l = Self::dfs(root.borrow().left.clone());
        let r = Self::dfs(root.borrow().right.clone());
        if l.1 > r.1 {
            return TreeNodeAndDepth(l.0, l.1 + 1);
        } else if l.1 < r.1 {
            return TreeNodeAndDepth(r.0, r.1 + 1);
        } else {
            return TreeNodeAndDepth(Some(root), l.1 + 1);
        }
    }

    pub fn subtree_with_all_deepest(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::dfs(root).0
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
