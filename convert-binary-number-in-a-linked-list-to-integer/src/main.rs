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

pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
    let mut cur = head;
    let mut rv: i32 = 0;
    while let Some(node) = cur {
        rv = rv * 2 + node.val;
        cur = node.next;
    }
    rv
}

fn main() {
    println!("Hello, world!");
}
