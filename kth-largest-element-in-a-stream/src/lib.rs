use std::{cmp::Reverse, collections::BinaryHeap};

#[derive(Default)]
pub struct KthLargest {
    k: usize,
    heap: BinaryHeap<Reverse<i32>>,
}

impl KthLargest {
    pub fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut kl = Self::default();
        kl.k = k as usize;
        for num in nums {
            kl.add(num);
        }
        kl
    }

    pub fn add(&mut self, val: i32) -> i32 {
        if self.heap.len() < self.k {
            self.heap.push(Reverse(val));
        } else {
            if self.heap.peek().unwrap().0 < val {
                self.heap.pop();
                self.heap.push(Reverse(val));
            }
        }
        self.heap.peek().unwrap().0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut kl = KthLargest::new(3, vec![4, 5, 8, 2]);
        assert_eq!(kl.add(3), 4);
        assert_eq!(kl.add(5), 5);
        assert_eq!(kl.add(10), 5);
        assert_eq!(kl.add(9), 8);
        assert_eq!(kl.add(4), 8);
    }
}
