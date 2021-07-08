use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Solution;
impl Solution {
    // time: O(klogn), space: O(n)
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = matrix.len();
        let mut heap = BinaryHeap::new();
        for i in 0..n {
            heap.push((Reverse(matrix[i][0]), (i, 0)));
        }
        let mut ans = 0;
        for _ in 0..k {
            let (Reverse(v), (i, j)) = heap.pop().unwrap();
            if j + 1 < n {
                heap.push((Reverse(matrix[i][j + 1]), (i, j + 1)));
            }
            ans = v;
        }
        ans
    }
}
