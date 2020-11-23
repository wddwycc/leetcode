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

pub struct Solution;
impl Solution {
    pub fn is_subtree(s: Option<Rc<RefCell<TreeNode>>>, t: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let (s, t) = match (s, t) {
            (Some(s), Some(t)) => (s, t),
            _ => return false,
        };
        let mut stack = vec![s];
        // NOTE: DFS
        while let Some(node) = stack.pop() {
            if let Some(left) = node.borrow().left.clone() {
                stack.push(left)
            }
            if let Some(right) = node.borrow().right.clone() {
                stack.push(right)
            }
            if node.borrow().val == t.borrow().val {
                if node == t.clone() {
                    return true;
                }
            }
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
