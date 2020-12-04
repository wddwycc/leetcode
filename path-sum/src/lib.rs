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
    fn helper(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        let root = match root {
            Some(a) => a,
            _ => return sum == 0,
        };
        let next_sum = sum - root.borrow().val;

        let l_some = root.borrow().left.is_some();
        let r_some = root.borrow().right.is_some();

        if l_some && r_some {
            return Self::helper(root.borrow().left.clone(), next_sum)
                || Self::helper(root.borrow().right.clone(), next_sum);
        } else if l_some {
            return Self::helper(root.borrow().left.clone(), next_sum);
        } else if r_some {
            return Self::helper(root.borrow().right.clone(), next_sum);
        }
        return next_sum == 0;
    }

    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        if root.is_none() {
            return false;
        }
        return Self::helper(root, sum);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
