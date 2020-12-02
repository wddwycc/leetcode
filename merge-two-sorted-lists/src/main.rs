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
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => None,
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (Some(mut a), Some(mut b)) => {
                if a.val > b.val {
                    b.next = Self::merge_two_lists(b.next, Some(a));
                    return Some(b);
                } else {
                    a.next = Self::merge_two_lists(a.next, Some(b));
                    return Some(a);
                }
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
