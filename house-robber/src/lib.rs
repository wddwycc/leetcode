pub struct Solution;
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        // dp[i] => when robber reaches i, max profit he can achieve
        // dp[i] = max {
        //     nums[i] + dp[i - 2],
        //     dp[i - 1]
        // }
        let mut pre2 = 0;
        let mut pre1 = 0;
        for num in nums {
            let next_pre1 = std::cmp::max(num + pre2, pre1);
            pre2 = pre1;
            pre1 = next_pre1;
        }
        pre1
    }
}
