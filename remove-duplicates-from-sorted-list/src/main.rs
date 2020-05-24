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

pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = head;
    let mut cur = &mut head;
    while let Some(mut node) = cur.take() {
        if let Some(next_node) = node.next.clone() {
            if node.val == next_node.val {
                node.next = next_node.next;
            } else {
                cur.unwrap().next = Some(next_node);
                cur = cur.unwrap().next;
            }
            cur = &mut node.next;
        }
    }

    None
}

fn main() {
    println!("Hello, world!");
}
