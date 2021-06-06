use std::collections::HashSet;

pub struct Solution;
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let hashset = nums.iter().map(|&a| a).collect::<HashSet<_>>();
        let mut ans = 0;
        for num in nums {
            if hashset.contains(&(num - 1)) {
                continue;
            }
            let mut offset = 1;
            while hashset.contains(&(num + offset)) {
                offset += 1;
            }
            ans = ans.max(offset);
        }
        ans
    }
}
