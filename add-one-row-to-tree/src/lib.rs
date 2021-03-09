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
use std::collections::VecDeque;
use std::rc::Rc;

pub struct Solution;
impl Solution {
    pub fn add_one_row(
        root: Option<Rc<RefCell<TreeNode>>>,
        v: i32,
        d: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let root = match root {
            Some(a) => a,
            None => return None,
        };

        if d == 1 {
            return Some(Rc::new(RefCell::new(TreeNode {
                val: v,
                left: Some(root),
                right: None,
            })));
        }

        let mut depth = 1;
        let mut queue = VecDeque::new();
        queue.push_back(root.clone());

        while queue.len() > 0 {
            println!("{:?}", queue.len());
            if depth + 1 == d {
                while let Some(node) = queue.pop_front() {
                    if node.borrow().left.is_some() {
                        let mut borrow = node.borrow_mut();
                        borrow.left = Some(Rc::new(RefCell::new(TreeNode {
                            val: v,
                            left: borrow.left.take(),
                            right: None,
                        })));
                    } else {
                        node.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode {
                            val: v,
                            left: None,
                            right: None,
                        })));
                    }
                    if node.borrow().right.is_some() {
                        let mut borrow = node.borrow_mut();
                        borrow.right = Some(Rc::new(RefCell::new(TreeNode {
                            val: v,
                            left: None,
                            right: borrow.right.take(),
                        })));
                    } else {
                        node.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode {
                            val: v,
                            left: None,
                            right: None,
                        })));
                    }
                }
                break;
            }

            for _ in 0..queue.len() {
                let node = queue.pop_front().unwrap();
                if let Some(ref l) = node.clone().borrow().left {
                    queue.push_back(l.clone());
                }
                if let Some(ref r) = node.clone().borrow().right {
                    queue.push_back(r.clone());
                }
            }

            depth += 1;
        }
        Some(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
