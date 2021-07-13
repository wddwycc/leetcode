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
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        Self::helper(1, n)
    }

    fn helper(l: i32, r: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if l == r {
            return vec![Some(Rc::new(RefCell::new(TreeNode::new(l))))];
        }
        let mut res = vec![];
        for pivot in l..=r {
            let l_trees = if pivot == l {
                vec![None]
            } else {
                Self::helper(l, pivot - 1)
            };
            let r_trees = if pivot == r {
                vec![None]
            } else {
                Self::helper(pivot + 1, r)
            };
            for i in 0..l_trees.len() {
                for j in 0..r_trees.len() {
                    let tree = TreeNode {
                        val: pivot,
                        left: l_trees[i].clone(),
                        right: r_trees[j].clone(),
                    };
                    res.push(Some(Rc::new(RefCell::new(tree))));
                }
            }
        }
        res
    }
}
