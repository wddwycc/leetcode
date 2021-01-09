// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub struct Solution;
impl Solution {
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut head = match head {
            Some(a) => a,
            None => return None,
        };
        // calc list len
        let len = {
            let mut res = 1;
            let mut cur = &head;
            while let Some(ref next) = cur.next {
                res += 1;
                cur = next;
            }
            res
        };
        // determine pos to rotate, reach the pos
        let mut target = len - k % len;
        if target == len {
            return Some(head);
        }
        let mut cur = &mut head;
        while target > 1 {
            cur = cur.next.as_mut().unwrap();
            target -= 1
        }
        // swap
        let mut res = std::mem::replace(&mut cur.next, None).unwrap();
        let mut cur = &mut res;
        while let Some(ref mut next) = cur.next {
            cur = next;
        }
        cur.next = Some(head);
        Some(res)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
