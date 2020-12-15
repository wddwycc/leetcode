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

pub struct Solution;
impl Solution {
    fn cons_linked_list(src: &[i32]) -> Option<Box<ListNode>> {
        if src.len() == 0 {
            return None;
        }
        Some(Box::new(ListNode {
            val: src[0],
            next: Self::cons_linked_list(&src[1..]),
        }))
    }

    pub fn plus_one(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut nums = {
            let mut res = vec![];
            while let Some(mut node) = head {
                let next = std::mem::replace(&mut node.next, None);
                res.push(node.val);
                head = next;
            }
            res
        };
        {
            let mut need_plus_one = true;
            let mut cur = nums.len() - 1;
            loop {
                if !need_plus_one {
                    break;
                }
                if nums[cur] == 9 {
                    nums[cur] = 0;
                } else {
                    nums[cur] = nums[cur] + 1;
                    need_plus_one = false;
                }
                if cur == 0 {
                    break;
                }
                cur -= 1;
            }
            if need_plus_one {
                nums.insert(0, 1);
            }
        }
        Self::cons_linked_list(&nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
