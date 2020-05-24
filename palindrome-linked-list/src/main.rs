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

pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    // convert into array
    let mut vec = vec![];
    let mut cur = &head;
    while let Some(a) = cur.as_ref() {
        vec.push(a.val);
        cur = &a.next;
    }
    // two cursor solution
    let len = vec.len();
    if len == 0 {
        return true;
    }
    let mut i = 0;
    let mut j = len - 1;
    while i < j {
        if vec[i] != vec[j] {
            return false;
        }
        i += 1;
        j -= 1;
    }

    true
}

fn main() {
    println!("Hello, world!");
}
