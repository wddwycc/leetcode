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
use std::collections::HashMap;
use std::rc::Rc;

pub struct Solution;
impl Solution {
    pub fn find_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let root = match root {
            Some(a) => a,
            None => return vec![],
        };
        let mut node_by_height = HashMap::new();
        Self::dfs(root, &mut node_by_height);
        let mut node_by_height = node_by_height.into_iter().collect::<Vec<_>>();
        node_by_height.sort_by_key(|&(k, _)| k);
        node_by_height.into_iter().map(|(_, v)| v).collect()
    }

    fn dfs(node: Rc<RefCell<TreeNode>>, node_by_height: &mut HashMap<i32, Vec<i32>>) -> i32 {
        let height = match (node.borrow().left.as_ref(), node.borrow().right.as_ref()) {
            (Some(l), Some(r)) => {
                let l_height = Self::dfs(l.clone(), node_by_height);
                let r_height = Self::dfs(r.clone(), node_by_height);
                std::cmp::max(l_height, r_height) + 1
            }
            (Some(c), None) | (None, Some(c)) => Self::dfs(c.clone(), node_by_height) + 1,
            (None, None) => 0,
        };
        node_by_height
            .entry(height)
            .or_insert(vec![])
            .push(node.borrow().val);
        height
    }
}
