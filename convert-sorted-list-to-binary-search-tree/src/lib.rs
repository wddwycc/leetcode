// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
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
    pub fn sorted_list_to_bst(mut head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut list = vec![];
        while let Some(node) = head {
            list.push(node.val);
            head = node.next;
        }
        Self::sorted_slice_to_bst(&list)
    }

    fn sorted_slice_to_bst(list: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if list.len() == 0 {
            return None;
        }
        let mid = list.len() / 2;
        return Some(Rc::new(RefCell::new(TreeNode {
            val: list[mid],
            left: Self::sorted_slice_to_bst(&list[..mid]),
            right: Self::sorted_slice_to_bst(&list[(mid + 1)..]),
        })));
    }
}
