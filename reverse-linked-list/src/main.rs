// Definition for singly-linked list.
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

use std::mem;

pub struct Solution {}
impl Solution {
    fn helper(
        head: Option<Box<ListNode>>,
        reversed: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if let Some(mut node) = head {
            let new_head = mem::replace(&mut node.next, reversed);
            Self::helper(new_head, Some(node))
        } else {
            reversed
        }
    }

    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::helper(head, None)
    }
}

fn main() {
    println!("Hello, world!");
}
