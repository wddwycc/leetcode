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

use std::collections::HashMap;

pub struct Solution;
impl Solution {
    pub fn delete_duplicates_unsorted(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // traverse twice, fst for dist collect, snd for deletion
        let dist = {
            let mut res = HashMap::new();
            let mut cur = &head;
            while let Some(ref node) = cur {
                *res.entry(node.val).or_insert(0) += 1;
                cur = &node.next;
            }
            res
        };
        let mut dummy_head = ListNode { val: 0, next: head };
        let mut cur = &mut dummy_head;
        while cur.next.is_some() {
            let val = cur.next.as_ref().unwrap().val;
            if *dist.get(&val).unwrap() > 1 {
                cur.next = cur.next.as_mut().unwrap().next.take();
            } else {
                cur = cur.next.as_mut().unwrap();
            }
        }
        dummy_head.next
    }
}
