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
    pub fn trim_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        low: i32,
        high: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            Some(tree) => {
                if tree.borrow().val < low {
                    return Solution::trim_bst(tree.borrow().right.clone(), low, high);
                }
                if tree.borrow().val > high {
                    return Solution::trim_bst(tree.borrow().left.clone(), low, high);
                }
                let new_tree = TreeNode {
                    val: tree.borrow().val,
                    left: Solution::trim_bst(tree.borrow().left.clone(), low, high),
                    right: Solution::trim_bst(tree.borrow().right.clone(), low, high),
                };
                return Some(Rc::new(RefCell::new(new_tree)));
            }
            None => None,
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
