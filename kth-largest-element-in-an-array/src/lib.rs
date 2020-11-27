use std::collections::BinaryHeap;

pub struct Solution;
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap = BinaryHeap::new();
        for num in nums {
            heap.push(num);
        }
        let mut k = k;
        while k > 1 {
            heap.pop();
            k -= 1;
        }
        *heap.peek().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2);
    }
}
