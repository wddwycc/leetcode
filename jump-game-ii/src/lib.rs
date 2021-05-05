pub struct Solution;
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        // dp[i] => min jumps to reach i from 0
        let mut dp = vec![std::usize::MAX; n];
        dp[0] = 0;
        for i in 0..n {
            let max_jump = nums[i] as usize;
            for j in (i + 1)..=(i + max_jump) {
                if j < n {
                    dp[j] = std::cmp::min(dp[i] + 1, dp[j]);
                }
            }
        }
        dp[n - 1] as i32
    }
}
