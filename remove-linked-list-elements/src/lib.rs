// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub struct Solution;
impl Solution {
    pub fn remove_elements(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut dummy_head = ListNode { val: 0, next: None };
        let mut cur = &mut dummy_head;
        while let Some(mut node) = head {
            head = std::mem::replace(&mut node.next, None);
            if node.val != val {
                cur.next = Some(node);
                cur = cur.next.as_mut().unwrap();
            }
        }
        dummy_head.next
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
