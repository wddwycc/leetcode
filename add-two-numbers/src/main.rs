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

// You are given two non-empty linked lists representing two non-negative integers.
// The digits are stored in reverse order and each of their nodes contain a single digit.
// Add the two numbers and return it as a linked list.

// You may assume the two numbers do not contain any leading zero, except the number 0 itself.

// Example:

// Input: (2 -> 4 -> 3) + (5 -> 6 -> 4)
// Output: 7 -> 0 -> 8
// Explanation: 342 + 465 = 807.

type List = Option<Box<ListNode>>;

pub fn add_two_numbers(l1: List, l2: List) -> List {
    let mut c1 = l1;
    let mut c2 = l2;
    let mut carry = 0;

    let mut result: List = None;
    let mut cur = &mut result;

    while c1.is_some() || c2.is_some() || carry > 0 {
        let mut sum: i32 = 0;
        if let Some(v) = c1 {
            sum += v.val;
            c1 = v.next;
        }
        if let Some(v) = c2 {
            sum += v.val;
            c2 = v.next;
        }
        sum += carry;
        carry = sum / 10;
        *cur = Some(Box::new(ListNode::new(sum % 10)));
        cur = &mut cur.as_mut().unwrap().next;
    }

    result
}

fn vec_to_singly_linked_list(src: Vec<i32>) -> Option<Box<ListNode>> {
    let mut tree: Option<Box<ListNode>> = None;
    let mut src = src;
    src.reverse();
    for el in src {
        match tree.take() {
            None => tree = Some(Box::new(ListNode::new(el))),
            Some(current) => {
                tree = Some(Box::new(ListNode {
                    val: el,
                    next: Some(current),
                }))
            }
        }
    }
    tree
}

fn main() {
    let a = add_two_numbers(
        vec_to_singly_linked_list(vec![9]),
        vec_to_singly_linked_list(vec![1, 9, 9, 9, 9, 9, 9, 9, 9, 9]),
    );
    let b = vec_to_singly_linked_list(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
    assert_eq!(a, b);
    // println!("Hello, world!");
}
