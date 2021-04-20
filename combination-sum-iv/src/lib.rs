use std::collections::HashSet;

pub struct Solution;
impl Solution {
    pub fn combination_sum4(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort();
        let mut dp = vec![1];
        for t in 1..=target {
            let mut sum = 0;
            for &num in nums.iter() {
                if num <= t {
                    sum += dp[(t - num) as usize];
                } else {
                    break;
                }
            }
            dp.push(sum);
        }
        dp[target as usize]
    }
}
