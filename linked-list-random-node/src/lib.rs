use rand::prelude::*;

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
struct Solution {
    len: usize,
    list: Option<Box<ListNode>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    /** @param head The linked list's head.
    Note that the head is guaranteed to be not null, so it contains at least one node. */
    fn new(head: Option<Box<ListNode>>) -> Self {
        let mut cur = head.as_ref();
        let mut len = 0;
        while let Some(node) = cur {
            cur = node.next.as_ref();
            len += 1;
        }
        Self { len, list: head }
    }

    /** Returns a random node's value. */
    fn get_random(&self) -> i32 {
        if self.len == 0 {
            return -1;
        }
        if self.len == 1 {
            return self.list.as_ref().unwrap().val;
        }
        let mut rng = rand::thread_rng();
        let target = rng.gen_range(0, self.len);
        let mut cur = 0;
        let mut node = self.list.as_ref().unwrap();
        while cur < target {
            node = node.next.as_ref().unwrap();
            cur += 1;
        }
        node.val
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(head);
 * let ret_1: i32 = obj.get_random();
 */

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
