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
    pub fn inorder_successor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let root = match root {
            Some(a) => a,
            None => return None,
        };
        let p = match p {
            Some(a) => a.borrow().val,
            None => return None,
        };

        let mut met_p = false;
        let mut stack = vec![(root, false)];
        while let Some((node, left_visited)) = stack.pop() {
            if left_visited {
                // -- start consume node
                if met_p {
                    return Some(node);
                }
                if node.borrow().val == p {
                    met_p = true;
                }
                // -- end consume node
                if let Some(ref r) = node.borrow().right {
                    stack.push((r.clone(), false));
                }
            } else {
                stack.push((node.clone(), true));
                if let Some(ref l) = node.borrow().left {
                    stack.push((l.clone(), false));
                };
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
