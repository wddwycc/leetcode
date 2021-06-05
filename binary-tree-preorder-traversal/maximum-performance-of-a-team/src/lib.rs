use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution;
impl Solution {
    pub fn max_performance(n: i32, speed: Vec<i32>, efficiency: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut workers = efficiency
            .into_iter()
            .zip(speed.into_iter())
            .map(|(e, s)| (Reverse(e), s))
            .collect::<Vec<_>>();
        workers.sort();
        let mut heap = BinaryHeap::new();
        let mut ans = 0_i64;
        let mut sum = 0_i64;
        for (Reverse(e), s) in workers {
            heap.push(Reverse(s));
            sum += s as i64;
            if heap.len() > k {
                sum -= heap.pop().unwrap().0 as i64;
            }
            ans = std::cmp::max(ans, sum * e as i64);
        }
        (ans % 1_000_000_007) as i32
    }
}
