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

pub fn merge_two_lists(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut cur1 = l1;
    let mut cur2 = l2;

    let mut dummy_head = ListNode::new(0);
    let mut cur = &mut dummy_head;

    loop {
        match (cur1.take(), cur2.take()) {
            (Some(a), Some(b)) => {
                if a.val < b.val {
                    cur.next = Some(Box::new(ListNode::new(a.val)));
                    cur = cur.next.as_mut().unwrap();
                    cur1 = a.next;
                    cur2 = Some(b);
                } else {
                    cur.next = Some(Box::new(ListNode::new(b.val)));
                    cur = cur.next.as_mut().unwrap();
                    cur1 = Some(a);
                    cur2 = b.next;
                }
            }
            (Some(a), None) => {
                cur.next = Some(Box::new(ListNode::new(a.val)));
                cur = cur.next.as_mut().unwrap();
                cur1 = a.next;
            }
            (None, Some(b)) => {
                cur.next = Some(Box::new(ListNode::new(b.val)));
                cur = cur.next.as_mut().unwrap();
                cur2 = b.next;
            }
            _ => return dummy_head.next,
        }
    }
}

fn main() {
    println!("Hello, world!");
}
