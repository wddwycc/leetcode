use std::collections::HashSet;

pub struct Solution;
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let nums = nums.into_iter().collect::<HashSet<_>>();
        let mut ans = 0;
        for &num in &nums {
            if nums.contains(&(num - 1)) {
                continue;
            }
            let mut offset = 1;
            while nums.contains(&(num + offset)) {
                offset += 1;
            }
            ans = ans.max(offset);
        }
        ans
    }
}
