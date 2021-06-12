pub struct Solution;
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return nums[0];
        }
        // dp[i] => when robber start at i, max profit he can achieve
        // dp[i] = max {
        //     nums[i] + max { dp[i + 2] + ... dp[n - 1] },
        //     max { dp[i + 1]... dp[n - 1] }
        // }
        let mut rob_cur_max = nums[n - 2];
        let mut skip_cur_max = nums[n - 1];
        for i in (0..(n - 2)).rev() {
            let new_rob_cur_max = nums[i] + skip_cur_max;
            let new_skip_cur_max = std::cmp::max(rob_cur_max, skip_cur_max);
            rob_cur_max = new_rob_cur_max;
            skip_cur_max = new_skip_cur_max;
        }
        std::cmp::max(rob_cur_max, skip_cur_max)
    }
}
