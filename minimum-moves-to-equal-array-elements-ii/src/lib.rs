pub struct Solution;
impl Solution {
    pub fn min_moves2(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let mut ans = 0;
        let mut l = 0;
        let mut r = nums.len() - 1;
        while l <= r {
            if r == 0 { break; }
            ans += nums[r] - nums[l];
            l += 1;
            r -= 1;
        }
        ans
    }
}