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

pub struct Solution {}
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = &head;
        let mut result: Option<Box<ListNode>> = None;
        while let Some(head_) = head {
            match result {
                Some(prev) => {
                    result = Some(Box::new(ListNode {
                        val: head_.val,
                        next: Some(prev),
                    }))
                }
                None => {
                    result = Some(Box::new(ListNode::new(head_.val)));
                }
            }
            head = &(head_.next);
        }

        result
    }
}

fn main() {
    println!("Hello, world!");
}
