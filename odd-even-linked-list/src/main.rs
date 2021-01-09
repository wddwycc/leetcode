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

pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut odd_dummy_head = ListNode::new(-1);
    let mut even_dummy_head = ListNode::new(-1);
    let mut odd_cur = &mut odd_dummy_head;
    let mut even_cur = &mut even_dummy_head;

    let mut is_even = false;
    while let Some(mut node) = head {
        head = std::mem::replace(&mut node.next, None);
        if is_even {
            even_cur.next = Some(node);
            even_cur = even_cur.next.as_mut().unwrap();
        } else {
            odd_cur.next = Some(node);
            odd_cur = odd_cur.next.as_mut().unwrap();
        }
        is_even = !is_even;
    }

    odd_cur.next = even_dummy_head.next;
    odd_dummy_head.next
}

fn main() {
    println!("Hello, world!");
}
