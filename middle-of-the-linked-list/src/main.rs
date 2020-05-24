// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut fast_cur = &head;
    let mut slow_cur = &head;
    let mut idx = 0;
    while let Some(node) = fast_cur {
        if idx % 2 == 1 {
            slow_cur = &slow_cur.as_ref().unwrap().next;
        }
        fast_cur = &node.next;
        idx += 1;
    }
    slow_cur.clone()
}

fn main() {
    println!("Hello, world!");
}
