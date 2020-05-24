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

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    // Iter for len
    let mut len = 0;
    let mut cur = &head;
    while let Some(node) = cur.as_ref() {
        cur = &node.next;
        len += 1;
    }
    // Iter to rm node
    let mut nth = (len as i32) - n;
    let mut head = head;
    let mut cur = &mut head;

    while nth > 0 {
        cur = &mut cur.as_mut().unwrap().next;
        nth -= 1;
    }
    *cur = cur.clone().unwrap().next;

    head
}

fn main() {
    println!("Hello, world!");
}
