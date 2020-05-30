struct MinHeap {
    data: Vec<Box<ListNode>>,
}

impl MinHeap {
    // MARK: Utils
    fn parent_idx(idx: usize) -> usize {
        (idx - 1) / 2
    }

    fn left_child_idx(idx: usize) -> usize {
        idx * 2 + 1
    }

    fn right_child_idx(idx: usize) -> usize {
        idx * 2 + 2
    }

    // MARK: Interface

    pub fn new() -> Self {
        MinHeap { data: vec![] }
    }

    fn insert(&mut self, node: Box<ListNode>) {
        self.data.push(node);
        let mut cur = self.data.len() - 1;
        loop {
            if cur == 0 {
                break;
            }
            let parent_idx = MinHeap::parent_idx(cur);
            if &self.data[cur].val < &self.data[parent_idx].val {
                self.data.swap(cur, parent_idx);
                cur = parent_idx;
            } else {
                break;
            }
        }
    }

    fn poll(&mut self) -> Option<Box<ListNode>> {
        let len = self.data.len();
        if len == 0 {
            return None;
        }
        // swap last with first
        self.data.swap(0, len - 1);
        // retrieve last
        let rv = self.data.pop();
        if self.data.len() == 0 {
            return rv;
        }
        // swap first downside
        let mut cur = 0;
        loop {
            let l_idx = MinHeap::left_child_idx(cur);
            let r_idx = MinHeap::right_child_idx(cur);
            let val = self.data[cur].val;
            match (self.data.get(l_idx), self.data.get(r_idx)) {
                (Some(l), Some(r)) => {
                    if val > std::cmp::min(l.val, r.val) {
                        if l.val < r.val {
                            self.data.swap(cur, l_idx);
                            cur = l_idx;
                        } else {
                            self.data.swap(cur, r_idx);
                            cur = r_idx;
                        }
                        continue;
                    } else {
                        break;
                    }
                }
                (Some(l), None) => {
                    if l.val < val {
                        self.data.swap(cur, l_idx);
                        cur = l_idx;
                        continue;
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
        rv
    }
}

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

struct Solution {}
impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut min_heap = MinHeap::new();
        for list in lists {
            if let Some(node) = list {
                min_heap.insert(node);
            }
        }
        let mut dummy_head = ListNode::new(0);
        let mut cur = &mut dummy_head;
        while let Some(node) = min_heap.poll() {
            cur.next = Some(Box::new(ListNode::new(node.val)));
            cur = cur.next.as_mut().unwrap();
            if let Some(next) = node.next {
                min_heap.insert(next);
            }
        }
        return dummy_head.next;
    }
}

fn main() {
    let d = Solution::merge_k_lists(vec![
        Some(Box::new(ListNode::new(1))),
        Some(Box::new(ListNode::new(1))),
        Some(Box::new(ListNode::new(2))),
    ]);
}
