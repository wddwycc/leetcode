use std::{cell::RefCell, rc::Rc};

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

struct BSTIterator {
    stack: Vec<(Rc<RefCell<TreeNode>>, bool)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {
    pub fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        Self {
            stack: root.map(|a| vec![(a, false)]).unwrap_or(vec![]),
        }
    }

    /** @return the next smallest number */
    pub fn next(&mut self) -> i32 {
        while let Some((node, left_seen)) = self.stack.pop() {
            if left_seen {
                if let Some(ref r) = node.borrow().right {
                    self.stack.push((r.clone(), false));
                }
                return node.borrow().val;
            } else {
                self.stack.push((node.clone(), true));
                if let Some(ref l) = node.borrow().left {
                    self.stack.push((l.clone(), false));
                }
            }
        }
        panic!()
    }

    /** @return whether we have a next smallest number */
    pub fn has_next(&self) -> bool {
        self.stack.len() > 0
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
