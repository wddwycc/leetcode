// Definition for singly-linked list.
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub struct Solution {}

// MARK: Recursively: Time: O(n), Space: O(n)
// Space O(n) is due to call stack size of recursion

// impl Solution {
//     fn helper(
//         head: Option<Box<ListNode>>,
//         reversed: Option<Box<ListNode>>,
//     ) -> Option<Box<ListNode>> {
//         if let Some(mut node) = head {
//             let new_head = std::mem::replace(&mut node.next, reversed);
//             Self::helper(new_head, Some(node))
//         } else {
//             reversed
//         }
//     }

//     pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//         Self::helper(head, None)
//     }
// }

// MARK: Iteratively: Time: O(n), Space: O(1)

impl Solution {
    /// Given res = None, head = a -> b -> c ...
    ///
    /// For every iteration
    /// res = head.val -> res
    /// head = head.next
    ///
    /// Iteration ends when head is None (reaches end of the source linked list)
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut res = None;
        while let Some(mut node) = head {
            head = std::mem::replace(&mut node.next, res);
            res = Some(node);
        }
        res
    }
}

fn main() {
    println!("Hello, world!");
}
