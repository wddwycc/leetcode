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
    pub fn sorted_array_to_bst_with_range(
        nums: &Vec<i32>,
        start: usize,
        end: usize,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if start == end {
            return Some(Rc::new(RefCell::new(TreeNode::new(nums[start]))));
        }
        let mid = (end + start) / 2;
        return Some(Rc::new(RefCell::new(TreeNode {
            val: nums[mid],
            left: {
                if mid == start {
                    None
                } else {
                    Self::sorted_array_to_bst_with_range(&nums, start, mid - 1)
                }
            },
            right: {
                if mid == end {
                    None
                } else {
                    Self::sorted_array_to_bst_with_range(&nums, mid + 1, end)
                }
            },
        })));
    }

    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let len = nums.len();
        if len == 0 {
            return None;
        }
        Self::sorted_array_to_bst_with_range(&nums, 0, len - 1)
    }
}

fn main() {
    println!("Hello, world!");
}
