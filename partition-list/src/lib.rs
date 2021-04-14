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

pub struct Solution;
impl Solution {
    pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut dummy_head1 = Box::new(ListNode::new(0));
        let mut cur1 = &mut dummy_head1;

        let mut dummy_head2 = Box::new(ListNode::new(0));
        let mut cur2 = &mut dummy_head2;

        while let Some(node) = head {
            if node.val < x {
                cur1.next = Some(Box::new(ListNode::new(node.val)));
                cur1 = cur1.next.as_mut().unwrap();
            } else {
                cur2.next = Some(Box::new(ListNode::new(node.val)));
                cur2 = cur2.next.as_mut().unwrap();
            }
            head = node.next;
        }

        cur1.next = dummy_head2.next;
        dummy_head1.next
    }
}
