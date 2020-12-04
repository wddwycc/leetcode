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
    pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut vals = vec![];
        Self::dfs_in_order(root, &mut vals);
        let dummy_head = Rc::new(RefCell::new(TreeNode::new(0)));
        let mut cur = dummy_head.clone();
        for val in vals {
            let node = Rc::new(RefCell::new(TreeNode::new(val)));
            cur.borrow_mut().right = Some(node.clone());
            cur = node;
        }
        let res = dummy_head.borrow().right.clone();
        res
    }

    fn dfs_in_order(node: Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
        match node {
            Some(a) => {
                Self::dfs_in_order(a.borrow().left.clone(), res);
                res.push(a.borrow().val);
                Self::dfs_in_order(a.borrow().right.clone(), res);
            }
            None => return,
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
